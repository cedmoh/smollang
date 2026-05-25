use std::fmt;

use ast::{
    ArrayLiteral, ArrayPatternElement, Assignment, BinaryLiteral, Block,
    BooleanLiteral, DecimalLiteral, DestructuringPatternElement, Directive,
    Directives, Dyadic, DyadicOperator, Expression, Expressions, FunctionBody,
    FunctionCall, FunctionCallArguments, FunctionDeclaration,
    FunctionParameter, FunctionParameters, HexadecimalLiteral, Identifier,
    IdentifierPattern, IntegerLiteral, Literal, LiteralPattern, Match,
    MatchArm, Member, ObjectLiteral, ObjectProperty, OctalLiteral, Pattern,
    Pipe, PipeArm, PipeArms, Program, Return, StringLiteral, TemplateLiteral,
    Then, Use, VariableDeclaration,
};

use crate::pretty_print::{
    INDENT_SIZE as PUSH, PrettyPrint, write_empty, write_field_label,
    write_node_label, write_none, write_scalar_field,
};

impl PrettyPrint for VariableDeclaration {
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter<'_>,
        indent: usize,
        colors_enabled: bool,
    ) -> fmt::Result {
        write_node_label(f, indent, colors_enabled, "VariableDeclaration")?;
        write_field_label(f, indent, colors_enabled, "name")?;
        self.name
            .fmt_with_indent(f, indent + PUSH, colors_enabled)?;
        write_scalar_field(
            f,
            indent,
            colors_enabled,
            "is_mutable",
            self.is_mutable,
        )?;
        write_field_label(f, indent, colors_enabled, "initial_value")?;

        match &self.initial_value {
            Some(initial_value) => {
                initial_value.fmt_with_indent(f, indent + PUSH, colors_enabled)
            }
            None => write_none(f, indent + PUSH, colors_enabled),
        }
    }
}

impl PrettyPrint for Assignment {
    fn fmt_with_indent(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        indent: usize,
        colors_enabled: bool,
    ) -> std::fmt::Result {
        write_node_label(f, indent, colors_enabled, "Assignment")?;
        write_field_label(f, indent, colors_enabled, "left")?;
        self.left
            .fmt_with_indent(f, indent + PUSH, colors_enabled)?;
        write_field_label(f, indent, colors_enabled, "right")?;
        self.right.fmt_with_indent(f, indent + PUSH, colors_enabled)
    }
}

impl PrettyPrint for Block {
    /// # Examples
    ///
    /// ```pest
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
        colors_enabled: bool,
    ) -> std::fmt::Result {
        write_node_label(f, indent, colors_enabled, "Block")?;
        write_field_label(f, indent, colors_enabled, "body")?;
        self.body.fmt_with_indent(f, indent + PUSH, colors_enabled)
    }
}

impl PrettyPrint for DyadicOperator {
    fn fmt_with_indent(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        indent: usize,
        colors_enabled: bool,
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
            DyadicOperator::Dot => "Dot",
            DyadicOperator::RangeInclusive => "RangeInclusive",
            DyadicOperator::Range => "Range",
        };

        write_node_label(f, indent, colors_enabled, label)
    }
}

impl PrettyPrint for Dyadic {
    fn fmt_with_indent(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        indent: usize,
        colors_enabled: bool,
    ) -> std::fmt::Result {
        write_node_label(f, indent, colors_enabled, "Dyadic")?;
        write_field_label(f, indent, colors_enabled, "operator")?;
        self.operator
            .fmt_with_indent(f, indent + PUSH, colors_enabled)?;
        write_field_label(f, indent, colors_enabled, "left")?;
        self.left
            .fmt_with_indent(f, indent + PUSH, colors_enabled)?;
        write_field_label(f, indent, colors_enabled, "right")?;
        self.right.fmt_with_indent(f, indent + PUSH, colors_enabled)
    }
}

