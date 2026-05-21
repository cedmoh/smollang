use std::fmt;

use ast::{
    Assignment, BinaryLiteral, Block, BooleanLiteral, DecimalLiteral, Dyadic,
    DyadicOperator, Expression, Expressions, FunctionBody, FunctionCall,
    FunctionCallArguments, FunctionDeclaration, FunctionParameter,
    FunctionParameters, HexadecimalLiteral, Identifier, IntegerLiteral,
    Literal, Match, MatchArm, Member, OctalLiteral, Pattern, Pipe, PipeArm,
    PipeArms, Program, Return, StringLiteral, Then, VariableDeclaration,
};

use crate::pretty_print::{
    PrettyPrint, write_empty, write_field_label, write_node_label, write_none,
    write_scalar_field,
};

impl PrettyPrint for VariableDeclaration {
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter<'_>,
        indent: usize,
    ) -> fmt::Result {
        write_node_label(f, indent, "VariableDeclaration")?;
        write_field_label(f, indent, "name")?;
        self.name.fmt_with_indent(f, indent + 2)?;
        write_scalar_field(f, indent, "is_mutable", self.is_mutable)?;
        write_field_label(f, indent, "initial_value")?;

        match &self.initial_value {
            Some(initial_value) => initial_value.fmt_with_indent(f, indent + 2),
            None => write_none(f, indent + 2),
        }
    }
}

impl PrettyPrint for Assignment {
    fn fmt_with_indent(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        indent: usize,
    ) -> std::fmt::Result {
        write_node_label(f, indent, "Assignment")?;
        write_field_label(f, indent, "left")?;
        self.left.fmt_with_indent(f, indent + 2)?;
        write_field_label(f, indent, "right")?;
        self.right.fmt_with_indent(f, indent + 2)
    }
}

impl PrettyPrint for Block {
    /// # Examples
    ///
    /// ```text
    /// Block
    /// body:
    ///   Expression > ...
    ///   Expression > ...
    ///   ...
    /// ```
    fn fmt_with_indent(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        indent: usize,
    ) -> std::fmt::Result {
        write_node_label(f, indent, "Block")?;
        write_field_label(f, indent, "body")?;
        self.body.fmt_with_indent(f, indent + 2)
    }
}

impl PrettyPrint for DyadicOperator {
    fn fmt_with_indent(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        indent: usize,
    ) -> std::fmt::Result {
        let label = match self {
            DyadicOperator::Add => "Add",
            DyadicOperator::Subtract => "Subtract",
            DyadicOperator::Multiply => "Multiply",
            DyadicOperator::Divide => "Divide",
            DyadicOperator::Modulo => "Modulo",
            DyadicOperator::Power => "Power",
            DyadicOperator::Equal => "Equal",
            DyadicOperator::NotEqual => "NotEqual",
            DyadicOperator::LessThan => "LessThan",
            DyadicOperator::GreaterThan => "GreaterThan",
            DyadicOperator::LessThanOrEqual => "LessThanOrEqual",
            DyadicOperator::GreaterThanOrEqual => "GreaterThanOrEqual",
            DyadicOperator::And => "And",
            DyadicOperator::Or => "Or",
            DyadicOperator::RangeInclusive => "RangeInclusive",
            DyadicOperator::Range => "Range",
        };

        write_node_label(f, indent, label)
    }
}

impl PrettyPrint for Dyadic {
    fn fmt_with_indent(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        indent: usize,
    ) -> std::fmt::Result {
        write_node_label(f, indent, "Dyadic")?;
        write_field_label(f, indent, "operator")?;
        self.operator.fmt_with_indent(f, indent + 2)?;
        write_field_label(f, indent, "left")?;
        self.left.fmt_with_indent(f, indent + 2)?;
        write_field_label(f, indent, "right")?;
        self.right.fmt_with_indent(f, indent + 2)
    }
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

impl PrettyPrint for Expressions {
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

impl PrettyPrint for FunctionCall {
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter<'_>,
        indent: usize,
    ) -> fmt::Result {
        write_node_label(f, indent, "FunctionCall")?;
        write_field_label(f, indent, "callee")?;
        self.callee.fmt_with_indent(f, indent + 2)?;
        write_field_label(f, indent, "arguments")?;
        self.arguments.fmt_with_indent(f, indent + 2)
    }
}

impl PrettyPrint for FunctionCallArguments {
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter<'_>,
        indent: usize,
    ) -> fmt::Result {
        self.expressions.fmt_with_indent(f, indent)
    }
}

