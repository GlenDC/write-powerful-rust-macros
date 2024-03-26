use proc_macro::TokenStream;
use quote::quote;
use syn::Data::{Struct, Enum};
use syn::Fields::{Named, Unnamed};
use syn::__private::{Span, TokenStream2};
use syn::{parse_macro_input, DataEnum, DataStruct, DeriveInput, FieldsNamed, FieldsUnnamed, Ident};

fn generated_methods(ast: &DeriveInput) -> Vec<TokenStream2> {
    let named_fields = match ast.data {
        Struct(DataStruct {
            fields: Named(FieldsNamed { ref named, .. }),
            ..
        }) => named,
        _ => unimplemented!("only works for structs with named fields"),
    };

    named_fields
        .iter()
        .map(|f| {
            let field_name = f.ident.as_ref().take().unwrap();
            let type_name = &f.ty;
            let method_name = Ident::new(&format!("get_{field_name}"), Span::call_site());

            quote!(
                fn #method_name(&self) -> &#type_name {
                    &self.#field_name
                }
            )
        })
        .collect()
}


#[proc_macro]
pub fn private(item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);

    let methods = generated_methods(&ast);

    let name = ast.ident;
    let attrs = ast.attrs;

    let private_version = match ast.data {
        Struct(DataStruct {
            fields: Named(FieldsNamed { ref named, .. }),
            ..
        }) => {
            let builder_fields = named.iter().map(|f| {
                let name = &f.ident;
                let ty = &f.ty;
                quote! { #name: #ty }
            });

            quote! {
                #(#attrs)*
                pub struct #name {
                    #(#builder_fields,)*
                }
            }
        }
        Struct(DataStruct {
            fields: Unnamed(FieldsUnnamed { ref unnamed, .. }),
            ..
        }) => {
            let builder_fields = unnamed.iter().map(|f| {
                let ty = &f.ty;
                quote! { #ty }
            });

            quote! {
                #(#attrs)*
                struct #name(#(#builder_fields,)*);
            }
        },
        Enum(DataEnum { ref variants, .. }) => {
            quote! {
                #(#attrs)*
                enum #name {
                    #variants
                }
            }
        }
        _ => unimplemented!("only works for structs with named fields"),
    };

    quote!(
        #private_version

        impl #name {
            #(#methods)*
        }
    ).into()
}   