//! This crate contains the implementation of the `RustEmbed` macro for
//! `rust-embed-for-web`.
//!
//! You generally don't want to use this crate directly, `rust-embed-for-web`
//! re-exports any necessary parts from this crate.
#![recursion_limit = "1024"]
#![forbid(unsafe_code)]
#[macro_use]
extern crate quote;
extern crate proc_macro;

mod attributes;
mod compress;
mod dynamic;
mod embed;

use attributes::read_attribute_config;
use dynamic::generate_dynamic_impl;
use embed::generate_embed_impl;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use std::{env, path::Path};
use syn::{Data, DeriveInput, Expr, ExprLit, Fields, Lit, Meta, MetaNameValue};

/// Find all pairs of the `name = "value"` attribute from the derive input
fn find_attribute_values(ast: &syn::DeriveInput, attr_name: &str) -> Vec<String> {
    ast.attrs
        .iter()
        .filter(|value| value.path().is_ident(attr_name))
        .map(|attr| &attr.meta)
        .filter_map(|meta| match meta {
            Meta::NameValue(MetaNameValue {
                value:
                    Expr::Lit(ExprLit {
                        lit: Lit::Str(val), ..
                    }),
                ..
            }) => Some(val.value()),
            _ => None,
        })
        .collect()
}

fn impl_rust_embed_for_web(ast: &syn::DeriveInput) -> syn::Result<TokenStream2> {
    match ast.data {
        Data::Struct(ref data) => match data.fields {
            Fields::Unit => {}
            _ => {
                return Err(syn::Error::new_spanned(
                    ast,
                    "RustEmbed can only be derived for unit structs",
                ))
            }
        },
        _ => {
            return Err(syn::Error::new_spanned(
                ast,
                "RustEmbed can only be derived for unit structs",
            ))
        }
    };

    let mut folder_paths = find_attribute_values(ast, "folder");
    if folder_paths.len() != 1 {
        return Err(syn::Error::new_spanned(
            ast,
            "#[derive(RustEmbed)] must contain one and only one folder attribute",
        ));
    }
    let folder_path = folder_paths.remove(0);
    #[cfg(feature = "interpolate-folder-path")]
    let folder_path = shellexpand::full(&folder_path)
        .map_err(|e| {
            syn::Error::new_spanned(ast, format!("Could not interpolate folder path: {e}"))
        })?
        .to_string();

    // Base relative paths on the Cargo.toml location
    let folder_path = if Path::new(&folder_path).is_relative() {
        let manifest_dir = env::var("CARGO_MANIFEST_DIR").map_err(|e| {
            syn::Error::new_spanned(
                ast,
                format!("Could not read the CARGO_MANIFEST_DIR environment variable: {e}"),
            )
        })?;
        Path::new(&manifest_dir)
            .join(folder_path)
            .to_str()
            .ok_or_else(|| {
                syn::Error::new_spanned(
                    ast,
                    "The folder path does not have a valid string representation",
                )
            })?
            .to_owned()
    } else {
        folder_path
    };

    let config = read_attribute_config(ast);

    // If the folder does not exist, either fail the build or, when the
    // `allow_missing` attribute is set, generate an empty asset set.
    if !Path::new(&folder_path).exists() && !config.allow_missing() {
        panic!(
            "#[derive(RustEmbed)] folder '{}' does not exist. \
             Set `#[allow_missing = true]` to allow a missing folder and \
             generate an empty asset set instead.",
            folder_path
        );
    }

    let prefixes = find_attribute_values(ast, "prefix");
    let prefix = if prefixes.is_empty() {
        "".to_string()
    } else if prefixes.len() == 1 {
        prefixes[0].clone()
    } else {
        return Err(syn::Error::new_spanned(
            ast,
            "#[derive(RustEmbed)] must have at most one prefix, you supplied several",
        ));
    };

    if cfg!(debug_assertions) && !cfg!(feature = "always-embed") {
        Ok(generate_dynamic_impl(
            &ast.ident,
            &config,
            &folder_path,
            &prefix,
        ))
    } else {
        Ok(generate_embed_impl(
            &ast.ident,
            &config,
            &folder_path,
            &prefix,
        ))
    }
}

#[proc_macro_derive(
    RustEmbed,
    attributes(folder, prefix, include, exclude, gzip, br, zstd, allow_missing)
)]
/// A folder that is embedded into your program.
///
/// For example:
///
/// ```ignore
/// #[derive(RustEmbed)]
/// #[folder = "examples/public"]
/// struct MyEmbeddedFiles;
/// ```
///
/// The `folder` is relative to where your `Cargo.toml` file is located. This
/// example will embed the files under `<your-workspace>/examples/public` into
/// your program.
///
/// Please check the package readme for more details.
pub fn derive_input_object(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = match syn::parse(input) {
        Ok(ast) => ast,
        Err(e) => return e.to_compile_error().into(),
    };
    match impl_rust_embed_for_web(&ast) {
        Ok(gen) => gen.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

#[cfg(test)]
mod tests {
    use super::impl_rust_embed_for_web;

    fn err_message(input: &str) -> String {
        let ast: syn::DeriveInput = syn::parse_str(input).unwrap();
        impl_rust_embed_for_web(&ast)
            .expect_err("expected the derive input to fail")
            .to_string()
    }

    #[test]
    fn rejects_enums() {
        assert!(err_message("#[folder = \"src\"] enum Bad { A }")
            .contains("can only be derived for unit structs"));
    }

    #[test]
    fn rejects_structs_with_fields() {
        assert!(err_message("#[folder = \"src\"] struct Bad { field: u32 }")
            .contains("can only be derived for unit structs"));
    }

    #[test]
    fn requires_a_folder_attribute() {
        assert!(err_message("struct Bad;").contains("one and only one folder attribute"));
    }

    #[test]
    fn rejects_multiple_folder_attributes() {
        assert!(
            err_message("#[folder = \"src\"] #[folder = \"src\"] struct Bad;")
                .contains("one and only one folder attribute")
        );
    }

    #[test]
    fn rejects_multiple_prefix_attributes() {
        assert!(err_message(
            "#[folder = \"src\"] #[prefix = \"a/\"] #[prefix = \"b/\"] struct Bad;"
        )
        .contains("at most one prefix"));
    }

    #[test]
    fn accepts_a_valid_unit_struct() {
        // `src` exists relative to this crate's Cargo.toml, so the derive
        // resolves the folder and emits an implementation.
        let ast: syn::DeriveInput = syn::parse_str("#[folder = \"src\"] struct Good;").unwrap();
        assert!(impl_rust_embed_for_web(&ast).is_ok());
    }
}