impl PrettyPrint for FunctionDeclaration {
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter<'_>,
        indent: usize,
    ) -> fmt::Result {
        write_node_label(f, indent, "FunctionDeclaration")?;

        write_field_label(f, indent, "name")?;
        match &self.name {
            Some(name) => name.fmt_with_indent(f, indent + 2)?,
            None => {
                write_none(f, indent + 2)?;
            }
        }

        write_field_label(f, indent, "params")?;
        self.params.fmt_with_indent(f, indent + 2)?;

        write_field_label(f, indent, "body")?;
        match &self.body {
            Some(body) => body.fmt_with_indent(f, indent + 2),
            None => write_none(f, indent + 2),
        }
    }
}

impl PrettyPrint for FunctionParameter {
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter<'_>,
        indent: usize,
    ) -> fmt::Result {
        write_node_label(f, indent, "FunctionParameter")?;
        write_field_label(f, indent, "name")?;
        self.name.fmt_with_indent(f, indent + 2)
    }
}

impl PrettyPrint for FunctionParameters {
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter<'_>,
        indent: usize,
    ) -> fmt::Result {
        if self.items.is_empty() {
            return write_empty(f, indent);
        }

        for parameter in &self.items {
            parameter.fmt_with_indent(f, indent)?;
        }

        Ok(())
    }
}

impl PrettyPrint for FunctionBody {
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter<'_>,
        indent: usize,
    ) -> fmt::Result {
        match &self.body {
            Some(body) => body.fmt_with_indent(f, indent),
            None => write_none(f, indent),
        }
    }
}

impl PrettyPrint for Return {
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter<'_>,
        indent: usize,
    ) -> fmt::Result {
        write_node_label(f, indent, "Return")?;
        write_field_label(f, indent + 2, "expression")?;

        match &self.expression {
            Some(expression) => expression.fmt_with_indent(f, indent + 4),
            None => write_none(f, indent + 4),
        }
    }
}

impl PrettyPrint for Identifier {
    /// # Examples
    ///
    /// ```text
    /// Identifier
    /// id: 'message'
    /// ```
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter<'_>,
        indent: usize,
    ) -> fmt::Result {
        write_node_label(f, indent, "Identifier")?;
        write_scalar_field(f, indent, "id", format!("'{}'", self.id))
    }
}

impl PrettyPrint for Program {
    /// # Examples
    ///
    /// ```pest
    /// Program
    /// body:
    ///   Expression > ...
    ///   Expression > ...
    ///   ...
    /// ```
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter<'_>,
        indent: usize,
    ) -> fmt::Result {
        write_node_label(f, indent, "Program")?;
        write_field_label(f, indent, "body")?;
        self.body.fmt_with_indent(f, indent + 2)
    }
}

impl PrettyPrint for Then {
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter<'_>,
        indent: usize,
    ) -> fmt::Result {
        write_node_label(f, indent, "Then")?;
        write_field_label(f, indent + 2, "condition")?;
        self.condition.fmt_with_indent(f, indent + 4)?;
        write_field_label(f, indent + 2, "then_body")?;
        self.then_body.fmt_with_indent(f, indent + 4)?;
        write_field_label(f, indent + 2, "else_body")?;

        match &self.else_body {
            Some(else_body) => else_body.fmt_with_indent(f, indent + 4),
            None => write_none(f, indent + 4),
        }
    }
}

impl PrettyPrint for Pipe {
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter<'_>,
        indent: usize,
    ) -> fmt::Result {
        write_node_label(f, indent, "Pipe")?;
        write_field_label(f, indent, "arms")?;
        self.arms.fmt_with_indent(f, indent + 2)
    }
}

impl PrettyPrint for PipeArm {
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter<'_>,
        indent: usize,
    ) -> fmt::Result {
        self.expression.fmt_with_indent(f, indent)
    }
}

impl PrettyPrint for PipeArms {
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter<'_>,
        indent: usize,
    ) -> fmt::Result {
        for arm in &self.arms {
            arm.fmt_with_indent(f, indent)?;
        }

        Ok(())
    }
}

