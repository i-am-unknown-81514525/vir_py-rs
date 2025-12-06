use crate::base::ValueContainer;
use crate::exec_ctx::{ExecutionContext, Result};
use proc_macro2::Span;
use std::cell::RefCell;
use std::rc::Rc;

pub trait ASTNode {
    type Output;
    fn eval(&self, ctx: Rc<RefCell<ExecutionContext>>) -> Result<Self::Output>;

    fn get_callsite(&self) -> Option<Span>;
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
}

impl ASTNode for Module {
    type Output = ();
    fn eval(&self, ctx: Rc<RefCell<ExecutionContext>>) -> Result<()> {
        for stmt in self.body.clone() {
            stmt.kind.eval(ctx.clone())?;
            ctx.borrow_mut().consume_one()?;
        }
        Ok(())
    }

    fn get_callsite(&self) -> Option<Span> {
        todo!()
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
    Call {
        function: Box<Node<Expr>>,
        args: Vec<Node<Expr>>,
    },
    Attribute {
        value: Box<Node<Expr>>,
        attr: String,
    },
    Subscript {
        value: Box<Node<Expr>>,
        slice: Box<Node<Expr>>,
    },
    Range {
        lower: Option<i64>,
        upper: Option<i64>,
        step: Option<i64>,
    },
}

impl ASTNode for Expr {
    type Output = ();

    fn eval(&self, ctx: Rc<RefCell<ExecutionContext>>) -> Result<Self::Output> {
        todo!()
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
    type Output = ();

    fn eval(&self, ctx: Rc<RefCell<ExecutionContext>>) -> Result<Self::Output> {
        todo!()
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
    Pow,
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
    type Output = ();

    fn eval(&self, ctx: Rc<RefCell<ExecutionContext>>) -> Result<Self::Output> {
        todo!()
    }

    fn get_callsite(&self) -> Option<Span> {
        todo!()
    }
}
