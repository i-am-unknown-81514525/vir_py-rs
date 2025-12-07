use proc_macro2::{Span};
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::parse_macro_input;
use vir_py_rs_parser::error::ParseError;
use vir_py_rs_parser::tokenizer::{Block, Stmt, Expr};

fn expr_to_token(expr: Expr) -> impl ToTokens {
    todo!();
    quote! {  }
}

fn stmts_to_token(stmts: Vec<Stmt>) -> impl ToTokens {
    let mut e = Vec::new();
    for stmt in stmts {
        let tokens = stmt_to_token(stmt);
        e.push(tokens);
    }
    quote! {
        vec![
            #(#e),*
        ]
    }
}

fn stmt_to_token(stmt: Stmt) -> impl ToTokens {
    match stmt {
        Stmt::Expr(expr) => {
            let expr_token = expr_to_token(expr);
            quote! {
                ::vir_py_rs_type::ast::core::Node {
                    kind: ::vir_py_rs_type::ast::core::Stmt::Expression( #expr_token ),
                    span: None,
                }
            }
        }
        Stmt::Assign { target, value } => {
            let target_token = expr_to_token(target);
            let value_token = expr_to_token(value);
            quote! {
                ::vir_py_rs_type::ast::core::Node {
                    kind: ::vir_py_rs_type::ast::core::Stmt::Assign {
                        target: #target_token,
                        value: #value_token,
                    },
                    span: None,
                }
            }
        }
        Stmt::If { test, body, otherwise } => {
            let test_token = expr_to_token(test);
            let body_token = stmts_to_token(body.stmts);
            let otherwise_token = match otherwise {
                Some(b) => {
                    let stmts = stmts_to_token(b.stmts);
                    quote! { Some(#stmts) }
                }
                None => quote! { None },
            };

            quote! {
                ::vir_py_rs_type::ast::core::Node {
                    kind: ::vir_py_rs_type::ast::core::Stmt::If {
                        test: #test_token,
                        body: #body_token,
                        otherwise: #otherwise_token,
                    },
                    span: None,
                }
            }
        }
    }
}

fn block_to_token(v: Block) -> impl ToTokens {
    let body = stmts_to_token(v.stmts);
    quote! {
        ::vir_py_rs_type::ast::core::Module {
            body: #body,
            span: None,
        }
    }
}

#[proc_macro]
pub fn parse(input: TokenStream) -> TokenStream {
    let output = parse_macro_input!(input as Block);
    let token_content = block_to_token(output);
    quote! { #token_content }.into()
}
