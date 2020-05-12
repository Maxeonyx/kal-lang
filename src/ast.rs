use crate::eval::{Eval, IntoEval, Location};
use std::{fmt::Debug, rc::Rc};

pub trait Statement: Eval + IntoEval<dyn Eval> {}

impl<T: Expression> Statement for T {}

#[derive(Debug)]
pub struct ExpressionStatement {
    pub expr: Rc<dyn Statement>,
}
impl Statement for ExpressionStatement {}

pub trait IntoStatement<T: ?Sized> {
    fn into_statement(self: Rc<Self>) -> Rc<T>;
}
impl<'a, T: Statement + 'static> IntoStatement<dyn Statement + 'static> for T {
    fn into_statement(self: Rc<Self>) -> Rc<dyn Statement + 'static> {
        Rc::new(ExpressionStatement {
            expr: self as Rc<dyn Statement>,
        })
    }
}

pub trait Expression: Eval + IntoEval<dyn Eval> + Statement + IntoStatement<dyn Statement> {}

#[derive(Debug)]
pub struct Null;
impl Expression for Null {}

#[derive(Debug)]
pub struct Bool(pub bool);
impl Expression for Bool {}

#[derive(Debug)]
pub struct Int(pub i64);
impl Expression for Int {}

#[derive(Debug)]
pub struct LetStatement {
    pub ident: String,
    pub expr: Rc<dyn Expression>,
}
impl Statement for LetStatement {}

#[derive(Debug)]
pub struct Assignment {
    pub location: LocationChain,
    pub expr: Rc<dyn Expression>,
}
impl Statement for Assignment {}

#[derive(Debug)]
pub struct NegativeExpression {
    pub expr: Rc<dyn Expression>,
}
impl Expression for NegativeExpression {}

#[derive(Debug)]
pub struct NotExpression {
    pub expr: Rc<dyn Expression>,
}
impl Expression for NotExpression {}

#[derive(Debug)]
pub struct BooleanExpression {
    pub left: Rc<dyn Expression>,
    pub right: Rc<dyn Expression>,
    pub operator: BooleanOperator,
}
impl Expression for BooleanExpression {}

#[derive(Debug, Clone, Copy)]
pub enum BooleanOperator {
    And,
    Or,
    Xor,
}

#[derive(Debug)]
pub struct DotExpression {
    pub base: Rc<dyn Expression>,
    pub prop: String,
}
impl Expression for DotExpression {}

#[derive(Debug)]
pub struct IndexExpression {
    pub base: Rc<dyn Expression>,
    pub index: Rc<dyn Expression>,
}
impl Expression for IndexExpression {}

#[derive(Debug)]
pub struct ComparisonExpression {
    pub left: Rc<dyn Expression>,
    pub right: Rc<dyn Expression>,
    pub operator: ComparisonOperator,
}
impl Expression for ComparisonExpression {}

#[derive(Debug, Clone, Copy)]
pub enum ComparisonOperator {
    Equal,
    NotEqual,
    LessEqual,
    Less,
    GreaterEqual,
    Greater,
}

#[derive(Debug)]
pub struct NumericExpression {
    pub left: Rc<dyn Expression>,
    pub right: Rc<dyn Expression>,
    pub operator: NumericOperator,
}
impl Expression for NumericExpression {}

#[derive(Debug, Clone, Copy)]
pub enum NumericOperator {
    Add,
    Multiply,
    Subtract,
    Divide,
}

#[derive(Debug)]
pub struct IfExpression {
    pub else_body: Option<Rc<Block>>,
    pub ifs: Vec<IfPart>,
}
impl Expression for IfExpression {}

#[derive(Debug, Clone)]
pub struct IfPart {
    pub cond: Rc<dyn Expression>,
    pub body: Rc<Block>,
}

#[derive(Debug, Clone)]
pub struct LoopExpression {
    pub body: Rc<Block>,
}
impl Expression for LoopExpression {}

#[derive(Debug)]
pub struct Function {
    pub body: Rc<Block>,
    pub parameters: Vec<String>,
}
impl Expression for Function {}

#[derive(Debug)]
pub struct Block {
    pub expression: Option<Rc<dyn Expression>>,
    pub statements: Vec<Rc<dyn Statement>>,
}
impl Expression for Block {}

#[derive(Debug)]
pub struct FunctionInvocation {
    pub base: Rc<dyn Expression>,
    pub parameters: Vec<Rc<dyn Expression>>,
}
impl Expression for FunctionInvocation {}

#[derive(Debug)]
pub struct Object {
    pub pairs: Vec<(String, Rc<dyn Expression>)>,
}
impl Expression for Object {}

#[derive(Debug)]
pub struct List {
    pub elements: Vec<ListElem>,
}
impl Expression for List {}

#[derive(Debug)]
pub enum ListElem {
    Spread(Rc<dyn Expression>),
    Elem(Rc<dyn Expression>),
}

#[derive(Debug)]
pub struct Handle {
    pub expr: Rc<dyn Expression>,
    pub match_arms: Vec<HandleMatch>,
}
impl Expression for Handle {}

#[derive(Debug, Clone)]
pub struct HandleMatch {
    pub symbol: String,
    pub param: String,
    pub block: Rc<Block>,
}

#[derive(Debug)]
pub struct SendExpr {
    pub symbol: String,
    pub expr: Option<Rc<dyn Expression>>,
}
impl Expression for SendExpr {}

#[derive(Debug)]
pub struct Continue {
    pub expr: Option<Rc<dyn Expression>>,
}
impl Expression for Continue {}

#[derive(Debug)]
pub struct Break {
    pub expr: Option<Rc<dyn Expression>>,
}
impl Expression for Break {}

#[derive(Debug)]
pub struct LocationChain {
    pub base: LocationChainBase,
    pub parts: Vec<Rc<dyn Location>>,
}
impl Expression for LocationChain {}

#[derive(Debug)]
pub enum LocationChainBase {
    Ident(String),
    Expression(Rc<dyn Expression>),
}

#[derive(Debug)]
pub struct DotLocation {
    pub prop: String,
}

#[derive(Debug)]
pub struct IndexLocation {
    pub index: Rc<dyn Expression>,
}