impl PrettyPrint for Pattern {
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter<'_>,
        indent: usize,
    ) -> fmt::Result {
        write_node_label(f, indent, "Pattern")?;
        write_scalar_field(
            f,
            indent + 2,
            "content",
            format!("'{}'", self.content),
        )
    }
}

impl PrettyPrint for Member {
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter<'_>,
        indent: usize,
    ) -> fmt::Result {
        write_node_label(f, indent, "Member")?;
        write_field_label(f, indent + 2, "chain")?;
        self.chain.fmt_with_indent(f, indent + 4)
    }
}

impl PrettyPrint for Match {
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter<'_>,
        indent: usize,
    ) -> fmt::Result {
        write_node_label(f, indent, "Match")?;
        write_field_label(f, indent + 2, "expression")?;
        self.expression.fmt_with_indent(f, indent + 4)?;
        write_field_label(f, indent + 2, "branches")?;
        self.branches.fmt_with_indent(f, indent + 4)
    }
}

impl PrettyPrint for MatchArm {
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter<'_>,
        indent: usize,
    ) -> fmt::Result {
        write_node_label(f, indent, "MatchArm")?;
        write_field_label(f, indent + 2, "pattern")?;
        self.pattern.fmt_with_indent(f, indent + 4)?;
        write_field_label(f, indent + 2, "body")?;
        self.body.fmt_with_indent(f, indent + 4)
    }
}

impl PrettyPrint for Literal {
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter<'_>,
        indent: usize,
    ) -> fmt::Result {
        match self {
            Literal::Nil => write_node_label(f, indent, "nil"),
            Literal::Boolean(boolean_literal) => {
                boolean_literal.fmt_with_indent(f, indent)
            }
            Literal::String(string_literal) => {
                string_literal.fmt_with_indent(f, indent)
            }
            Literal::Integer(integer_literal) => {
                integer_literal.fmt_with_indent(f, indent)
            }
            Literal::Decimal(decimal_literal) => {
                decimal_literal.fmt_with_indent(f, indent)
            }
            Literal::Hexadecimal(hexadecimal_literal) => {
                hexadecimal_literal.fmt_with_indent(f, indent)
            }
            Literal::Binary(binary_literal) => {
                binary_literal.fmt_with_indent(f, indent)
            }
            Literal::Octal(octal_literal) => {
                octal_literal.fmt_with_indent(f, indent)
            }
        }
    }
}

impl PrettyPrint for BooleanLiteral {
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter<'_>,
        indent: usize,
    ) -> fmt::Result {
        write_node_label(f, indent, "BooleanLiteral")?;
        write_scalar_field(f, indent + 2, "value", self.value)
    }
}

impl PrettyPrint for StringLiteral {
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter<'_>,
        indent: usize,
    ) -> fmt::Result {
        write_node_label(f, indent, "StringLiteral")?;
        write_scalar_field(f, indent + 2, "value", format!("'{}'", self.value))
    }
}

impl PrettyPrint for IntegerLiteral {
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter<'_>,
        indent: usize,
    ) -> fmt::Result {
        write_node_label(f, indent, "IntegerLiteral")?;
        write_scalar_field(f, indent + 2, "value", self.value)
    }
}

impl PrettyPrint for DecimalLiteral {
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter<'_>,
        indent: usize,
    ) -> fmt::Result {
        write_node_label(f, indent, "DecimalLiteral")?;
        write_scalar_field(f, indent + 2, "value", self.value)
    }
}

impl PrettyPrint for HexadecimalLiteral {
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter<'_>,
        indent: usize,
    ) -> fmt::Result {
        write_node_label(f, indent, "HexadecimalLiteral")?;
        write_scalar_field(f, indent + 2, "value", self.value)
    }
}

impl PrettyPrint for BinaryLiteral {
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter<'_>,
        indent: usize,
    ) -> fmt::Result {
        write_node_label(f, indent, "BinaryLiteral")?;
        write_scalar_field(f, indent + 2, "value", self.value)
    }
}

impl PrettyPrint for OctalLiteral {
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter<'_>,
        indent: usize,
    ) -> fmt::Result {
        write_node_label(f, indent, "OctalLiteral")?;
        write_scalar_field(f, indent + 2, "value", self.value)
    }
}
