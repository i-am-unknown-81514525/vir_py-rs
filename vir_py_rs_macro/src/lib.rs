use proc_macro2::{Span, TokenStream as TokenStream2};
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::parse_macro_input;
use vir_py_rs_parser::tokenizer::{Block, Stmt, Expr, Atom};
use vir_py_rs_type::ast::core::{BinaryOperator, UnaryOperator, Literal};

fn literal_to_token(lit: Literal) -> impl ToTokens {
    match lit {
        Literal::Int(v) => quote! { ::vir_py_rs_type::ast::core::Literal::Int(#v) },
        Literal::Float(v) => quote! { ::vir_py_rs_type::ast::core::Literal::Float(#v) },
        Literal::String(v) => quote! { ::vir_py_rs_type::ast::core::Literal::String(#v.to_string()) },
        Literal::Bool(v) => quote! { ::vir_py_rs_type::ast::core::Literal::Bool(#v) },
        Literal::None => quote! { ::vir_py_rs_type::ast::core::Literal::None },
    }
}

fn binary_op_to_token(op: BinaryOperator) -> impl ToTokens {
    match op {
        BinaryOperator::Add => quote! { ::vir_py_rs_type::ast::core::BinaryOperator::Add },
        BinaryOperator::Subtract => quote! { ::vir_py_rs_type::ast::core::BinaryOperator::Subtract },
        BinaryOperator::Multiply => quote! { ::vir_py_rs_type::ast::core::BinaryOperator::Multiply },
        BinaryOperator::Divide => quote! { ::vir_py_rs_type::ast::core::BinaryOperator::Divide },
        BinaryOperator::And => quote! { ::vir_py_rs_type::ast::core::BinaryOperator::And },
        BinaryOperator::Or => quote! { ::vir_py_rs_type::ast::core::BinaryOperator::Or },
        BinaryOperator::Xor => quote! { ::vir_py_rs_type::ast::core::BinaryOperator::Xor },
        BinaryOperator::Modulo => quote! { ::vir_py_rs_type::ast::core::BinaryOperator::Modulo },
        BinaryOperator::BitwiseAnd => quote! { ::vir_py_rs_type::ast::core::BinaryOperator::BitwiseAnd },
        BinaryOperator::BitwiseOr => quote! { ::vir_py_rs_type::ast::core::BinaryOperator::BitwiseOr },
        BinaryOperator::Eq => quote! { ::vir_py_rs_type::ast::core::BinaryOperator::Eq },
        BinaryOperator::NotEq => quote! { ::vir_py_rs_type::ast::core::BinaryOperator::NotEq },
        BinaryOperator::Lt => quote! { ::vir_py_rs_type::ast::core::BinaryOperator::Lt },
        BinaryOperator::Lte => quote! { ::vir_py_rs_type::ast::core::BinaryOperator::Lte },
        BinaryOperator::Gt => quote! { ::vir_py_rs_type::ast::core::BinaryOperator::Gt },
        BinaryOperator::Gte => quote! { ::vir_py_rs_type::ast::core::BinaryOperator::Gte },
        BinaryOperator::LeftShift => quote! { ::vir_py_rs_type::ast::core::BinaryOperator::LeftShift },
        BinaryOperator::RightShift => quote! { ::vir_py_rs_type::ast::core::BinaryOperator::RightShift },
    }
}

fn unary_op_to_token(op: UnaryOperator) -> impl ToTokens {
    match op {
        UnaryOperator::Positive => quote! { ::vir_py_rs_type::ast::core::UnaryOperator::Positive },
        UnaryOperator::Negative => quote! { ::vir_py_rs_type::ast::core::UnaryOperator::Negative },
        UnaryOperator::Not => quote! { ::vir_py_rs_type::ast::core::UnaryOperator::Not },
    }
}

fn atom_to_token(atom: Atom) -> TokenStream2 {
    match atom {
        Atom::Literal(l) => {
            let lit_token = literal_to_token(l);
            quote! { ::vir_py_rs_type::ast::core::Expr::Literal(#lit_token) }
        }
        Atom::Variable(v) => {
            quote! { ::vir_py_rs_type::ast::core::Expr::Variable(#v.to_string()) }
        }
        Atom::Paren(expr) => {
            let expr_token = expr_to_token(*expr);
            return quote! {
                ::vir_py_rs_type::ast::core::Expr::Wrapped(
                    Box::new(#expr_token)
                )
            }
        }
    }
}

fn expr_to_token(expr: Expr) -> impl ToTokens {
    let kind = match expr {
        Expr::Atom(atom) => atom_to_token(atom),
        Expr::Binary(left, op, right) => {
            let left_token = expr_to_token(*left);
            let op_token = binary_op_to_token(op);
            let right_token = expr_to_token(*right);
            quote! {
                ::vir_py_rs_type::ast::core::Expr::BinaryOp {
                    left: Box::new(#left_token),
                    op: #op_token,
                    right: Box::new(#right_token),
                }
            }
        }
        Expr::Unary(op, operand) => {
            let op_token = unary_op_to_token(op);
            let operand_token = expr_to_token(*operand);
            quote! {
                ::vir_py_rs_type::ast::core::Expr::UnaryOp {
                    op: #op_token,
                    operand: Box::new(#operand_token),
                }
            }
        }
    };
    quote! {
        ::vir_py_rs_type::ast::core::Node {
            kind: #kind,
            span: None,
        }
    }
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
