use proc_macro::TokenStream;
use proc_macro_error::{emit_error, proc_macro_error};
use quote::{quote, ToTokens};
use syn::{token::Semi, Expr, ItemFn, ReturnType, Stmt, StmtMacro};

fn signature_output_as_result(ast: &ItemFn) -> ReturnType {
    let output = match ast.sig.output {
        ReturnType::Default => {
            quote! {
                -> Result<(), String>
            }
        }
        ReturnType::Type(_, ref ty) => {
            if ty.to_token_stream().to_string().contains("Result") {
                emit_error!(
                    ty,
                    "cannot use macro on a function with Result as return type";
                    help = "try to remove the Result from the return type";
                    note = "we will wrap your return type in Result<_, String>";
                );
                ast.sig.output.to_token_stream()
            } else {
                quote! {
                    -> Result<#ty, String>
                }
            }
        }
    };
    syn::parse2(output).unwrap()
}

fn last_statement_as_result(last_statement: Option<Stmt>) -> Stmt {
    let last_unwrapped = last_statement.unwrap();
    let last_modified = quote! {
        Ok(#last_unwrapped)
    };
    Stmt::Expr(syn::parse2(last_modified).unwrap(), None)
}

fn handle_expression(expression: Expr, token: Option<Semi>) -> Stmt {
    match expression.clone() {
        Expr::If(mut ex_if) => {
            ex_if.then_branch.stmts = ex_if
                .then_branch
                .stmts
                .into_iter()
                .map(|s| match s {
                    Stmt::Macro(ref expr_macro) => {
                        let output = extract_panic_content(expr_macro);

                        if output.as_ref().map(|v| v.is_empty()).unwrap_or_default() {
                            emit_error!(
                                expr_macro,
                                "panic needs a message!";
                                help = "try to add a message: `panic!(\"message\")`";
                                note = "we will add the message to Result's Err"
                            );
                            s
                        } else {
                            output
                                .map(|t| {
                                    quote! {
                                        return Err(#t.to_string());
                                    }
                                })
                                .map(syn::parse2)
                                .map(Result::unwrap)
                                .unwrap_or(s)
                        }
                    }
                    _ => s,
                })
                .collect();
            Stmt::Expr(Expr::If(ex_if), token)
        }
        _ => Stmt::Expr(expression, token),
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

#[proc_macro_error]
#[proc_macro_attribute]
pub fn panic_to_result(_a: TokenStream, item: TokenStream) -> TokenStream {
    let mut ast: ItemFn = syn::parse(item).unwrap();

    ast.sig.output = signature_output_as_result(&ast);

    let last_statement = ast.block.stmts.pop();
    ast.block
        .stmts
        .push(last_statement_as_result(last_statement));

    ast.block.stmts = ast
        .block
        .stmts
        .into_iter()
        .map(|s| match s {
            Stmt::Expr(e, t) => handle_expression(e, t),
            _ => s,
        })
        .collect();

    ast.to_token_stream().into()
}
