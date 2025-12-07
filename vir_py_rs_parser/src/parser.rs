use crate::tokenizer;
use vir_py_rs_type::ast::core as final_ast;
use crate::error::ParseError;

// --- Conversion Layer (Intermediate AST -> Final AST) ---

fn convert_expr(expr: tokenizer::Expr) -> final_ast::Node<final_ast::Expr> {
    let kind = match expr {
        tokenizer::Expr::Atom(atom) => match atom {
            tokenizer::Atom::Literal(l) => final_ast::Expr::Literal(l),
            tokenizer::Atom::Variable(v) => final_ast::Expr::Variable(v),
            tokenizer::Atom::Paren(expr_in_paren) => {
                return convert_expr(*expr_in_paren);
            }
        },
        tokenizer::Expr::Binary(left, op, right) => final_ast::Expr::BinaryOp {
            left: Box::new(convert_expr(*left)),
            op,
            right: Box::new(convert_expr(*right)),
        },
        tokenizer::Expr::Unary(op, operand) => final_ast::Expr::UnaryOp {
            op,
            operand: Box::new(convert_expr(*operand)),
        },
    };
    final_ast::Node { kind, span: None }
}


pub fn parse(source: &str) -> std::result::Result<final_ast::Node<final_ast::Expr>, ParseError> {
    let intermediate_expr: tokenizer::Expr = syn::parse_str(source).map_err(ParseError::SynParseError)?;
    Ok(convert_expr(intermediate_expr))
}
