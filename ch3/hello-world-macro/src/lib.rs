/*
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Hello)]
pub fn hello(item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);
    let name = ast.ident;

    let add_hello_world = quote! {
        impl #name {
            fn hello_world(&self) {
                println!("Hello world")
            }
        }
    };

    add_hello_world.into()
}
*/

// without sy nand quote:

/*
use proc_macro::{TokenStream, TokenTree};

#[proc_macro_derive(Hello)]
pub fn hello_alt(item: TokenStream) -> TokenStream {
    fn ident_name(item: TokenTree) -> String {
        match item {
            TokenTree::Ident(i) => i.to_string(),
            _ => panic!("no ident")
        }
    }
    let name = ident_name(item.into_iter().nth(1).unwrap());

    format!("impl {} {{ fn hello_world(&self) \
    {{ println!(\"Hello world\") }} }} ", name
        ).parse()
        .unwrap()
}
*/

// with venial:

use quote::quote;
use proc_macro::TokenStream;
use venial::{parse_declaration, Declaration, Struct, Enum};

#[proc_macro_derive(Hello)]
pub fn hello(item: TokenStream) -> TokenStream {
    let declaration = parse_declaration(item.into()).unwrap();

    let name = match declaration {
        Declaration::Struct(Struct { name, .. }) => name,
        Declaration::Enum(Enum { name, .. }) => name,
        _ => panic!("only implemented for struct and enum")
    };

    let add_hello_world = quote! {
        impl #name {
            fn hello_world(&self) {
                println!("Hello {}", stringify!(#name));
            }

            fn testing_testing() {
                println!("one two three");
            }
        }
    };

    add_hello_world.into()
}

// exercise

// use proc_macro::TokenStream;
// use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(UpperCaseName)]
pub fn uppercase(item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);
    let name = ast.ident;
    let uppercase_name = name.to_string().to_uppercase();

    let add_uppercase = quote! {
        impl #name {
            fn uppercase(&self) {
                println!("{}", #uppercase_name);
            }
        }
    };
    add_uppercase.into()
}
