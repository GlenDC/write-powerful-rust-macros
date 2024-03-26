extern crate core;

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input,
    Data::{Enum, Struct},
    DataEnum, DataStruct, DeriveInput,
    Fields::{Named, Unnamed},
    FieldsNamed, FieldsUnnamed,
};

#[proc_macro_attribute]
pub fn public(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let ast: DeriveInput = parse_macro_input!(item as DeriveInput);
    let name = ast.ident;
    let attrs = ast.attrs;

    let public_version = match ast.data {
        Struct(DataStruct {
            fields: Named(FieldsNamed { ref named, .. }),
            ..
        }) => {
            let builder_fields = named.iter().map(|f| {
                let name = &f.ident;
                let ty = &f.ty;
                quote! { pub #name: #ty }
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
                quote! { pub #ty }
            });

            quote! {
                #(#attrs)*
                pub struct #name(#(#builder_fields,)*);
            }
        },
        Enum(DataEnum { ref variants, .. }) => {
            quote! {
                #(#attrs)*
                pub enum #name {
                    #variants
                }
            }
        }
        _ => unimplemented!("only works for structs with named fields"),
    };

    public_version.into()
}

// Same as above, but using structs:

/*

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    token::Colon,
    Data::Struct,
    DataStruct,
    DeriveInput, // Field,
    Fields::Named,
    FieldsNamed,
    Ident,
    Visibility,
};

struct StructField {
    name: Ident,
    ty: Ident, // Type,
}

// impl StructField {
//     fn new(field: &Field) -> Self {
//         Self {
//             name: field.ident.as_ref().unwrap().clone(),
//             ty: field.ty.clone(),
//         }
//     }
// }

impl Parse for StructField {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let _vis: Result<Visibility, _> = input.parse();
        let list = Punctuated::<Ident, Colon>::parse_terminated(input).unwrap();

        Ok(StructField {
            name: list.first().unwrap().clone(),
            ty: list.last().unwrap().clone(),
        })
    }
}

impl ToTokens for StructField {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let n = &self.name;
        let t = &self.ty;
        quote!(pub #n: #t).to_tokens(tokens)
    }
}

#[proc_macro_attribute]
pub fn public(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);
    let name = ast.ident;

    let fields = match ast.data {
        Struct(DataStruct {
            fields: Named(FieldsNamed { ref named, .. }),
            ..
        }) => named,
        _ => unimplemented!("only works for structs with named fields"),
    };

    // This replaced the thing below

    let builder_fields = fields
        .iter()
        .map(|f| syn::parse2::<StructField>(f.to_token_stream()).unwrap());

    // let builder_fields = fields.iter().map(StructField::new);

    // above instead of:

    /*
    let builder_fields = fields.iter().map(|f| {
        let name = &f.ident;
        let ty = &f.ty;
        quote! { pub #name: #ty }
    });
     */

    let public_version = quote! {
        pub struct #name {
            #(#builder_fields,)*
        }
    };

    public_version.into()
}

*/
