use crate::{
    ArrayLiteral, Assignment, BinaryLiteral, Block, BooleanLiteral, Break,
    Continue, DecimalLiteral, Dyadic, FunctionCall, FunctionDeclaration,
    HexadecimalLiteral, Identifier, IntegerLiteral, Literal, Loop, Match,
    Member, ObjectLiteral, OctalLiteral, Pipe, Return, StringLiteral,
    TemplateLiteral, Then, VariableDeclaration,
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
    Break(Break),
    Continue(Continue),
    Loop(Loop),
    VariableDeclaration(VariableDeclaration),
}

impl From<Pipe> for Expression {
    fn from(pipe: Pipe) -> Self {
        Expression::Pipe(pipe)
    }
}

impl From<Break> for Expression {
    fn from(break_expression: Break) -> Self {
        Expression::Break(break_expression)
    }
}

impl From<Continue> for Expression {
    fn from(continue_expression: Continue) -> Self {
        Expression::Continue(continue_expression)
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

impl From<BooleanLiteral> for Expression {
    fn from(boolean_literal: BooleanLiteral) -> Expression {
        Literal::Boolean(boolean_literal).into()
    }
}

impl From<IntegerLiteral> for Expression {
    fn from(integer_literal: IntegerLiteral) -> Expression {
        Literal::Integer(integer_literal).into()
    }
}

impl From<TemplateLiteral> for Expression {
    fn from(template_literal: TemplateLiteral) -> Expression {
        Literal::Template(template_literal).into()
    }
}

impl From<DecimalLiteral> for Expression {
    fn from(decimal_literal: DecimalLiteral) -> Expression {
        Literal::Decimal(decimal_literal).into()
    }
}

impl From<HexadecimalLiteral> for Expression {
    fn from(hexadecimal_literal: HexadecimalLiteral) -> Expression {
        Literal::Hexadecimal(hexadecimal_literal).into()
    }
}

impl From<BinaryLiteral> for Expression {
    fn from(binary_literal: BinaryLiteral) -> Expression {
        Literal::Binary(binary_literal).into()
    }
}

impl From<OctalLiteral> for Expression {
    fn from(octal_literal: OctalLiteral) -> Expression {
        Literal::Octal(octal_literal).into()
    }
}

impl From<ArrayLiteral> for Expression {
    fn from(array_literal: ArrayLiteral) -> Expression {
        Literal::Array(array_literal).into()
    }
}

impl From<ObjectLiteral> for Expression {
    fn from(object_literal: ObjectLiteral) -> Expression {
        Literal::Object(object_literal).into()
    }
}

impl From<StringLiteral> for Expression {
    fn from(string_literal: StringLiteral) -> Self {
        Literal::String(string_literal).into()
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

impl From<Loop> for Expression {
    fn from(loop_expression: Loop) -> Self {
        Expression::Loop(loop_expression)
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
