use proc_macro2::TokenStream as TokenStream2;

pub(crate) fn generate_dynamic_impl(ident: &syn::Ident, folder_path: &str) -> TokenStream2 {
    quote! {
      impl #ident {
        fn get(path: &str) -> Option<rust_embed_for_web::DynamicFile> {
          let folder_path: std::path::PathBuf = std::convert::From::from(#folder_path);
          let combined_path = folder_path.join(path);
          rust_embed_for_web::DynamicFile::read_from_fs(combined_path).ok()
        }
      }

      impl rust_embed_for_web::RustEmbed for #ident {
        type File = rust_embed_for_web::DynamicFile;

        fn get(file_path: &str) -> Option<Self::File> {
          #ident::get(file_path)
        }
      }
    }
}
