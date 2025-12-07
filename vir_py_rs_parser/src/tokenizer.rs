use syn::parse::{Parse, ParseStream, Result};
use syn::{braced, parenthesized, Ident, Lit, Token};
use vir_py_rs_type::ast::core as final_ast;
use vir_py_rs_type::ast::core::BinaryOperator;

pub enum Expr {
    Atom(Atom),
    Binary(Box<Expr>, final_ast::BinaryOperator, Box<Expr>),
    Unary(final_ast::UnaryOperator, Box<Expr>),
}

pub enum Atom {
    Literal(final_ast::Literal),
    Variable(String),
    Paren(Box<Expr>),
}


impl Parse for Expr {
    fn parse(input: ParseStream) -> Result<Self> {
        parse_expr_with_precedence(input, 0)
    }
}

// Pratt
fn parse_expr_with_precedence(input: ParseStream, min_bp: u8) -> Result<Expr> {
    // First, parse the left-hand side, which can be a prefix operator or an atom.
    let mut lhs = if input.peek(Token![!]) {
        input.parse::<Token![!]>()?;
        let rhs = parse_expr_with_precedence(input, prefix_binding_power(&final_ast::UnaryOperator::Not))?;
        Expr::Unary(final_ast::UnaryOperator::Not, Box::new(rhs))
    } else if input.peek(Token![-]) {
        input.parse::<Token![-]>()?;
        let rhs = parse_expr_with_precedence(input, prefix_binding_power(&final_ast::UnaryOperator::Negative))?;
        Expr::Unary(final_ast::UnaryOperator::Negative, Box::new(rhs))
    } else {
        Expr::Atom(input.parse()?)
    };

    loop {
        let (op, l_bp, r_bp) = match peek_infix_op(input) {
            Some(op_data) if op_data.1 >= min_bp => op_data,
            _ => break,
        };

        // Consume the operator token.
        consume_op(input, &op)?;

        let rhs = parse_expr_with_precedence(input, r_bp)?;
        lhs = Expr::Binary(Box::new(lhs), op, Box::new(rhs));
    }

    Ok(lhs)
}

impl Parse for Atom {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(Lit) {
            let lit: Lit = input.parse()?;
            let final_lit = match lit {
                Lit::Int(i) => final_ast::Literal::Int(i.base10_parse()?),
                Lit::Float(f) => final_ast::Literal::Float(f.base10_parse()?),
                Lit::Str(s) => final_ast::Literal::String(s.value()),
                Lit::Bool(b) => final_ast::Literal::Bool(b.value),
                _ => return Err(input.error("unsupported literal type")),
            };
            Ok(Atom::Literal(final_lit))
        } else if input.peek(Ident) {
            let ident: Ident = input.parse()?;
            Ok(Atom::Variable(ident.to_string()))
        } else if input.peek(syn::token::Paren) {
            let content;
            parenthesized!(content in input);
            Ok(Atom::Paren(Box::new(content.parse()?)))
        } else {
            Err(input.error("expected a literal, an identifier, or a parenthesized expression"))
        }
    }
}

// --- Operator Precedence and Binding Power ---

fn prefix_binding_power(op: &final_ast::UnaryOperator) -> u8 {
    match op {
        final_ast::UnaryOperator::Not => 9,
        final_ast::UnaryOperator::Negative | final_ast::UnaryOperator::Positive => 9,
    }
}

fn infix_binding_power(op: &final_ast::BinaryOperator) -> (u8, u8) {
    // Right-associative would be (r_bp, r_bp - 1), e.g. (12, 11)
    match op {
        final_ast::BinaryOperator::Or => (1, 2),
        final_ast::BinaryOperator::And => (3, 4),
        final_ast::BinaryOperator::Eq | final_ast::BinaryOperator::NotEq => (5, 6),
        final_ast::BinaryOperator::Lt | final_ast::BinaryOperator::Lte | final_ast::BinaryOperator::Gt | final_ast::BinaryOperator::Gte => (7, 8),
        final_ast::BinaryOperator::BitwiseOr => (9, 10),
        final_ast::BinaryOperator::Xor => (11, 12),
        final_ast::BinaryOperator::BitwiseAnd => (13, 14),
        final_ast::BinaryOperator::LeftShift | final_ast::BinaryOperator::RightShift => (15, 16),
        final_ast::BinaryOperator::Add | final_ast::BinaryOperator::Subtract => (17, 18),
        final_ast::BinaryOperator::Multiply | final_ast::BinaryOperator::Divide | final_ast::BinaryOperator::Modulo => (19, 20),
    }
}

