use rust_embed_for_web_utils::Config;
use syn::{Attribute, Lit, Meta, MetaNameValue};

fn parse_str(attribute: &Attribute) -> Option<String> {
    if let Ok(Meta::NameValue(MetaNameValue {
        lit: Lit::Str(value),
        ..
    })) = attribute.parse_meta()
    {
        return Some(value.value());
    }
    None
}

fn parse_bool(attribute: &Attribute) -> Option<bool> {
    if let Ok(Meta::NameValue(MetaNameValue {
        lit: Lit::Bool(value),
        ..
    })) = attribute.parse_meta()
    {
        return Some(value.value());
    }
    None
}

pub(crate) fn read_attribute_config(ast: &syn::DeriveInput) -> Config {
    let mut config = Config::default();

    for attribute in &ast.attrs {
        if let Some(ident) = attribute.path.get_ident() {
            let ident = ident.to_string();
            match ident.as_str() {
                "include" => parse_str(attribute).map(|v| config.add_include(v)),
                "exclude" => parse_str(attribute).map(|v| config.add_exclude(v)),
                "gzip" => parse_bool(attribute).map(|v| config.set_gzip(v)),
                "br" => parse_bool(attribute).map(|v| config.set_br(v)),
                _ => None,
            };
        }
    }

    config
}
