use proc_macro2::{Span};
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::parse_macro_input;
use vir_py_rs_parser::error::ParseError;
use vir_py_rs_parser::tokenizer::{Block, Stmt};
use vir_py_rs_type::ast::core::{Module};

fn stmt_to_token(stmt: Stmt) -> impl ToTokens {
    todo!();
    quote! { }
}

fn block_to_token(v: Block) -> impl ToTokens {
    let mut e = Vec::new();
    for stmt in v.stmts {
        let tokens = stmt_to_token(stmt);
        e.push(tokens);
    }
    quote! {
        ::vir_py_rs_type::ast::core::Module {
            body: vec![
                #(#e),*
            ],
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