fn peek_infix_op(input: ParseStream) -> Option<(final_ast::BinaryOperator, u8, u8)> {
    let op = if input.peek(Token![+]) { final_ast::BinaryOperator::Add }
    else if input.peek(Token![-]) { final_ast::BinaryOperator::Subtract }
    else if input.peek(Token![*]) { final_ast::BinaryOperator::Multiply }
    else if input.peek(Token![/]) { final_ast::BinaryOperator::Divide }
    else if input.peek(Token![%]) { final_ast::BinaryOperator::Modulo }
    else if input.peek(Token![&&]) { final_ast::BinaryOperator::And }
    else if input.peek(Token![||]) { final_ast::BinaryOperator::Or }
    else if input.peek(Token![==]) { final_ast::BinaryOperator::Eq }
    else if input.peek(Token![!=]) { final_ast::BinaryOperator::NotEq }
    else if input.peek(Token![<]) { final_ast::BinaryOperator::Lt }
    else if input.peek(Token![<=]) { final_ast::BinaryOperator::Lte }
    else if input.peek(Token![>]) { final_ast::BinaryOperator::Gt }
    else if input.peek(Token![>=]) { final_ast::BinaryOperator::Gte }
    else if input.peek(Token![<<]) {final_ast::BinaryOperator::LeftShift }
    else if input.peek(Token![>>]) { final_ast::BinaryOperator::RightShift }
    else { return None; };
    let (l_bp, r_bp) = infix_binding_power(&op);
    Some((op, l_bp, r_bp))
}

fn consume_op(input: ParseStream, op: &final_ast::BinaryOperator) -> Result<()> {
    match op {
        final_ast::BinaryOperator::Add => input.parse::<Token![+]>().map(|_| ()),
        final_ast::BinaryOperator::Subtract => input.parse::<Token![-]>().map(|_| ()),
        final_ast::BinaryOperator::Multiply => input.parse::<Token![*]>().map(|_| ()),
        final_ast::BinaryOperator::Divide => input.parse::<Token![/]>().map(|_| ()),
        final_ast::BinaryOperator::Modulo => input.parse::<Token![%]>().map(|_| ()),
        final_ast::BinaryOperator::And => input.parse::<Token![&&]>().map(|_| ()),
        final_ast::BinaryOperator::Or => input.parse::<Token![||]>().map(|_| ()),
        final_ast::BinaryOperator::Eq => input.parse::<Token![==]>().map(|_| ()),
        final_ast::BinaryOperator::NotEq => input.parse::<Token![!=]>().map(|_| ()),
        final_ast::BinaryOperator::Lt => input.parse::<Token![<]>().map(|_| ()),
        final_ast::BinaryOperator::Lte => input.parse::<Token![<=]>().map(|_| ()),
        final_ast::BinaryOperator::Gt => input.parse::<Token![>]>().map(|_| ()),
        final_ast::BinaryOperator::Gte => input.parse::<Token![>=]>().map(|_| ()),
        final_ast::BinaryOperator::Xor => input.parse::<Token![^]>().map(|_| ()),
        final_ast::BinaryOperator::BitwiseAnd => input.parse::<Token![&]>().map(|_| ()),
        final_ast::BinaryOperator::BitwiseOr => input.parse::<Token![|]>().map(|_| ()),
        final_ast::BinaryOperator::LeftShift => input.parse::<Token![<<]>().map(|_| ()),
        final_ast::BinaryOperator::RightShift => input.parse::<Token![>>]>().map(|_| ())
    }
}


pub enum AssinOp {
    Set, // =
    AddSet, // + =
    SubSet, // -=
    MulSet, // *=
    DivSet, // /=
    ModSet, // %=
    BitAndSet, // &=
    BitOrSet, // |=
    BitXorSet, // ^=
    LeftShiftSet, // <<=
    RightShiftSet, // >>=
}

pub enum Stmt {
    Expr(Expr),
    Assign(Expr, AssinOp, Expr),
    If {
        test: Expr,
        body: Vec<Stmt>,
        otherwise: Option<Vec<Stmt>>,
    }
}

fn map_assign_op_to_binary_op(op: AssinOp) -> Option<final_ast::BinaryOperator> { // None -> Plain eq
    Some(match op {
        AssinOp::Set => return None,
        AssinOp::AddSet => final_ast::BinaryOperator::Add,
        AssinOp::SubSet => final_ast::BinaryOperator::Subtract,
        AssinOp::MulSet => final_ast::BinaryOperator::Multiply,
        AssinOp::DivSet => final_ast::BinaryOperator::Divide,
        AssinOp::ModSet => final_ast::BinaryOperator::Modulo,
        AssinOp::BitAndSet => final_ast::BinaryOperator::BitwiseAnd,
        AssinOp::BitOrSet => final_ast::BinaryOperator::BitwiseOr,
        AssinOp::BitXorSet => final_ast::BinaryOperator::Xor,
        AssinOp::LeftShiftSet => final_ast::BinaryOperator::LeftShift,
        AssinOp::RightShiftSet => final_ast::BinaryOperator::RightShift
    })
}