impl PrettyPrint for Expression {
    fn fmt_with_indent(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        indent: usize,
        colors_enabled: bool,
    ) -> std::fmt::Result {
        match self {
            Expression::Assignment(assignment) => {
                assignment.fmt_with_indent(f, indent, colors_enabled)
            }
            Expression::Block(block) => {
                block.fmt_with_indent(f, indent, colors_enabled)
            }
            Expression::Dyadic(dyadic) => {
                dyadic.fmt_with_indent(f, indent, colors_enabled)
            }
            Expression::FunctionCall(function_call) => {
                function_call.fmt_with_indent(f, indent, colors_enabled)
            }
            Expression::FunctionDeclaration(function_declaration) => {
                function_declaration.fmt_with_indent(f, indent, colors_enabled)
            }
            Expression::Then(then_expression) => {
                then_expression.fmt_with_indent(f, indent, colors_enabled)
            }
            Expression::Pipe(pipe) => {
                pipe.fmt_with_indent(f, indent, colors_enabled)
            }
            Expression::Identifier(identifier) => {
                identifier.fmt_with_indent(f, indent, colors_enabled)
            }
            Expression::Literal(literal) => {
                literal.fmt_with_indent(f, indent, colors_enabled)
            }
            Expression::Match(match_expression) => {
                match_expression.fmt_with_indent(f, indent, colors_enabled)
            }
            Expression::Member(member) => {
                member.fmt_with_indent(f, indent, colors_enabled)
            }
            Expression::Return(return_expression) => {
                return_expression.fmt_with_indent(f, indent, colors_enabled)
            }
            Expression::VariableDeclaration(variable_declaration) => {
                variable_declaration.fmt_with_indent(f, indent, colors_enabled)
            }
        }
    }
}

impl PrettyPrint for Expressions {
    fn fmt_with_indent(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        indent: usize,
        colors_enabled: bool,
    ) -> std::fmt::Result {
        if self.items.is_empty() {
            return write_empty(f, indent, colors_enabled);
        }

        for expr in &self.items {
            expr.fmt_with_indent(f, indent, colors_enabled)?;
        }

        Ok(())
    }
}

impl PrettyPrint for FunctionCall {
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter<'_>,
        indent: usize,
        colors_enabled: bool,
    ) -> fmt::Result {
        write_node_label(f, indent, colors_enabled, "FunctionCall")?;
        write_field_label(f, indent, colors_enabled, "callee")?;
        self.callee
            .fmt_with_indent(f, indent + PUSH, colors_enabled)?;
        write_field_label(f, indent, colors_enabled, "arguments")?;
        self.arguments
            .fmt_with_indent(f, indent + PUSH, colors_enabled)
    }
}

impl PrettyPrint for FunctionCallArguments {
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter<'_>,
        indent: usize,
        colors_enabled: bool,
    ) -> fmt::Result {
        self.expressions.fmt_with_indent(f, indent, colors_enabled)
    }
}

impl PrettyPrint for FunctionDeclaration {
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter<'_>,
        indent: usize,
        colors_enabled: bool,
    ) -> fmt::Result {
        write_node_label(f, indent, colors_enabled, "FunctionDeclaration")?;

        write_field_label(f, indent, colors_enabled, "name")?;
        match &self.name {
            Some(name) => {
                name.fmt_with_indent(f, indent + PUSH, colors_enabled)?
            }
            None => {
                write_none(f, indent + PUSH, colors_enabled)?;
            }
        }

        write_field_label(f, indent, colors_enabled, "params")?;
        self.params
            .fmt_with_indent(f, indent + PUSH, colors_enabled)?;

        write_field_label(f, indent, colors_enabled, "body")?;
        match &self.body {
            Some(body) => {
                body.fmt_with_indent(f, indent + PUSH, colors_enabled)
            }
            None => write_none(f, indent + PUSH, colors_enabled),
        }
    }
}

impl PrettyPrint for FunctionParameter {
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter<'_>,
        indent: usize,
        colors_enabled: bool,
    ) -> fmt::Result {
        write_node_label(f, indent, colors_enabled, "FunctionParameter")?;
        write_field_label(f, indent, colors_enabled, "name")?;
        self.name.fmt_with_indent(f, indent + PUSH, colors_enabled)
    }
}

impl PrettyPrint for FunctionParameters {
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter<'_>,
        indent: usize,
        colors_enabled: bool,
    ) -> fmt::Result {
        if self.items.is_empty() {
            return write_empty(f, indent, colors_enabled);
        }

        for parameter in &self.items {
            parameter.fmt_with_indent(f, indent, colors_enabled)?;
        }

        Ok(())
    }
}

