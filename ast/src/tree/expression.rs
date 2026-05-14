use crate::{
    Assignment, Block, Dyadic, FunctionCall, FunctionDeclaration, Identifier,
    Literal, Match, Member, Return, VariableDeclaration,
};

/// An expression, which is a piece of code that can be evaluated to produce a
/// value.
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

/// A list of expressions, which can be used in various contexts such as
/// function arguments, tuple elements, etc.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Expressions {
    pub items: Vec<Expression>,
}

impl Expressions {
    pub fn new(items: Vec<Expression>) -> Self {
        Self { items }
    }

    pub fn add_expression(&mut self, expression: Expression) -> &mut Self {
        self.items.push(expression);
        self
    }
}

impl From<Assignment> for Expression {
    fn from(assignment: Assignment) -> Self {
        Expression::Assignment(assignment)
    }
}

impl From<Block> for Expression {
    fn from(block: Block) -> Self {
        Expression::Block(block)
    }
}

impl From<Dyadic> for Expression {
    fn from(dyadic: Dyadic) -> Self {
        Expression::Dyadic(dyadic)
    }
}

impl From<FunctionCall> for Expression {
    fn from(function_call: FunctionCall) -> Self {
        Expression::FunctionCall(function_call)
    }
}

impl From<FunctionDeclaration> for Expression {
    fn from(function_declaration: FunctionDeclaration) -> Self {
        Expression::FunctionDeclaration(function_declaration)
    }
}

impl From<Identifier> for Expression {
    fn from(identifier: Identifier) -> Self {
        Expression::Identifier(identifier)
    }
}

impl From<Literal> for Expression {
    fn from(literal: Literal) -> Self {
        Expression::Literal(literal)
    }
}

impl From<Match> for Expression {
    fn from(match_expression: Match) -> Self {
        Expression::Match(match_expression)
    }
}

impl From<Member> for Expression {
    fn from(member: Member) -> Self {
        Expression::Member(member)
    }
}

impl From<Return> for Expression {
    fn from(return_expression: Return) -> Self {
        Expression::Return(return_expression)
    }
}

impl From<VariableDeclaration> for Expression {
    fn from(variable_declaration: VariableDeclaration) -> Self {
        Expression::VariableDeclaration(variable_declaration)
    }
}
