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
    derive_strand_impl(item, true)
}

fn derive_strand_impl(item: TokenStream, impl_strand: bool) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::DeriveInput);

    let struct_name = &input.ident;
    let vis = &input.vis;
    let distr_trait = format_ident!("{}Distribution", struct_name);
    let distr_struct = format_ident!("{}Distr", struct_name);

    match &input.data {
        Data::Struct(syn::DataStruct { fields, .. }) => {
            let mut distr_generics = quote! {};
            let mut distr_generic_defaults = quote! {};
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
                distr_generic_defaults.extend(quote! { #field_param = (), });
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

            let mut strand_impl = quote!{};
            if impl_strand {
                strand_impl = quote! {
                    #[allow(refining_impl_trait)]
                    impl ::strander::Strand for #struct_name {
                        fn strand() -> impl #distr_trait {
                            #distr_struct::new()
                        }
                    }
                }
            };

            quote! {
                #vis trait #distr_trait: ::strander::rand::distr::Distribution<#struct_name> {
                    #distr_trait_method_defs
                }

                #vis struct #distr_struct <#distr_generic_defaults> {
                    #distr_struct_fields
                }

                impl<#distr_generics> ::strander::rand::distr::Distribution<#struct_name> for #distr_struct <#distr_generics>
                    #distr_where_clause
                {
                    fn sample<R: ::strander::rand::Rng + ?Sized>(&self, rng: &mut R) -> #struct_name {
                        use ::strander::rand::distr::Distribution;
                        #struct_name {
                            #distr_field_samplers
                        }
                    }
                }

                impl<#distr_generics> #distr_trait for #distr_struct <#distr_generics>
                    #distr_where_clause
                {
                    #distr_trait_method_impls
                }

                impl #distr_struct {
                    pub fn new() -> impl #distr_trait {
                        #distr_struct {
                            #distr_field_constructors
                        }
                    }
                }

                #strand_impl
            }
        }
        _ => unimplemented!(),
    }
    .into()
}

#[proc_macro_attribute]
pub fn strand_remote(_args: TokenStream, item: TokenStream) -> TokenStream {
    derive_strand_impl(item, false)
}

#[cfg(test)]
mod tests {}