impl PrettyPrint for FunctionBody {
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter<'_>,
        indent: usize,
        colors_enabled: bool,
    ) -> fmt::Result {
        match &self.body {
            Some(body) => body.fmt_with_indent(f, indent, colors_enabled),
            None => write_none(f, indent, colors_enabled),
        }
    }
}

impl PrettyPrint for Return {
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter<'_>,
        indent: usize,
        colors_enabled: bool,
    ) -> fmt::Result {
        write_node_label(f, indent, colors_enabled, "Return")?;
        write_field_label(f, indent, colors_enabled, "expression")?;

        match &self.expression {
            Some(expression) => {
                expression.fmt_with_indent(f, indent + PUSH, colors_enabled)
            }
            None => write_none(f, indent + PUSH, colors_enabled),
        }
    }
}

impl PrettyPrint for Identifier {
    /// # Examples
    ///
    /// ```pest
    /// Identifier
    /// id: 'message'
    /// ```
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter<'_>,
        indent: usize,
        colors_enabled: bool,
    ) -> fmt::Result {
        write_node_label(f, indent, colors_enabled, "Identifier")?;
        write_scalar_field(
            f,
            indent,
            colors_enabled,
            "id",
            format!("'{}'", self.id),
        )
    }
}

impl PrettyPrint for Program {
    /// # Examples
    ///
    /// ```pest
    /// Program
    /// directives:
    ///   Directive > ...
    ///   Directive > ...
    /// body:
    ///   Expression > ...
    ///   Expression > ...
    ///   ...
    /// ```
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter<'_>,
        indent: usize,
        colors_enabled: bool,
    ) -> fmt::Result {
        write_node_label(f, indent, colors_enabled, "Program")?;

        write_field_label(f, indent, colors_enabled, "directives")?;
        self.directives
            .fmt_with_indent(f, indent + PUSH, colors_enabled)?;

        write_field_label(f, indent, colors_enabled, "body")?;
        self.body.fmt_with_indent(f, indent + PUSH, colors_enabled)
    }
}

impl PrettyPrint for Then {
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter<'_>,
        indent: usize,
        colors_enabled: bool,
    ) -> fmt::Result {
        write_node_label(f, indent, colors_enabled, "Then")?;
        write_field_label(f, indent, colors_enabled, "condition")?;
        self.condition
            .fmt_with_indent(f, indent + PUSH, colors_enabled)?;
        write_field_label(f, indent, colors_enabled, "then_body")?;
        self.then_body
            .fmt_with_indent(f, indent + PUSH, colors_enabled)?;
        write_field_label(f, indent, colors_enabled, "else_body")?;

        match &self.else_body {
            Some(else_body) => {
                else_body.fmt_with_indent(f, indent + PUSH, colors_enabled)
            }
            None => write_none(f, indent + PUSH, colors_enabled),
        }
    }
}

impl PrettyPrint for Pipe {
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter<'_>,
        indent: usize,
        colors_enabled: bool,
    ) -> fmt::Result {
        write_node_label(f, indent, colors_enabled, "Pipe")?;
        write_field_label(f, indent, colors_enabled, "arms")?;
        self.arms.fmt_with_indent(f, indent + PUSH, colors_enabled)
    }
}

impl PrettyPrint for PipeArm {
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter<'_>,
        indent: usize,
        colors_enabled: bool,
    ) -> fmt::Result {
        self.expression.fmt_with_indent(f, indent, colors_enabled)
    }
}

impl PrettyPrint for PipeArms {
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter<'_>,
        indent: usize,
        colors_enabled: bool,
    ) -> fmt::Result {
        for arm in &self.arms {
            arm.fmt_with_indent(f, indent, colors_enabled)?;
        }

        Ok(())
    }
}

