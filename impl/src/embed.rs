use proc_macro2::TokenStream as TokenStream2;
use rust_embed_for_web_utils::{get_files, Config, DynamicFile, EmbedableFile, FileEntry};

use crate::compress::{compress_br, compress_gzip};

/// Anything that can be embedded into the program.
///
/// We're using our own trait instead of the actual `ToTokens` trait because the
/// types we implement it for are not defined in this crate, so we'd have to
/// wrap all of them.
pub(crate) trait IntoEmbed {
    fn into_embed(&self) -> TokenStream2;
}

impl IntoEmbed for Vec<u8> {
    fn into_embed(&self) -> TokenStream2 {
        // Not sure why quote doesn't like it if I use #self here
        let v = self;
        quote! { &[#(#v),*] }
    }
}

impl IntoEmbed for String {
    fn into_embed(&self) -> TokenStream2 {
        quote! { #self }
    }
}

impl IntoEmbed for i64 {
    fn into_embed(&self) -> TokenStream2 {
        quote! { #self }
    }
}

impl<T: IntoEmbed> IntoEmbed for Option<T> {
    fn into_embed(&self) -> TokenStream2 {
        match self {
            Some(v) => {
                let embed = v.into_embed();
                quote! { Some(#embed) }
            }
            None => quote! { None },
        }
    }
}

impl IntoEmbed for DynamicFile {
    fn into_embed(&self) -> TokenStream2 {
        let file = self;
        let name = file.name().into_embed();
        let data = file.data();
        let data_gzip = compress_gzip(&data).into_embed();
        let data_br = compress_br(&data).into_embed();
        let data = data.into_embed();
        let hash = file.hash().into_embed();
        let etag = file.etag().into_embed();
        let last_modified = file.last_modified().into_embed();
        let last_modified_timestamp = file.last_modified_timestamp().into_embed();
        let mime_type = file.mime_type().into_embed();
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
                let Some(file) = DynamicFile::read_from_fs(&full_canonical_path).ok() else{ return None };
                let file_embed = file.into_embed();
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
