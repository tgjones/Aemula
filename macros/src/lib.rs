extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
use proc_macro_error::{abort, abort_call_site, ResultExt};
use quote::quote;
use syn::{self, parse_macro_input, spanned::Spanned, DataStruct, DeriveInput, Field, Lit, Meta, MetaNameValue, Visibility};

#[proc_macro_derive(PinAccessors, attributes(pin, handle))]
pub fn derive_pin_accessors(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    //println!("item: \"{}\"", item.to_string());
    //let ast = syn::parse(item).unwrap();
    //item
    produce(&input).into()
}

fn produce(ast: &DeriveInput) -> TokenStream2 {
    let name = &ast.ident;

    if let syn::Data::Struct(DataStruct { ref fields, .. }) = ast.data {
        let generated = fields.iter().map(|f| generate_field_accessors(f));

        quote! {
            impl #name {
                #(#generated)*
            }
        }
    } else {
        abort_call_site!("#[derive(PinAccessors)] can only be used on structs")
    }
}

fn generate_field_accessors(field: &Field) -> TokenStream2 {
    let field_name = field
        .clone()
        .ident
        .unwrap_or_else(|| abort!(field.span(), "Expected the field to have a name"));

    let pin_attr = field
        .attrs
        .iter()
        .filter_map(|v| parse_pin_attr(v))
        .last();

    let ty = field.ty.clone();

    match pin_attr {
        Some(arg) => {
            let mut functions = Vec::new();

            if arg.path().is_ident("in") || arg.path().is_ident("bidirectional") {
                let handle_attrs = field
                    .attrs
                    .iter()
                    .filter_map(|v| parse_handle_attr(v))
                    .last();

                let fn_name = Ident::new(&format!("set_{}", field_name), Span::call_site());

                //println!("{:#?}", handle_attrs);

                let mut transitions: Vec<TokenStream2> = Vec::new();
                let mut others: Vec<TokenStream2> = Vec::new();
                let mut needs_old_value = false;

                match handle_attrs {
                    Some(metas) => {
                        for meta in metas {
                            let path = meta.path();
                            if path.is_ident("transition_lo_to_hi") {
                                let transition_name = Ident::new(&format!("on_{}_transition_lo_to_hi", field_name), Span::call_site());
                                transitions.push(quote! {
                                    (false, true) => self.#transition_name(),
                                });
                                needs_old_value = true;
                            } else if path.is_ident("transition_hi_to_lo") {
                                let transition_name = Ident::new(&format!("on_{}_transition_hi_to_lo", field_name), Span::call_site());
                                transitions.push(quote! {
                                    (true, false) => self.#transition_name(),
                                });
                                needs_old_value = true;
                            } else if path.is_ident("always") {
                                let callback_name = Ident::new(&format!("on_{}_set", field_name), Span::call_site());
                                others.push(quote! {
                                    self.#callback_name();
                                });
                            } else if path.is_ident("change") {
                                let callback_name = Ident::new(&format!("on_{}_change", field_name), Span::call_site());
                                others.push(quote! {
                                    if old_value != value {
                                        self.#callback_name();
                                    }
                                });
                                needs_old_value = true;
                            }
                        }
                    }

                    None => {}
                }

                let transitions_match =
                    if !transitions.is_empty() {
                        quote! {
                            match (old_value, value) {
                                #(#transitions)*
                                _ => {}
                            }
                        }
                    } else {
                        quote! {}
                    };

                let old_value_fragment =
                    if needs_old_value {
                        quote! {
                            let old_value = self.#field_name;
                        }
                    } else {
                        quote! {}
                    };

                functions.push(quote! {
                    pub fn #fn_name(&mut self, value: #ty) {
                        #old_value_fragment

                        self.#field_name = value;

                        #transitions_match

                        #(#others)*
                    }
                });
            }
            
            if arg.path().is_ident("out") || arg.path().is_ident("bidirectional") {
                let fn_name = Ident::new(&format!("{}", field_name), Span::call_site());
                functions.push(quote! {
                    pub fn #fn_name(&self) -> #ty {
                        self.#field_name
                    }
                });
            }

            quote! {
                #(#functions)*
            }
        }

        None => quote! {}
    }
}

fn parse_pin_attr(attr: &syn::Attribute) -> Option<Meta> {
    use syn::{punctuated::Punctuated, Token};

    if attr.path.is_ident("pin") {
        attr.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)
            .unwrap_or_abort()
            .into_iter()
            .inspect(|meta| {
                if !(meta.path().is_ident("in")
                    || meta.path().is_ident("out")
                    || meta.path().is_ident("bidirectional"))
                {
                    abort!(meta.path().span(), "unknown pin direction")
                }
            })
            .last()
    } else {
        None
    }
}

fn parse_handle_attr(attr: &syn::Attribute) -> Option<Vec<Meta>> {
    use syn::{punctuated::Punctuated, Token};

    if attr.path.is_ident("handle") {
        let result: Vec<Meta> = attr.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)
            .unwrap_or_abort()
            .into_iter()
            .collect();
        Some(result)
    } else {
        None
    }
}