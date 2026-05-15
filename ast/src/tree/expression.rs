use crate::PrettyPrint;

use crate::{
    Assignment, Block, Dyadic, FunctionCall, FunctionDeclaration, Identifier,
    Literal, Match, Member, Pipe, Return, Then, VariableDeclaration,
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
    Then(Then),
    Pipe(Pipe),
    Identifier(Identifier),
    Literal(Literal),
    Match(Match),
    Member(Member),
    Return(Return),
    VariableDeclaration(VariableDeclaration),
}

impl PrettyPrint for Expression {
    fn fmt_with_indent(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        indent: usize,
    ) -> std::fmt::Result {
        match self {
            Expression::Assignment(assignment) => {
                assignment.fmt_with_indent(f, indent)
            }
            Expression::Block(block) => block.fmt_with_indent(f, indent),
            Expression::Dyadic(dyadic) => dyadic.fmt_with_indent(f, indent),
            Expression::FunctionCall(function_call) => {
                function_call.fmt_with_indent(f, indent)
            }
            Expression::FunctionDeclaration(function_declaration) => {
                function_declaration.fmt_with_indent(f, indent)
            }
            Expression::Then(then_expression) => {
                then_expression.fmt_with_indent(f, indent)
            }
            Expression::Pipe(pipe) => pipe.fmt_with_indent(f, indent),
            Expression::Identifier(identifier) => {
                identifier.fmt_with_indent(f, indent)
            }
            Expression::Literal(literal) => literal.fmt_with_indent(f, indent),
            Expression::Match(match_expression) => {
                match_expression.fmt_with_indent(f, indent)
            }
            Expression::Member(member) => member.fmt_with_indent(f, indent),
            Expression::Return(return_expression) => {
                return_expression.fmt_with_indent(f, indent)
            }
            Expression::VariableDeclaration(variable_declaration) => {
                variable_declaration.fmt_with_indent(f, indent)
            }
        }
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

impl From<Then> for Expression {
    fn from(then_expression: Then) -> Self {
        Expression::Then(then_expression)
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

    pub fn add_expressions(
        &mut self,
        expressions: Vec<Expression>,
    ) -> &mut Self {
        self.items.extend(expressions);
        self
    }
}

impl PrettyPrint for Expressions {
    /// # Example
    /// ```
    /// # use ast::{Expression, Expressions, Identifier, PrettyPrint};
    /// # let expressions = Expressions::new(vec![Identifier::new("message".to_string()).into()]);
    /// # let rendered = expressions.pretty().to_string();
    /// # assert!(rendered.contains("- Identifier"));
    /// ```
    fn fmt_with_indent(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        indent: usize,
    ) -> std::fmt::Result {
        for expr in &self.items {
            expr.fmt_with_indent(f, indent)?;
        }

        Ok(())
    }
}
