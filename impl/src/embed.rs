use proc_macro2::TokenStream as TokenStream2;
use rust_embed_for_web_utils::{get_files, Config, DynamicFile, EmbedableFile, FileEntry};

use crate::compress::{compress_br, compress_gzip, compress_zstd};

/// Anything that can be embedded into the program.
///
/// We're using our own trait instead of the actual `ToTokens` trait because the
/// types we implement it for are not defined in this crate, so we'd have to
/// wrap all of them.
pub(crate) trait MakeEmbed {
    fn make_embed(&self) -> TokenStream2;
}

impl MakeEmbed for Vec<u8> {
    fn make_embed(&self) -> TokenStream2 {
        // Not sure why quote doesn't like it if I use #self here
        let v = self;
        quote! { &[#(#v),*] }
    }
}

impl MakeEmbed for String {
    fn make_embed(&self) -> TokenStream2 {
        quote! { #self }
    }
}

impl MakeEmbed for i64 {
    fn make_embed(&self) -> TokenStream2 {
        quote! { #self }
    }
}

impl<T: MakeEmbed> MakeEmbed for Option<T> {
    fn make_embed(&self) -> TokenStream2 {
        match self {
            Some(v) => {
                let embed = v.make_embed();
                quote! { Some(#embed) }
            }
            None => quote! { None },
        }
    }
}

struct EmbedDynamicFile<'t> {
    file: &'t DynamicFile,
    config: &'t Config,
}

impl<'t> EmbedDynamicFile<'t> {
    fn new(file: &'t DynamicFile, config: &'t Config) -> EmbedDynamicFile<'t> {
        EmbedDynamicFile { file, config }
    }
}

impl<'t> MakeEmbed for EmbedDynamicFile<'t> {
    fn make_embed(&self) -> TokenStream2 {
        let file = self.file;
        let name = file.name().make_embed();
        let data = file.data();
        let data_gzip = if self.config.should_gzip() {
            compress_gzip(&data).make_embed()
        } else {
            None::<Vec<u8>>.make_embed()
        };
        let data_br = if self.config.should_br() {
            compress_br(&data).make_embed()
        } else {
            None::<Vec<u8>>.make_embed()
        };
        let data_zstd = if self.config.should_zstd() {
            compress_zstd(&data).make_embed()
        } else {
            None::<Vec<u8>>.make_embed()
        };
        let data = data.make_embed();
        let hash = file.hash().make_embed();
        let etag = file.etag().make_embed();
        let last_modified = file.last_modified().make_embed();
        let last_modified_timestamp = file.last_modified_timestamp().make_embed();
        let mime_type = file.mime_type().make_embed();
        // Make sure that the order of these parameters is correct!
        quote! {
            rust_embed_for_web::EmbeddedFile::__internal_make(
                #name,
                #data,
                #data_gzip,
                #data_br,
                #data_zstd,
                #hash,
                #etag,
                #last_modified,
                #last_modified_timestamp,
                #mime_type,
            )
        }
    }
}

pub(crate) fn generate_embed_impl(
    ident: &syn::Ident,
    config: &Config,
    folder_path: &str,
    prefix: &str,
) -> TokenStream2 {
    // Collect (path, embed-tokens) pairs and sort them by path so the generated
    // table can be searched at runtime with a binary search.
    let mut entries: Vec<(String, TokenStream2)> = get_files(folder_path, config, prefix)
        .filter_map(
            |FileEntry {
                 rel_path,
                 full_canonical_path,
             }| {
                if let Ok(file) = DynamicFile::read_from_fs(full_canonical_path) {
                    let file_embed = EmbedDynamicFile::new(&file, config).make_embed();
                    Some((rel_path, file_embed))
                } else {
                    None
                }
            },
        )
        .collect();
    entries.sort_by(|(a, _), (b, _)| a.cmp(b));

    let embeds: Vec<TokenStream2> = entries
        .into_iter()
        .map(|(rel_path, file_embed)| {
            quote! {
                (#rel_path, #file_embed),
            }
        })
        .collect();

    quote! {
      impl #ident {
          fn get(path: &str) -> Option<rust_embed_for_web::EmbeddedFile> {
              // Sorted by path at macro-expansion time, so a binary search is valid.
              const ENTRIES: &[(&str, rust_embed_for_web::EmbeddedFile)] = &[
                  #(#embeds)*
              ];
              ENTRIES
                  .binary_search_by_key(&path, |__rust_embed_entry| __rust_embed_entry.0)
                  .ok()
                  .map(|__rust_embed_idx| ENTRIES[__rust_embed_idx].1)
          }
      }

      impl rust_embed_for_web::RustEmbed for #ident {
        type File = rust_embed_for_web::EmbeddedFile;

        fn get(file_path: &str) -> Option<Self::File> {
          #ident::get(file_path)
        }
      }
    }
}
