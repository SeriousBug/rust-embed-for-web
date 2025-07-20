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

fn impl_rust_embed_for_web(ast: &syn::DeriveInput) -> TokenStream2 {
    match ast.data {
        Data::Struct(ref data) => match data.fields {
            Fields::Unit => {}
            _ => panic!("RustEmbed can only be derived for unit structs"),
        },
        _ => panic!("RustEmbed can only be derived for unit structs"),
    };

    let mut folder_paths = find_attribute_values(ast, "folder");
    if folder_paths.len() != 1 {
        panic!("#[derive(RustEmbed)] must contain one and only one folder attribute");
    }
    let folder_path = folder_paths.remove(0);
    #[cfg(feature = "interpolate-folder-path")]
    let folder_path = shellexpand::full(&folder_path).unwrap().to_string();

    // Base relative paths on the Cargo.toml location
    let folder_path = if Path::new(&folder_path).is_relative() {
        Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap())
            .join(folder_path)
            .to_str()
            .unwrap()
            .to_owned()
    } else {
        folder_path
    };

    let config = read_attribute_config(ast);

    let prefixes = find_attribute_values(ast, "prefix");
    let prefix = if prefixes.is_empty() {
        "".to_string()
    } else if prefixes.len() == 1 {
        prefixes[0].clone()
    } else {
        panic!("#[derive(RustEmbed)] must have at most one prefix, you supplied several");
    };

    if cfg!(debug_assertions) && !cfg!(feature = "always-embed") {
        generate_dynamic_impl(&ast.ident, &config, &folder_path, &prefix)
    } else {
        generate_embed_impl(&ast.ident, &config, &folder_path, &prefix)
    }
}

#[proc_macro_derive(RustEmbed, attributes(folder, prefix, include, exclude, gzip, br, zstd))]
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
    let ast: DeriveInput = syn::parse(input).unwrap();
    let gen = impl_rust_embed_for_web(&ast);
    gen.into()
}
