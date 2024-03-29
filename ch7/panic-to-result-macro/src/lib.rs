use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{spanned::Spanned, token::Semi, Expr, ItemFn, ReturnType, Stmt, StmtMacro};

fn signature_output_as_result(ast: &ItemFn) -> Result<ReturnType, syn::Error> {
    let output = match ast.sig.output {
        ReturnType::Default => {
            quote! {
                -> Result<(), String>
            }
        }
        ReturnType::Type(_, ref ty) => {
            if ty.to_token_stream().to_string().contains("Result") {
                return Err(syn::Error::new(
                    ast.sig.span(),
                    format!(
                        "cannot use macro on a function with Result as return type. Signature: {}",
                        quote!(#ty)
                    ),
                ));
            }
            quote! {
                -> Result<#ty, String>
            }
        }
    };
    Ok(syn::parse2(output).unwrap())
}

fn last_statement_as_result(last_statement: Option<Stmt>) -> Stmt {
    let last_unwrapped = last_statement.unwrap();
    let last_modified = quote! {
        Ok(#last_unwrapped)
    };
    Stmt::Expr(syn::parse2(last_modified).unwrap(), None)
}

fn handle_expression(expression: Expr, token: Option<Semi>) -> Result<Stmt, syn::Error> {
    match expression.clone() {
        Expr::If(mut ex_if) => {
            let new_statements: Result<Vec<Stmt>, syn::Error> = ex_if
                .then_branch
                .stmts
                .into_iter()
                .map(|s| match s {
                    Stmt::Macro(ref expr_macro) => {
                        let output = extract_panic_content(expr_macro);

                        if output.as_ref().map(|v| v.is_empty()).unwrap_or_default() {
                            Err(syn::Error::new(
                                expr_macro.span(),
                                format!("please make sure every panic in your function has a message, check: {}", quote!(#expression))
                            ))
                        } else {
                            Ok(output
                                .map(|t| {
                                    quote! {
                                        return Err(#t.to_string());
                                    }
                                })
                                .map(syn::parse2)
                                .map(Result::unwrap)
                                .unwrap_or(s))
                        }
                    }
                    _ => Ok(s),
                })
                .collect();
            ex_if.then_branch.stmts = new_statements?;
            Ok(Stmt::Expr(Expr::If(ex_if), token))
        }
        _ => Ok(Stmt::Expr(expression, token)),
    }
}

fn extract_panic_content(expr_macro: &StmtMacro) -> Option<proc_macro2::TokenStream> {
    let does_panic = expr_macro
        .mac
        .path
        .segments
        .iter()
        .any(|v| v.ident.to_string().eq("panic"));

    if does_panic {
        Some(expr_macro.mac.tokens.clone())
    } else {
        None
    }
}

#[proc_macro_attribute]
pub fn panic_to_result(_a: TokenStream, item: TokenStream) -> TokenStream {
    let mut ast: ItemFn = syn::parse(item).unwrap();

    let signature_output = signature_output_as_result(&ast);

    let last_statement = ast.block.stmts.pop();
    ast.block
        .stmts
        .push(last_statement_as_result(last_statement));

    let statements_output: Result<Vec<Stmt>, syn::Error> = ast
        .block
        .stmts
        .into_iter()
        .map(|s| match s {
            Stmt::Expr(e, t) => handle_expression(e, t),
            _ => Ok(s),
        })
        .collect();

    match (statements_output, signature_output) {
        (Ok(statements), Ok(output)) => {
            ast.sig.output = output;
            ast.block.stmts = statements;
        }
        (Ok(_), Err(e)) => return e.to_compile_error().into(),
        (Err(e), Ok(_)) => return e.to_compile_error().into(),
        (Err(mut statements_error), Err(signature_error)) => {
            statements_error.combine(signature_error);
            return statements_error.to_compile_error().into();
        }
    }

    ast.to_token_stream().into()
}
