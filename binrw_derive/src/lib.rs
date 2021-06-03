#![warn(clippy::pedantic)]
#![warn(rust_2018_idioms)]
#![allow(clippy::expl_impl_clone_on_copy)]

mod codegen;
mod named_args;
mod parser;

use codegen::{
    generate_impl,
    typed_builder::{Builder, BuilderField, BuilderFieldKind},
};
use named_args::NamedArgAttr;
use parser::{is_binread_attr, Input, ParseResult};
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, spanned::Spanned, DeriveInput};

#[proc_macro_derive(BinRead, attributes(binread, br))]
#[cfg(not(tarpaulin_include))]
pub fn derive_binread_trait(input: TokenStream) -> TokenStream {
    derive_from_input(&parse_macro_input!(input as DeriveInput))
        .1
        .into()
}

#[proc_macro_attribute]
#[cfg(not(tarpaulin_include))]
pub fn binread(_: TokenStream, input: TokenStream) -> TokenStream {
    derive_from_attribute(parse_macro_input!(input as DeriveInput)).into()
}

fn clean_field_attrs(
    binread_input: &Option<Input>,
    variant_index: usize,
    fields: &mut syn::Fields,
) {
    if let Some(binread_input) = binread_input {
        let fields = match fields {
            syn::Fields::Named(fields) => &mut fields.named,
            syn::Fields::Unnamed(fields) => &mut fields.unnamed,
            syn::Fields::Unit => return,
        };

        *fields = fields
            .iter_mut()
            .enumerate()
            .filter_map(|(index, value)| {
                if binread_input.is_temp_field(variant_index, index) {
                    None
                } else {
                    let mut value = value.clone();
                    clean_struct_attrs(&mut value.attrs);
                    Some(value)
                }
            })
            .collect();
    }
}

#[proc_macro_derive(BinrwNamedArgs, attributes(named_args))]
pub fn derive_binrw_named_args(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let fields = match match input.data {
        syn::Data::Struct(s) => s
            .fields
            .iter()
            .map(|field| {
                let attrs: Vec<NamedArgAttr> = field
                    .attrs
                    .iter()
                    .filter_map(|attr| {
                        let is_named_args = attr
                            .path
                            .get_ident()
                            .map_or(false, |ident| ident == "named_args");
                        if is_named_args {
                            attr.parse_args().ok()
                        } else {
                            None
                        }
                    })
                    .collect();
                let kind = if attrs
                    .iter()
                    .any(|attr| matches!(attr, NamedArgAttr::TryOptional))
                {
                    BuilderFieldKind::TryOptional
                } else {
                    BuilderFieldKind::Required
                };
                Ok(BuilderField {
                    kind,
                    name: match field.ident.as_ref() {
                        Some(ident) => ident.clone(),
                        None => {
                            return Err(syn::Error::new(
                                field.span(),
                                "must not be a tuple-style field",
                            ))
                        }
                    },
                    ty: field.ty.clone(),
                })
            })
            .collect::<Result<Vec<_>, syn::Error>>(),
        _ => {
            return syn::Error::new(input.span(), "only structs are supported")
                .to_compile_error()
                .into()
        }
    } {
        Ok(fields) => fields,
        Err(err) => return err.into_compile_error().into(),
    };

    let generics: Vec<_> = input.generics.params.iter().cloned().collect();

    Builder {
        result_name: &input.ident,
        builder_name: &quote::format_ident!("{}Builder", input.ident),
        fields: &fields,
        generics: &generics,
        vis: &input.vis,
    }
    .generate(false)
    .into()
}

fn clean_struct_attrs(attrs: &mut Vec<syn::Attribute>) {
    attrs.retain(|attr| !is_binread_attr(attr));
}

fn derive_from_attribute(mut derive_input: DeriveInput) -> proc_macro2::TokenStream {
    let (binread_input, generated_impl) = derive_from_input(&derive_input);
    let binread_input = binread_input.ok();

    clean_struct_attrs(&mut derive_input.attrs);

    match &mut derive_input.data {
        syn::Data::Struct(input_struct) => {
            clean_field_attrs(&binread_input, 0, &mut input_struct.fields);
        }
        syn::Data::Enum(input_enum) => {
            for (index, variant) in input_enum.variants.iter_mut().enumerate() {
                clean_struct_attrs(&mut variant.attrs);
                clean_field_attrs(&binread_input, index, &mut variant.fields);
            }
        }
        syn::Data::Union(union) => {
            for field in union.fields.named.iter_mut() {
                clean_struct_attrs(&mut field.attrs);
            }
        }
    }

    quote!(
        #derive_input
        #generated_impl
    )
}

fn derive_from_input(derive_input: &DeriveInput) -> (ParseResult<Input>, proc_macro2::TokenStream) {
    let binread_input = Input::from_input(&derive_input);
    let generated_impl = generate_impl(&derive_input, &binread_input);
    (binread_input, generated_impl)
}

#[cfg(test)]
#[cfg(tarpaulin)]
#[test]
fn derive_code_coverage_for_tarpaulin() {
    use runtime_macros_derive::emulate_derive_expansion_fallible;
    use std::{env, fs};

    let derive_tests_folder = env::current_dir()
        .unwrap()
        .join("..")
        .join("binrw")
        .join("tests")
        .join("derive");

    let mut run_success = true;
    for entry in fs::read_dir(derive_tests_folder).unwrap() {
        let entry = entry.unwrap();
        if entry.file_type().unwrap().is_file() {
            let file = fs::File::open(entry.path()).unwrap();
            if emulate_derive_expansion_fallible(file, "BinRead", |input| {
                derive_from_input(&input).1
            })
            .is_err()
            {
                run_success = false;
            }
        }
    }

    assert!(run_success)
}