impl PrettyPrint for Pattern {
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter<'_>,
        indent: usize,
        colors_enabled: bool,
    ) -> fmt::Result {
        match self {
            Pattern::Identifier(identifier) => {
                identifier.fmt_with_indent(f, indent, colors_enabled)
            }
            Pattern::Literal(literal) => {
                literal.fmt_with_indent(f, indent, colors_enabled)
            }
            Pattern::Array(array_pattern_elements) => {
                write_node_label(f, indent, colors_enabled, "ArrayPattern")?;
                for element in array_pattern_elements {
                    element.fmt_with_indent(
                        f,
                        indent + PUSH,
                        colors_enabled,
                    )?;
                }
                Ok(())
            }
            Pattern::Destructuring(destructuring_pattern_elements) => {
                write_node_label(
                    f,
                    indent,
                    colors_enabled,
                    "DestructuringPattern",
                )?;
                for element in destructuring_pattern_elements {
                    element.fmt_with_indent(
                        f,
                        indent + PUSH,
                        colors_enabled,
                    )?;
                }
                Ok(())
            }
            Pattern::Wildcard => {
                write_node_label(f, indent, colors_enabled, "Wildcard")
            }
        }
    }
}

impl PrettyPrint for ArrayPatternElement {
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter<'_>,
        indent: usize,
        colors_enabled: bool,
    ) -> fmt::Result {
        match self {
            ArrayPatternElement::Pattern(pattern) => {
                pattern.fmt_with_indent(f, indent, colors_enabled)
            }
            ArrayPatternElement::Rest => {
                write_node_label(f, indent, colors_enabled, "Rest")
            }
        }
    }
}

impl PrettyPrint for DestructuringPatternElement {
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter<'_>,
        indent: usize,
        colors_enabled: bool,
    ) -> fmt::Result {
        write_node_label(
            f,
            indent,
            colors_enabled,
            "DestructuringPatternElement",
        )?;
        write_field_label(f, indent, colors_enabled, "name")?;
        self.name
            .fmt_with_indent(f, indent + PUSH, colors_enabled)?;
        write_field_label(f, indent, colors_enabled, "pattern")?;
        if let Some(pattern) = &self.pattern {
            pattern.fmt_with_indent(f, indent + PUSH, colors_enabled)
        } else {
            write_none(f, indent + PUSH, colors_enabled)
        }
    }
}

impl PrettyPrint for IdentifierPattern {
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter<'_>,
        indent: usize,
        colors_enabled: bool,
    ) -> fmt::Result {
        write_node_label(f, indent, colors_enabled, "IdentifierPattern")?;
        write_scalar_field(
            f,
            indent,
            colors_enabled,
            "name",
            format!("'{}'", self.0.id),
        )
    }
}

impl PrettyPrint for LiteralPattern {
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter<'_>,
        indent: usize,
        colors_enabled: bool,
    ) -> fmt::Result {
        match self {
            LiteralPattern::Nil => {
                write_node_label(f, indent, colors_enabled, "Nil")
            }
            LiteralPattern::Boolean(boolean_literal) => {
                boolean_literal.fmt_with_indent(f, indent, colors_enabled)
            }
            LiteralPattern::String(string_literal) => {
                string_literal.fmt_with_indent(f, indent, colors_enabled)
            }
            LiteralPattern::Template(template_literal) => {
                template_literal.fmt_with_indent(f, indent, colors_enabled)
            }
            LiteralPattern::Integer(integer_literal) => {
                integer_literal.fmt_with_indent(f, indent, colors_enabled)
            }
            LiteralPattern::Decimal(decimal_literal) => {
                decimal_literal.fmt_with_indent(f, indent, colors_enabled)
            }
            LiteralPattern::Hexadecimal(hexadecimal_literal) => {
                hexadecimal_literal.fmt_with_indent(f, indent, colors_enabled)
            }
            LiteralPattern::Binary(binary_literal) => {
                binary_literal.fmt_with_indent(f, indent, colors_enabled)
            }
            LiteralPattern::Octal(octal_literal) => {
                octal_literal.fmt_with_indent(f, indent, colors_enabled)
            }
        }
    }
}

impl PrettyPrint for Member {
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter<'_>,
        indent: usize,
        colors_enabled: bool,
    ) -> fmt::Result {
        write_node_label(f, indent, colors_enabled, "Member")?;
        write_field_label(f, indent, colors_enabled, "object")?;
        self.object
            .fmt_with_indent(f, indent + PUSH, colors_enabled)?;
        write_field_label(f, indent, colors_enabled, "property")?;
        self.property
            .fmt_with_indent(f, indent + PUSH, colors_enabled)
    }
}

