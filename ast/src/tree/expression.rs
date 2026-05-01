use crate::{
    Assignment, Block, Dyadic, FunctionCall, FunctionDeclaration, Identifier, Literal, Match,
    Member, Return, VariableDeclaration,
};

/// An expression, which is a piece of code that can be evaluated to produce a value.
#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Assignment(Assignment),
    Block(Block),
    Dyadic(Dyadic),
    FunctionCall(FunctionCall),
    FunctionDeclaration(FunctionDeclaration),
    Identifier(Identifier),
    Literal(Literal),
    Match(Match),
    Member(Member),
    Return(Return),
    VariableDeclaration(VariableDeclaration),
}

/// A list of expressions, which can be used in various contexts such as function arguments, tuple elements, etc.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Expressions {
    pub items: Vec<Expression>,
}

impl Expressions {
    pub fn new(items: Vec<Expression>) -> Self {
        Self { items }
    }
}
