#[cfg(feature = "include-exclude")]
use globset::GlobMatcher;
use proc_macro2::TokenStream as TokenStream2;
use rust_embed_for_web_utils::Config;

use crate::embed::IntoEmbed;

impl IntoEmbed for Vec<String> {
    fn into_embed(&self) -> TokenStream2 {
        let v = self;
        quote! { &[#(#v),*] }
    }
}

#[cfg(feature = "include-exclude")]
impl IntoEmbed for Vec<GlobMatcher> {
    fn into_embed(&self) -> TokenStream2 {
        let patterns: Vec<String> = self.iter().map(|v| v.glob().to_string()).collect();
        patterns.into_embed()
    }
}

impl IntoEmbed for Config {
    fn into_embed(&self) -> TokenStream2 {
        let includes_embed = {
            let includes = self.get_includes();
            if includes.len() == 0 {
                quote! {}
            } else {
                let includes = includes.into_embed();
                quote! {
                    for ele in #includes {
                        config.add_include(ele.to_string());
                    }
                }
            }
        };
        let excludes_embed = {
            let excludes = self.get_excludes();
            if excludes.len() == 0 {
                quote! {}
            } else {
                let excludes = excludes.into_embed();
                quote! {
                    for ele in #excludes {
                        config.add_exclude(ele.to_string());
                    }
                }
            }
        };

        quote! {
            let mut config = rust_embed_for_web_utils::Config::new();
            #includes_embed
            #excludes_embed
            config
        }
    }
}

pub(crate) fn generate_dynamic_impl(
    ident: &syn::Ident,
    config: &Config,
    folder_path: &str,
) -> TokenStream2 {
    let config = config.into_embed();

    quote! {
      impl #ident {
        fn get(path: &str) -> Option<rust_embed_for_web::DynamicFile> {
          let config = { #config };
          if config.should_include(path) {
            let folder_path: std::path::PathBuf = std::convert::From::from(#folder_path);
            let combined_path = folder_path.join(path);
            rust_embed_for_web::DynamicFile::read_from_fs(combined_path).ok()
          } else {
            None
          }
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