impl PrettyPrint for Match {
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter<'_>,
        indent: usize,
        colors_enabled: bool,
    ) -> fmt::Result {
        write_node_label(f, indent, colors_enabled, "Match")?;
        write_field_label(f, indent, colors_enabled, "expression")?;
        self.expression
            .fmt_with_indent(f, indent + PUSH, colors_enabled)?;
        write_field_label(f, indent, colors_enabled, "branches")?;
        self.branches
            .fmt_with_indent(f, indent + PUSH, colors_enabled)
    }
}

impl PrettyPrint for MatchArm {
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter<'_>,
        indent: usize,
        colors_enabled: bool,
    ) -> fmt::Result {
        write_node_label(f, indent, colors_enabled, "MatchArm")?;
        write_field_label(f, indent, colors_enabled, "pattern")?;
        self.pattern
            .fmt_with_indent(f, indent + PUSH, colors_enabled)?;
        write_field_label(f, indent, colors_enabled, "body")?;
        self.body.fmt_with_indent(f, indent + PUSH, colors_enabled)
    }
}

impl PrettyPrint for Literal {
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter<'_>,
        indent: usize,
        colors_enabled: bool,
    ) -> fmt::Result {
        match self {
            Literal::Nil => write_node_label(f, indent, colors_enabled, "nil"),
            Literal::Boolean(boolean_literal) => {
                boolean_literal.fmt_with_indent(f, indent, colors_enabled)
            }
            Literal::String(string_literal) => {
                string_literal.fmt_with_indent(f, indent, colors_enabled)
            }
            Literal::Template(template_literal) => {
                template_literal.fmt_with_indent(f, indent, colors_enabled)
            }
            Literal::Integer(integer_literal) => {
                integer_literal.fmt_with_indent(f, indent, colors_enabled)
            }
            Literal::Decimal(decimal_literal) => {
                decimal_literal.fmt_with_indent(f, indent, colors_enabled)
            }
            Literal::Hexadecimal(hexadecimal_literal) => {
                hexadecimal_literal.fmt_with_indent(f, indent, colors_enabled)
            }
            Literal::Binary(binary_literal) => {
                binary_literal.fmt_with_indent(f, indent, colors_enabled)
            }
            Literal::Octal(octal_literal) => {
                octal_literal.fmt_with_indent(f, indent, colors_enabled)
            }
            Literal::Array(array_literal) => {
                array_literal.fmt_with_indent(f, indent, colors_enabled)
            }
            Literal::Object(object_literal) => {
                object_literal.fmt_with_indent(f, indent, colors_enabled)
            }
        }
    }
}

impl PrettyPrint for ArrayLiteral {
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter<'_>,
        indent: usize,
        colors_enabled: bool,
    ) -> fmt::Result {
        write_node_label(f, indent, colors_enabled, "ArrayLiteral")?;
        write_field_label(f, indent, colors_enabled, "elements")?;

        if self.elements.items.is_empty() {
            return write_empty(f, indent + PUSH, colors_enabled);
        }

        for element in &self.elements.items {
            element.fmt_with_indent(f, indent + PUSH, colors_enabled)?;
        }

        Ok(())
    }
}

impl PrettyPrint for ObjectLiteral {
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter<'_>,
        indent: usize,
        colors_enabled: bool,
    ) -> fmt::Result {
        write_node_label(f, indent, colors_enabled, "ObjectLiteral")?;
        write_field_label(f, indent, colors_enabled, "properties")?;

        if self.properties.properties.is_empty() {
            return write_empty(f, indent + PUSH, colors_enabled);
        }

        for property in &self.properties.properties {
            property.fmt_with_indent(f, indent + PUSH, colors_enabled)?;
        }

        Ok(())
    }
}

impl PrettyPrint for ObjectProperty {
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter<'_>,
        indent: usize,
        colors_enabled: bool,
    ) -> fmt::Result {
        write_node_label(f, indent, colors_enabled, "Property")?;

        match self {
            ObjectProperty::Shorthand(shorthand) => write_scalar_field(
                f,
                indent,
                colors_enabled,
                "key",
                format!("'{}'", shorthand),
            ),
            ObjectProperty::KeyValue(key, expression) => {
                write_field_label(f, indent, colors_enabled, "key")?;
                write_scalar_field(
                    f,
                    indent + PUSH,
                    colors_enabled,
                    "value",
                    format!("'{}'", key),
                )?;
                write_field_label(f, indent, colors_enabled, "value")?;
                expression.fmt_with_indent(f, indent + PUSH, colors_enabled)
            }
        }
    }
}

