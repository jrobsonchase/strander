use proc_macro::TokenStream;
use syn::{Data, Expr, Lit, Meta, MetaNameValue};

use quote::{ToTokens, format_ident, quote};

// TODO: field attrs to control default distributions
// TODO: struct attrs to control generated trait/struct names
// TODO: visibility
// TODO: input generics
// TODO: enums?
// TODO: tuple structs?

#[proc_macro_derive(Strand, attributes(strand))]
pub fn derive_strand(item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::DeriveInput);

    let struct_identifier = &input.ident;
    let distr_trait = format_ident!("{}Distribution", struct_identifier);
    let distr_struct = format_ident!("{}Distr", struct_identifier);

    match &input.data {
        Data::Struct(syn::DataStruct { fields, .. }) => {
            let mut distr_generics = quote! {};
            let mut distr_where_clause = quote! {
                where
            };
            let mut distr_trait_method_defs = quote! {};
            let mut distr_trait_method_impls = quote! {};
            let mut distr_struct_fields = quote! {};
            let mut distr_field_constructors = quote! {};
            let mut distr_field_samplers = quote! {};

            for (i, field) in fields.iter().enumerate() {
                let field_name = field.ident.as_ref().unwrap();
                let field_type = &field.ty;
                let field_param = format_ident!("T_{}", i);
                let method_name = format_ident!("with_{}", field_name);
                let field_trait = quote! { ::strander::rand::distr::Distribution<#field_type> };
                let method_signature = quote!{ fn #method_name(self, #field_name: impl #field_trait) -> impl #distr_trait };

                let mut other_fields = quote!{};

                for other in fields.iter().filter(|f| f.ident != field.ident) {
                    let other_name = other.ident.as_ref().unwrap();
                    other_fields.extend(quote! { #other_name : self.#other_name, });
                }

                distr_generics.extend(quote! { #field_param , });
                distr_struct_fields.extend(quote! { #field_name: #field_param, });
                distr_where_clause.extend(quote! { #field_param : #field_trait , });

                distr_trait_method_defs.extend(quote! { #method_signature ; });
                distr_trait_method_impls.extend(quote! { #method_signature {
                    #distr_struct {
                        #field_name,
                        #other_fields
                    }
                }});
                let mut constructor = quote! { <#field_type as ::strander::Strand>::strand() };
                for attr in field.attrs.iter().map(|a| &a.meta) {
                    match attr {
                        Meta::NameValue(MetaNameValue{ value, .. }) => {
                            if let Expr::Lit(expr) = &value {
                                if let Lit::Str(lit_str) = &expr.lit {
                                    constructor = lit_str.parse::<Expr>().expect("a valid rust expression").into_token_stream();
                                }
                            }
                        },
                        other => panic!("unsupported attribute: {:?}", other),
                    }
                }
                distr_field_constructors.extend(quote! { #field_name: #constructor, });
                distr_field_samplers.extend(quote! { #field_name: <#field_param as #field_trait>::sample(&self.#field_name, rng), })

            }

            quote! {
                pub trait #distr_trait: ::strander::rand::distr::Distribution<#struct_identifier> {
                    #distr_trait_method_defs
                }

                pub struct #distr_struct <#distr_generics> {
                    #distr_struct_fields
                }

                mod __strand_impl {
                    #![allow(refining_impl_trait)]
                    use super::*;
                    use ::strander::rand::distr::Distribution;
                    use ::strander::rand::Rng;

                    use super::*;
                    impl<#distr_generics> Distribution<#struct_identifier> for #distr_struct <#distr_generics>
                        #distr_where_clause
                    {
                        fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> #struct_identifier {
                            #struct_identifier {
                                #distr_field_samplers
                            }
                        }
                    }

                    impl<#distr_generics> #distr_trait for #distr_struct <#distr_generics>
                        #distr_where_clause
                    {
                        #distr_trait_method_impls
                    }

                    impl ::strander::Strand for #struct_identifier {
                        fn strand() -> impl #distr_trait {
                            #distr_struct {
                                #distr_field_constructors
                            }
                        }
                    }
                }
            }
        }
        _ => unimplemented!(),
    }
    .into()
}

#[cfg(test)]
mod tests {}
