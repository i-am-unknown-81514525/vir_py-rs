use crate::base::{ValueContainer, ValueKind};
use crate::builtin::{VirPyFloat, VirPyInt};
use crate::exec_ctx::{ExecutionContext, Result};
use crate::op::*;
use std::cell::{Ref, RefCell};
use std::rc::Rc;

#[derive(Debug, Clone, Copy)]
pub struct Span {
    pub row: u64,
    pub col: u64,
    pub length: u64,
}

pub trait ASTNode {
    type Output<'ctx>; // = ValueKind<'ctx>; // Oh cool that this is a unstable feature?
    fn eval<'ctx>(&self, ctx:Rc<RefCell<ExecutionContext<'ctx>>>) -> Result<Self::Output<'ctx>>;

    fn get_callsite(&self) -> Option<Span>;
}

fn with_arena<'ctx, F, R>(ctx: &Rc<RefCell<ExecutionContext<'ctx>>>, f: F) -> R
where
    F: FnOnce(&'ctx bumpalo::Bump) -> R,
{
    let ctx_borrow: Ref<ExecutionContext<'ctx>> = ctx.borrow();
    let arena_borrow: Ref<bumpalo::Bump> = ctx_borrow.arena.borrow();
    let arena_ref: &bumpalo::Bump = &arena_borrow;
    let arena_ref_ctx: &'ctx bumpalo::Bump = unsafe { std::mem::transmute(arena_ref) };
    f(arena_ref_ctx)
}

#[derive(Debug, Clone)]
pub struct Node<T>
where
    T: ASTNode,
{
    pub kind: T,
    pub span: Option<Span>,
}

pub struct Module {
    pub body: Vec<Node<Stmt>>,
    pub span: Option<Span>,
}

impl ASTNode for Module {
    type Output<'ctx> = ValueKind<'ctx>;
    fn eval<'ctx>(&self, ctx: Rc<RefCell<ExecutionContext<'ctx>>>) -> Result<ValueKind<'ctx>> {
        for stmt in self.body.clone() {
            stmt.kind.eval(ctx.clone())?;
            ctx.borrow_mut().consume_one()?;
        }
        Ok(ValueKind::None)
    }

    fn get_callsite(&self) -> Option<Span> {
        self.span
    }
}

#[derive(Debug, Clone)]
pub enum Expr {
    Literal(Literal),
    Variable(String),
    BinaryOp {
        left: Box<Node<Expr>>,
        op: BinaryOperator,
        right: Box<Node<Expr>>,
    },
    UnaryOp {
        op: UaryOperator,
        operand: Box<Node<Expr>>,
    },
    // Call {
    //     function: Box<Node<Expr>>,
    //     args: Vec<Node<Expr>>,
    // },
    // Attribute {
    //     value: Box<Node<Expr>>,
    //     attr: String,
    // },
    // Subscript {
    //     value: Box<Node<Expr>>,
    //     slice: Box<Node<Expr>>,
    // },
    // Range {
    //     lower: Option<i64>,
    //     upper: Option<i64>,
    //     step: Option<i64>,
    // },
}

impl ASTNode for Expr {
    type Output<'ctx> = ValueKind<'ctx>;

    fn eval<'ctx>(&self, ctx:Rc<RefCell<ExecutionContext<'ctx>>>) -> Result<Self::Output<'ctx>> {
        ctx.borrow_mut().consume_one()?;
        match self {
            Expr::Literal(l) => l.eval(ctx),
            Expr::Variable(v) => Ok(ctx.borrow().get(v)?.borrow().kind.clone()),
            Expr::UnaryOp { op, operand } => {
                let rhs_kind = operand.kind.eval(ctx.clone())?;
                with_arena(&ctx, |arena| {
                    let rhs = ValueContainer::new(rhs_kind, arena);
                    match op {
                        UaryOperator::Negative => Ok(err_op_neg(rhs, arena)?.kind.clone()),
                        UaryOperator::Positive => Ok(err_op_pos(rhs, arena)?.kind.clone()),
                        UaryOperator::Not => Ok(err_op_not(rhs, arena)?.kind.clone()),
                    }
                })
            }
            _ => todo!()
        }
    }

    fn get_callsite(&self) -> Option<Span> {
        todo!()
    }
}

#[derive(Debug, Clone)]
pub enum Literal {
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),
    None,
}

impl ASTNode for Literal {
    type Output<'ctx> = ValueKind<'ctx>;

    fn eval<'ctx>(&self, ctx: Rc<RefCell<ExecutionContext<'ctx>>>) -> Result<Self::Output<'ctx>> {
        match self {
            Literal::Bool(v) => Ok(ValueKind::Bool(*v)),
            Literal::None => Ok(ValueKind::None),
            Literal::Int(v) => Ok(ValueKind::Int(VirPyInt::new(*v))),
            Literal::Float(v) => Ok(ValueKind::Float(VirPyFloat::new(*v))),
            Literal::String(v) => Ok(ValueKind::String(v.clone())),
        }
    }

    fn get_callsite(&self) -> Option<Span> {
        todo!()
    }
}

#[derive(Debug, Clone, Copy)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    And,
    Or,
    Xor,
    Modulo,
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    Eq,
    NotEq,
    Lt,
    Lte,
    Gt,
    Gte,
}

#[derive(Debug, Clone, Copy)]
pub enum UaryOperator {
    Positive,
    Negative,
    Not,
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Expression(Node<Expr>),
    Assign {
        target: Node<Expr>,
        value: Node<Expr>,
    },
    If {
        test: Node<Expr>,
        body: Vec<Node<Stmt>>,
        otherwise: Option<Vec<Node<Stmt>>>,
    },
    FunctionDef {
        name: String,
        args: Vec<String>,
        body: Vec<Node<Stmt>>,
    },
    ClassDef {
        name: String,
        bases: Vec<Node<Expr>>,
        body: Vec<Node<Stmt>>,
    },
    ForLoop {
        target: Node<Expr>, // Added target
        iter_expr: Node<Expr>,
        body: Vec<Node<Stmt>>,
        not_break: Vec<Node<Stmt>>,
    },
    WhileLoop {
        test: Node<Expr>,
        body: Vec<Node<Stmt>>,
        otherwise: Option<Vec<Node<Stmt>>>, // Added otherwise
    },
    Return(Option<Node<Expr>>),
    Break,
    Continue,
}

impl ASTNode for Stmt {
    type Output<'ctx> = ();

    fn eval<'ctx>(&self, ctx: Rc<RefCell<ExecutionContext<'ctx>>>) -> Result<Self::Output<'ctx>> {
        todo!()
    }

    fn get_callsite(&self) -> Option<Span> {
        todo!()
    }
}