impl PrettyPrint for BooleanLiteral {
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter<'_>,
        indent: usize,
        colors_enabled: bool,
    ) -> fmt::Result {
        write_node_label(f, indent, colors_enabled, "BooleanLiteral")?;
        write_scalar_field(f, indent, colors_enabled, "value", self.value)
    }
}

impl PrettyPrint for StringLiteral {
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter<'_>,
        indent: usize,
        colors_enabled: bool,
    ) -> fmt::Result {
        write_node_label(f, indent, colors_enabled, "StringLiteral")?;
        write_scalar_field(
            f,
            indent,
            colors_enabled,
            "value",
            format!("'{}'", self.value),
        )
    }
}

impl PrettyPrint for TemplateLiteral {
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter<'_>,
        indent: usize,
        colors_enabled: bool,
    ) -> fmt::Result {
        write_node_label(f, indent, colors_enabled, "TemplateLiteral")?;
        write_scalar_field(
            f,
            indent,
            colors_enabled,
            "value",
            format!("'{}'", self.value),
        )
    }
}

impl PrettyPrint for IntegerLiteral {
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter<'_>,
        indent: usize,
        colors_enabled: bool,
    ) -> fmt::Result {
        write_node_label(f, indent, colors_enabled, "IntegerLiteral")?;
        write_scalar_field(f, indent, colors_enabled, "value", self.value)
    }
}

impl PrettyPrint for DecimalLiteral {
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter<'_>,
        indent: usize,
        colors_enabled: bool,
    ) -> fmt::Result {
        write_node_label(f, indent, colors_enabled, "DecimalLiteral")?;
        write_scalar_field(f, indent, colors_enabled, "value", self.value)
    }
}

impl PrettyPrint for HexadecimalLiteral {
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter<'_>,
        indent: usize,
        colors_enabled: bool,
    ) -> fmt::Result {
        write_node_label(f, indent, colors_enabled, "HexadecimalLiteral")?;
        write_scalar_field(f, indent, colors_enabled, "value", self.value)
    }
}

impl PrettyPrint for BinaryLiteral {
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter<'_>,
        indent: usize,
        colors_enabled: bool,
    ) -> fmt::Result {
        write_node_label(f, indent, colors_enabled, "BinaryLiteral")?;
        write_scalar_field(f, indent, colors_enabled, "value", self.value)
    }
}

impl PrettyPrint for OctalLiteral {
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter<'_>,
        indent: usize,
        colors_enabled: bool,
    ) -> fmt::Result {
        write_node_label(f, indent, colors_enabled, "OctalLiteral")?;
        write_scalar_field(f, indent, colors_enabled, "value", self.value)
    }
}

impl PrettyPrint for Use {
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter<'_>,
        indent: usize,
        colors_enabled: bool,
    ) -> fmt::Result {
        write_node_label(f, indent, colors_enabled, "Use")?;
        write_scalar_field(
            f,
            indent,
            colors_enabled,
            "path",
            self.path.to_string(),
        )?;

        write_field_label(f, indent, colors_enabled, "imports")?;
        for import in &self.imports {
            import.fmt_with_indent(f, indent + PUSH, colors_enabled)?;
        }
        Ok(())
    }
}

impl PrettyPrint for Directive {
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter<'_>,
        indent: usize,
        colors_enabled: bool,
    ) -> fmt::Result {
        match self {
            Directive::Use(use_directive) => {
                use_directive.fmt_with_indent(f, indent, colors_enabled)
            }
        }
    }
}

impl PrettyPrint for Directives {
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter<'_>,
        indent: usize,
        colors_enabled: bool,
    ) -> fmt::Result {
        if self.items.is_empty() {
            return write_empty(f, indent, colors_enabled);
        }

        for directive in &self.items {
            directive.fmt_with_indent(f, indent, colors_enabled)?;
        }

        Ok(())
    }
}
