use super::{Assignment, Block, Call, Dyadic, Identifier, Literal, Match, Member, Return};

/// An expression, which is a piece of code that can be evaluated to produce a value.
#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Block(Block),
    Match(Match),
    Member(Member),
    Call(Call),
    Identifier(Identifier),
    Literal(Literal),
    Dyadic(Dyadic),
    Return(Return),
    Assignment(Assignment),
}

/// A list of expressions, which can be used in various contexts such as function arguments, tuple elements, etc.
#[derive(Debug, Clone, PartialEq)]
pub struct Expressions {
    pub items: Vec<Expression>,
}
