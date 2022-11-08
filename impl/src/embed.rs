use proc_macro2::TokenStream as TokenStream2;
use rust_embed_for_web_utils::{get_files, Config, DynamicFile, EmbedableFile, FileEntry};

use crate::compress::{compress_br, compress_gzip};

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
) -> TokenStream2 {
    let embeds: Vec<TokenStream2> = get_files(folder_path, config)
        .filter_map(
            |FileEntry {
                 rel_path,
                 full_canonical_path,
             }| {
                let Ok(file) = DynamicFile::read_from_fs(&full_canonical_path) else { return None };
                let file_embed = EmbedDynamicFile::new(&file, config).make_embed();
                Some(quote! {
                    #rel_path => Some(#file_embed),
                })
            },
        )
        .collect();

    quote! {
      impl #ident {
          fn get(path: &str) -> Option<rust_embed_for_web::EmbeddedFile> {
              match path {
                    #(#embeds)*
                    _ => None,
              }
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
