use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{punctuated::Punctuated, token::Comma, Field, Ident, Type};

pub fn builder_field_definitions(
    fields: &Punctuated<Field, Comma>,
) -> impl Iterator<Item = TokenStream> + '_ {
    fields.iter().map(|f| {
        let (name, f_type) = get_name_and_type(f);
        match name {
            Some(name) => quote! { #name: Option<#f_type> },
            None => quote! { Option<#f_type> },
        }
    })
}

pub fn builder_init_values(
    fields: &Punctuated<Field, Comma>,
) -> impl Iterator<Item = TokenStream> + '_ {
    fields.iter().map(|f| {
        match &f.ident {
            Some(field_name) => quote! { #field_name: None },
            None => quote! { None },
        }
    })
}

pub fn builder_methods(
    fields: &Punctuated<Field, Comma>,
) -> impl Iterator<Item = TokenStream> + '_ {
    fields.iter().enumerate().map(|(i, f)| {
        match &f.ident {
            Some(field_name) => {
                let maybe_field_name = format_ident!("maybe_{}", field_name);
                let field_type = &f.ty;
                quote! {
                    pub fn #field_name(mut self, input: #field_type) -> Self {
                        self.#field_name = Some(input);
                        self
                    }

                    pub fn #maybe_field_name(mut self, maybe_input: Option<#field_type>) -> Self {
                        self.#field_name = maybe_input;
                        self
                    }
                }
            }
            None => {
                let field_name = format_ident!("field_{}", i);
                let maybe_field_name = format_ident!("maybe_field_{}", i);
                let field_type = &f.ty;
                let index = syn::Index::from(i);
                quote! {
                    pub fn #field_name(mut self, input: #field_type) -> Self {
                        self.#index = Some(input);
                        self
                    }

                    pub fn #maybe_field_name(mut self, maybe_input: Option<#field_type>) -> Self {
                        self.#index = maybe_input;
                        self
                    }
                }
            }
        }
    })
}

/*
pub fn original_struct_setters(
    fields: &Punctuated<Field, Comma>,
) -> impl Iterator<Item = TokenStream> + '_ {
    fields.iter().map(|f| {
        let (field_name, field_type) = get_name_and_type(f);
        let field_name_as_string = field_name.as_ref().unwrap().to_string();

        if matches_type(field_type, "String") {
            quote! {
                #field_name: self.#field_name.as_ref()
                    .expect(
                        &format!("field {} not set", #field_name_as_string)
                    ).to_string()
            }
        } else {
            // assume a primitive or another copy type
            quote! {
                #field_name: self.#field_name
                    .expect(
                        &format!("field {} not set", #field_name_as_string)
                    )
            }
        }
    })
}
*/

// de-deplication of the above

/*
pub fn original_struct_setters(
    fields: &Punctuated<Field, Comma>,
) -> impl Iterator<Item = TokenStream> + '_ {
    fields.iter().map(|f| {
        let (field_name, field_type) = get_name_and_type(f);
        let field_name_as_string = field_name.as_ref().unwrap().to_string();
        let error = quote!(expect(&format!("Field {} not set", #field_name_as_string)));

        let handle_type = if matches_type(field_type, "String") {
            quote! {
                    as_ref()
                    .#error
                    .to_string()
            }
        } else {
            quote! {
                #error
            }
        };

        quote! {
            #field_name: self.#field_name.#handle_type
        }
    })
}
*/

// when moving we no longer need to see strings as special:

pub fn original_struct_setters(
    fields: &Punctuated<Field, Comma>,
) -> impl Iterator<Item = TokenStream> + '_ {
    fields.iter().enumerate().map(|(i, f)| {
        let field_name = &f.ident;
        match field_name {
            Some(field_name) => {
                let field_name_as_string = field_name.to_string();

                quote! {
                    #field_name: self.#field_name
                        .expect(
                            concat!("field not set: ", #field_name_as_string),
                        )
                }
            }
            None => {
                let index = syn::Index::from(i);
                quote! {
                    self.#index.expect(concat!("field not set: ", #i))
                }
            }
        }
        
    })
}

fn get_name_and_type<'a>(f: &'a Field) -> (&'a Option<Ident>, &'a Type) {
    let field_name = &f.ident;
    let field_type = &f.ty;
    (field_name, field_type)
}

// fn matches_type(ty: &Type, type_name: &str) -> bool {
//     if let Type::Path(ref p) = ty {
//         let first_match = p.path.segments[0].ident.to_string();
//         return first_match == *type_name;
//     }
//     false
// }

#[cfg(test)]
mod tests {
    use proc_macro2::Span;
    use syn::{FieldMutability, Path, PathSegment, TypePath, Visibility};

    use super::*;

    #[test]
    fn get_name_and_type_give_back_name() {
        let p = PathSegment {
            ident: Ident::new("String", Span::call_site()),
            arguments: Default::default(),
        };
        let mut pun = Punctuated::new();
        pun.push(p);
        let ty = Type::Path(TypePath {
            qself: None,
            path: Path {
                leading_colon: None,
                segments: pun,
            },
        });
        let f = Field {
            attrs: vec![],
            vis: Visibility::Inherited,
            mutability: FieldMutability::None,
            ident: Some(Ident::new("example", Span::call_site())),
            colon_token: None,
            ty,
        };

        let (actual_name, _) = get_name_and_type(&f);

        assert_eq!(
            actual_name.as_ref().unwrap().to_string(),
            "example".to_string()
        )
    }
}
