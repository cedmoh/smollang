use std::fmt;

use ast::{
    ArrayLiteral, ArrayPatternElement, Assignment, BinaryLiteral, Block,
    BooleanLiteral, Break, Continue, DecimalLiteral,
    DestructuringPatternElement, Directive, Directives, Dyadic, DyadicOperator,
    Expression, Expressions, FunctionBody, FunctionCall, FunctionCallArguments,
    FunctionDeclaration, FunctionParameter, FunctionParameters,
    HexadecimalLiteral, Identifier, IdentifierPattern, IntegerLiteral, Literal,
    LiteralPattern, Loop, Match, MatchArm, Member, ObjectLiteral,
    ObjectProperty, OctalLiteral, Pattern, Pipe, PipeArm, PipeArms, Program,
    Return, StringLiteral, TemplateLiteral, Then, Use, VariableDeclaration,
};

use crate::pretty_print::{
    INDENT, PrettyPrint, tree_child_prefix, write_empty, write_node_label,
    write_none, write_tree_bool_scalar_field, write_tree_field_label,
    write_tree_number_scalar_field, write_tree_scalar_field,
};

impl PrettyPrint for VariableDeclaration {
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter<'_>,
        prefix: &str,
        colors_enabled: bool,
    ) -> fmt::Result {
        write_node_label(f, prefix, colors_enabled, "VariableDeclaration")?;
        write_tree_field_label(f, prefix, colors_enabled, "name", false)?;
        self.name.fmt_with_indent(
            f,
            &tree_child_prefix(prefix, false, colors_enabled),
            colors_enabled,
        )?;
        write_tree_bool_scalar_field(
            f,
            prefix,
            colors_enabled,
            "is_mutable",
            self.is_mutable,
            false,
        )?;
        write_tree_field_label(
            f,
            prefix,
            colors_enabled,
            "initial_value",
            true,
        )?;

        match &self.initial_value {
            Some(initial_value) => initial_value.fmt_with_indent(
                f,
                &tree_child_prefix(prefix, true, colors_enabled),
                colors_enabled,
            ),
            None => write_none(
                f,
                &tree_child_prefix(prefix, true, colors_enabled),
                colors_enabled,
            ),
        }
    }
}

impl PrettyPrint for Assignment {
    fn fmt_with_indent(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        prefix: &str,
        colors_enabled: bool,
    ) -> std::fmt::Result {
        write_node_label(f, prefix, colors_enabled, "Assignment")?;
        write_tree_field_label(f, prefix, colors_enabled, "left", false)?;
        self.left.fmt_with_indent(
            f,
            &tree_child_prefix(prefix, false, colors_enabled),
            colors_enabled,
        )?;
        write_tree_field_label(f, prefix, colors_enabled, "right", true)?;
        self.right.fmt_with_indent(
            f,
            &tree_child_prefix(prefix, true, colors_enabled),
            colors_enabled,
        )
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
        prefix: &str,
        colors_enabled: bool,
    ) -> std::fmt::Result {
        write_node_label(f, prefix, colors_enabled, "Block")?;
        write_tree_field_label(f, prefix, colors_enabled, "body", true)?;
        self.body.fmt_with_indent(
            f,
            &tree_child_prefix(prefix, true, colors_enabled),
            colors_enabled,
        )
    }
}

impl PrettyPrint for DyadicOperator {
    fn fmt_with_indent(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        prefix: &str,
        colors_enabled: bool,
    ) -> std::fmt::Result {
        use DyadicOperator::*;

        let label = match self {
            Add => "Add",
            Subtract => "Subtract",
            Multiply => "Multiply",
            Divide => "Divide",
            Modulo => "Modulo",
            Power => "Power",
            Equal => "Equal",
            NotEqual => "NotEqual",
            LessThan => "LessThan",
            GreaterThan => "GreaterThan",
            LessThanOrEqual => "LessThanOrEqual",
            GreaterThanOrEqual => "GreaterThanOrEqual",
            And => "And",
            Or => "Or",
            RangeInclusive => "RangeInclusive",
            Range => "Range",
            AddAssign => "AddAssign",
            SubtractAssign => "SubtractAssign",
            MultiplyAssign => "MultiplyAssign",
            DivideAssign => "DivideAssign",
            ModuloAssign => "ModuloAssign",
            PowerAssign => "PowerAssign",
            AndAssign => "AndAssign",
            OrAssign => "OrAssign",
        };

        write_node_label(f, prefix, colors_enabled, label)
    }
}

impl PrettyPrint for Dyadic {
    fn fmt_with_indent(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        prefix: &str,
        colors_enabled: bool,
    ) -> std::fmt::Result {
        write_node_label(f, prefix, colors_enabled, "Dyadic")?;
        write_tree_field_label(f, prefix, colors_enabled, "operator", false)?;
        self.operator.fmt_with_indent(
            f,
            &tree_child_prefix(prefix, false, colors_enabled),
            colors_enabled,
        )?;
        write_tree_field_label(f, prefix, colors_enabled, "left", false)?;
        self.left.fmt_with_indent(
            f,
            &tree_child_prefix(prefix, false, colors_enabled),
            colors_enabled,
        )?;
        write_tree_field_label(f, prefix, colors_enabled, "right", true)?;
        self.right.fmt_with_indent(
            f,
            &tree_child_prefix(prefix, true, colors_enabled),
            colors_enabled,
        )
    }
}

impl PrettyPrint for Expression {
    fn fmt_with_indent(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        prefix: &str,
        colors_enabled: bool,
    ) -> std::fmt::Result {
        match self {
            Expression::Assignment(assignment) => {
                assignment.fmt_with_indent(f, prefix, colors_enabled)
            }
            Expression::Block(block) => {
                block.fmt_with_indent(f, prefix, colors_enabled)
            }
            Expression::Dyadic(dyadic) => {
                dyadic.fmt_with_indent(f, prefix, colors_enabled)
            }
            Expression::FunctionCall(function_call) => {
                function_call.fmt_with_indent(f, prefix, colors_enabled)
            }
            Expression::FunctionDeclaration(function_declaration) => {
                function_declaration.fmt_with_indent(f, prefix, colors_enabled)
            }
            Expression::Then(then_expression) => {
                then_expression.fmt_with_indent(f, prefix, colors_enabled)
            }
            Expression::Pipe(pipe) => {
                pipe.fmt_with_indent(f, prefix, colors_enabled)
            }
            Expression::Identifier(identifier) => {
                identifier.fmt_with_indent(f, prefix, colors_enabled)
            }
            Expression::Literal(literal) => {
                literal.fmt_with_indent(f, prefix, colors_enabled)
            }
            Expression::Match(match_expression) => {
                match_expression.fmt_with_indent(f, prefix, colors_enabled)
            }
            Expression::Member(member) => {
                member.fmt_with_indent(f, prefix, colors_enabled)
            }
            Expression::Return(return_expression) => {
                return_expression.fmt_with_indent(f, prefix, colors_enabled)
            }
            Expression::VariableDeclaration(variable_declaration) => {
                variable_declaration.fmt_with_indent(f, prefix, colors_enabled)
            }
            Expression::Loop(loop_expression) => {
                loop_expression.fmt_with_indent(f, prefix, colors_enabled)
            }
            Expression::Break(break_expression) => {
                break_expression.fmt_with_indent(f, prefix, colors_enabled)
            }
            Expression::Continue(continue_expression) => {
                continue_expression.fmt_with_indent(f, prefix, colors_enabled)
            }
        }
    }
}

impl PrettyPrint for Expressions {
    fn fmt_with_indent(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        prefix: &str,
        colors_enabled: bool,
    ) -> std::fmt::Result {
        if self.items.is_empty() {
            return write_empty(f, prefix, colors_enabled);
        }

        for expr in &self.items {
            expr.fmt_with_indent(f, prefix, colors_enabled)?;
        }

        Ok(())
    }
}

impl PrettyPrint for FunctionCall {
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter<'_>,
        prefix: &str,
        colors_enabled: bool,
    ) -> fmt::Result {
        write_node_label(f, prefix, colors_enabled, "FunctionCall")?;
        write_tree_field_label(f, prefix, colors_enabled, "callee", false)?;
        self.callee.fmt_with_indent(
            f,
            &tree_child_prefix(prefix, false, colors_enabled),
            colors_enabled,
        )?;
        write_tree_field_label(f, prefix, colors_enabled, "arguments", true)?;
        self.arguments.fmt_with_indent(
            f,
            &tree_child_prefix(prefix, true, colors_enabled),
            colors_enabled,
        )
    }
}

impl PrettyPrint for FunctionCallArguments {
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter<'_>,
        prefix: &str,
        colors_enabled: bool,
    ) -> fmt::Result {
        self.expressions.fmt_with_indent(f, prefix, colors_enabled)
    }
}

impl PrettyPrint for FunctionDeclaration {
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter<'_>,
        prefix: &str,
        colors_enabled: bool,
    ) -> fmt::Result {
        write_node_label(f, prefix, colors_enabled, "FunctionDeclaration")?;

        write_tree_field_label(f, prefix, colors_enabled, "name", false)?;
        match &self.name {
            Some(name) => name.fmt_with_indent(
                f,
                &tree_child_prefix(prefix, false, colors_enabled),
                colors_enabled,
            )?,
            None => {
                write_none(
                    f,
                    &tree_child_prefix(prefix, false, colors_enabled),
                    colors_enabled,
                )?;
            }
        }

        write_tree_field_label(f, prefix, colors_enabled, "params", false)?;
        self.params.fmt_with_indent(
            f,
            &tree_child_prefix(prefix, false, colors_enabled),
            colors_enabled,
        )?;

        write_tree_field_label(f, prefix, colors_enabled, "body", true)?;
        match &self.body {
            Some(body) => body.fmt_with_indent(
                f,
                &tree_child_prefix(prefix, true, colors_enabled),
                colors_enabled,
            ),
            None => write_none(
                f,
                &tree_child_prefix(prefix, true, colors_enabled),
                colors_enabled,
            ),
        }
    }
}

impl PrettyPrint for FunctionParameter {
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter<'_>,
        prefix: &str,
        colors_enabled: bool,
    ) -> fmt::Result {
        write_node_label(f, prefix, colors_enabled, "FunctionParameter")?;
        write_tree_field_label(f, prefix, colors_enabled, "name", true)?;
        self.name.fmt_with_indent(
            f,
            &tree_child_prefix(prefix, true, colors_enabled),
            colors_enabled,
        )
    }
}

impl PrettyPrint for FunctionParameters {
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter<'_>,
        prefix: &str,
        colors_enabled: bool,
    ) -> fmt::Result {
        if self.items.is_empty() {
            return write_empty(f, prefix, colors_enabled);
        }

        for parameter in &self.items {
            parameter.fmt_with_indent(f, prefix, colors_enabled)?;
        }

        Ok(())
    }
}

impl PrettyPrint for FunctionBody {
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter<'_>,
        prefix: &str,
        colors_enabled: bool,
    ) -> fmt::Result {
        self.body.fmt_with_indent(f, prefix, colors_enabled)
    }
}

impl PrettyPrint for Return {
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter<'_>,
        prefix: &str,
        colors_enabled: bool,
    ) -> fmt::Result {
        write_node_label(f, prefix, colors_enabled, "Return")?;
        write_tree_field_label(f, prefix, colors_enabled, "expression", true)?;

        match &self.expression {
            Some(expression) => expression.fmt_with_indent(
                f,
                &tree_child_prefix(prefix, true, colors_enabled),
                colors_enabled,
            ),
            None => write_none(
                f,
                &tree_child_prefix(prefix, true, colors_enabled),
                colors_enabled,
            ),
        }
    }
}

impl PrettyPrint for Break {
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter<'_>,
        prefix: &str,
        colors_enabled: bool,
    ) -> fmt::Result {
        write_node_label(f, prefix, colors_enabled, "Break")?;
        write_tree_field_label(f, prefix, colors_enabled, "expression", true)?;

        match &self.expression {
            Some(expression) => expression.fmt_with_indent(
                f,
                &tree_child_prefix(prefix, true, colors_enabled),
                colors_enabled,
            ),
            None => write_none(
                f,
                &tree_child_prefix(prefix, true, colors_enabled),
                colors_enabled,
            ),
        }
    }
}

impl PrettyPrint for Continue {
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter<'_>,
        prefix: &str,
        colors_enabled: bool,
    ) -> fmt::Result {
        write_node_label(f, prefix, colors_enabled, "Continue")
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
        prefix: &str,
        colors_enabled: bool,
    ) -> fmt::Result {
        write_node_label(f, prefix, colors_enabled, "Identifier")?;
        write_tree_scalar_field(
            f,
            prefix,
            colors_enabled,
            "id",
            format!("'{}'", self.id),
            true,
        )
    }
}

impl PrettyPrint for Program {
    /// # Examples
    ///
    /// ```text
    /// Program
    /// ├── directives
    /// │   Use
    /// │   path: 'std'
    /// │   imports:
    /// │     Identifier
    /// │     id: 'foo'
    /// └── body
    ///     FunctionCall
    ///     ...
    /// ```
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter<'_>,
        prefix: &str,
        colors_enabled: bool,
    ) -> fmt::Result {
        write_node_label(f, prefix, colors_enabled, "Program")?;

        write_tree_field_label(f, prefix, colors_enabled, "directives", false)?;
        let dir_prefix = tree_child_prefix(prefix, false, colors_enabled);
        self.directives
            .fmt_with_indent(f, &dir_prefix, colors_enabled)?;

        write_tree_field_label(f, prefix, colors_enabled, "body", true)?;
        let body_prefix = tree_child_prefix(prefix, true, colors_enabled);
        self.body.fmt_with_indent(f, &body_prefix, colors_enabled)
    }
}

impl PrettyPrint for Then {
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter<'_>,
        prefix: &str,
        colors_enabled: bool,
    ) -> fmt::Result {
        write_node_label(f, prefix, colors_enabled, "Then")?;
        write_tree_field_label(f, prefix, colors_enabled, "condition", false)?;
        self.condition.fmt_with_indent(
            f,
            &tree_child_prefix(prefix, false, colors_enabled),
            colors_enabled,
        )?;
        write_tree_field_label(f, prefix, colors_enabled, "then_body", false)?;
        self.then_body.fmt_with_indent(
            f,
            &tree_child_prefix(prefix, false, colors_enabled),
            colors_enabled,
        )?;
        write_tree_field_label(f, prefix, colors_enabled, "else_body", true)?;

        match &self.else_body {
            Some(else_body) => else_body.fmt_with_indent(
                f,
                &tree_child_prefix(prefix, true, colors_enabled),
                colors_enabled,
            ),
            None => write_none(
                f,
                &tree_child_prefix(prefix, true, colors_enabled),
                colors_enabled,
            ),
        }
    }
}

impl PrettyPrint for Pipe {
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter<'_>,
        prefix: &str,
        colors_enabled: bool,
    ) -> fmt::Result {
        write_node_label(f, prefix, colors_enabled, "Pipe")?;
        write_tree_field_label(f, prefix, colors_enabled, "arms", true)?;
        self.arms.fmt_with_indent(
            f,
            &tree_child_prefix(prefix, true, colors_enabled),
            colors_enabled,
        )
    }
}

impl PrettyPrint for PipeArm {
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter<'_>,
        prefix: &str,
        colors_enabled: bool,
    ) -> fmt::Result {
        self.expression.fmt_with_indent(f, prefix, colors_enabled)
    }
}

impl PrettyPrint for PipeArms {
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter<'_>,
        prefix: &str,
        colors_enabled: bool,
    ) -> fmt::Result {
        for arm in &self.arms {
            arm.fmt_with_indent(f, prefix, colors_enabled)?;
        }

        Ok(())
    }
}

impl PrettyPrint for Pattern {
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter<'_>,
        prefix: &str,
        colors_enabled: bool,
    ) -> fmt::Result {
        match self {
            Pattern::Identifier(identifier) => {
                identifier.fmt_with_indent(f, prefix, colors_enabled)
            }
            Pattern::Literal(literal) => {
                literal.fmt_with_indent(f, prefix, colors_enabled)
            }
            Pattern::Array(array_pattern_elements) => {
                write_node_label(f, prefix, colors_enabled, "ArrayPattern")?;
                for element in array_pattern_elements {
                    element.fmt_with_indent(
                        f,
                        &format!("{prefix}{INDENT}"),
                        colors_enabled,
                    )?;
                }
                Ok(())
            }
            Pattern::Destructuring(destructuring_pattern_elements) => {
                write_node_label(
                    f,
                    prefix,
                    colors_enabled,
                    "DestructuringPattern",
                )?;
                for element in destructuring_pattern_elements {
                    element.fmt_with_indent(
                        f,
                        &format!("{prefix}{INDENT}"),
                        colors_enabled,
                    )?;
                }
                Ok(())
            }
            Pattern::Wildcard => {
                write_node_label(f, prefix, colors_enabled, "Wildcard")
            }
        }
    }
}

impl PrettyPrint for ArrayPatternElement {
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter<'_>,
        prefix: &str,
        colors_enabled: bool,
    ) -> fmt::Result {
        match self {
            ArrayPatternElement::Pattern(pattern) => {
                pattern.fmt_with_indent(f, prefix, colors_enabled)
            }
            ArrayPatternElement::Rest => {
                write_node_label(f, prefix, colors_enabled, "Rest")
            }
        }
    }
}

impl PrettyPrint for DestructuringPatternElement {
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter<'_>,
        prefix: &str,
        colors_enabled: bool,
    ) -> fmt::Result {
        write_node_label(
            f,
            prefix,
            colors_enabled,
            "DestructuringPatternElement",
        )?;
        write_tree_field_label(f, prefix, colors_enabled, "name", false)?;
        self.name.fmt_with_indent(
            f,
            &tree_child_prefix(prefix, false, colors_enabled),
            colors_enabled,
        )?;
        write_tree_field_label(f, prefix, colors_enabled, "pattern", true)?;
        if let Some(pattern) = &self.pattern {
            pattern.fmt_with_indent(
                f,
                &tree_child_prefix(prefix, true, colors_enabled),
                colors_enabled,
            )
        } else {
            write_none(
                f,
                &tree_child_prefix(prefix, true, colors_enabled),
                colors_enabled,
            )
        }
    }
}

impl PrettyPrint for IdentifierPattern {
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter<'_>,
        prefix: &str,
        colors_enabled: bool,
    ) -> fmt::Result {
        write_node_label(f, prefix, colors_enabled, "IdentifierPattern")?;
        write_tree_scalar_field(
            f,
            prefix,
            colors_enabled,
            "name",
            format!("'{}'", self.0.id),
            true,
        )
    }
}

impl PrettyPrint for LiteralPattern {
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter<'_>,
        prefix: &str,
        colors_enabled: bool,
    ) -> fmt::Result {
        match self {
            LiteralPattern::Nil => {
                write_node_label(f, prefix, colors_enabled, "Nil")
            }
            LiteralPattern::Boolean(boolean_literal) => {
                boolean_literal.fmt_with_indent(f, prefix, colors_enabled)
            }
            LiteralPattern::String(string_literal) => {
                string_literal.fmt_with_indent(f, prefix, colors_enabled)
            }
            LiteralPattern::Template(template_literal) => {
                template_literal.fmt_with_indent(f, prefix, colors_enabled)
            }
            LiteralPattern::Integer(integer_literal) => {
                integer_literal.fmt_with_indent(f, prefix, colors_enabled)
            }
            LiteralPattern::Decimal(decimal_literal) => {
                decimal_literal.fmt_with_indent(f, prefix, colors_enabled)
            }
            LiteralPattern::Hexadecimal(hexadecimal_literal) => {
                hexadecimal_literal.fmt_with_indent(f, prefix, colors_enabled)
            }
            LiteralPattern::Binary(binary_literal) => {
                binary_literal.fmt_with_indent(f, prefix, colors_enabled)
            }
            LiteralPattern::Octal(octal_literal) => {
                octal_literal.fmt_with_indent(f, prefix, colors_enabled)
            }
        }
    }
}

impl PrettyPrint for Member {
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter<'_>,
        prefix: &str,
        colors_enabled: bool,
    ) -> fmt::Result {
        write_node_label(f, prefix, colors_enabled, "Member")?;
        write_tree_field_label(f, prefix, colors_enabled, "object", false)?;
        self.object.fmt_with_indent(
            f,
            &tree_child_prefix(prefix, false, colors_enabled),
            colors_enabled,
        )?;
        write_tree_field_label(f, prefix, colors_enabled, "property", true)?;
        self.property.fmt_with_indent(
            f,
            &tree_child_prefix(prefix, true, colors_enabled),
            colors_enabled,
        )
    }
}

impl PrettyPrint for Match {
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter<'_>,
        prefix: &str,
        colors_enabled: bool,
    ) -> fmt::Result {
        write_node_label(f, prefix, colors_enabled, "Match")?;
        write_tree_field_label(f, prefix, colors_enabled, "expression", false)?;
        self.expression.fmt_with_indent(
            f,
            &tree_child_prefix(prefix, false, colors_enabled),
            colors_enabled,
        )?;
        write_tree_field_label(f, prefix, colors_enabled, "branches", true)?;
        self.branches.fmt_with_indent(
            f,
            &tree_child_prefix(prefix, true, colors_enabled),
            colors_enabled,
        )
    }
}

impl PrettyPrint for MatchArm {
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter<'_>,
        prefix: &str,
        colors_enabled: bool,
    ) -> fmt::Result {
        write_node_label(f, prefix, colors_enabled, "MatchArm")?;
        write_tree_field_label(f, prefix, colors_enabled, "pattern", false)?;
        self.pattern.fmt_with_indent(
            f,
            &tree_child_prefix(prefix, false, colors_enabled),
            colors_enabled,
        )?;
        write_tree_field_label(f, prefix, colors_enabled, "body", true)?;
        self.body.fmt_with_indent(
            f,
            &tree_child_prefix(prefix, true, colors_enabled),
            colors_enabled,
        )
    }
}

impl PrettyPrint for Literal {
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter<'_>,
        prefix: &str,
        colors_enabled: bool,
    ) -> fmt::Result {
        match self {
            Literal::Nil => write_node_label(f, prefix, colors_enabled, "nil"),
            Literal::Boolean(boolean_literal) => {
                boolean_literal.fmt_with_indent(f, prefix, colors_enabled)
            }
            Literal::String(string_literal) => {
                string_literal.fmt_with_indent(f, prefix, colors_enabled)
            }
            Literal::Template(template_literal) => {
                template_literal.fmt_with_indent(f, prefix, colors_enabled)
            }
            Literal::Integer(integer_literal) => {
                integer_literal.fmt_with_indent(f, prefix, colors_enabled)
            }
            Literal::Decimal(decimal_literal) => {
                decimal_literal.fmt_with_indent(f, prefix, colors_enabled)
            }
            Literal::Hexadecimal(hexadecimal_literal) => {
                hexadecimal_literal.fmt_with_indent(f, prefix, colors_enabled)
            }
            Literal::Binary(binary_literal) => {
                binary_literal.fmt_with_indent(f, prefix, colors_enabled)
            }
            Literal::Octal(octal_literal) => {
                octal_literal.fmt_with_indent(f, prefix, colors_enabled)
            }
            Literal::Array(array_literal) => {
                array_literal.fmt_with_indent(f, prefix, colors_enabled)
            }
            Literal::Object(object_literal) => {
                object_literal.fmt_with_indent(f, prefix, colors_enabled)
            }
        }
    }
}

impl PrettyPrint for ArrayLiteral {
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter<'_>,
        prefix: &str,
        colors_enabled: bool,
    ) -> fmt::Result {
        write_node_label(f, prefix, colors_enabled, "ArrayLiteral")?;
        write_tree_field_label(f, prefix, colors_enabled, "elements", true)?;

        let child_prefix = tree_child_prefix(prefix, true, colors_enabled);

        if self.elements.items.is_empty() {
            return write_empty(f, &child_prefix, colors_enabled);
        }

        for element in &self.elements.items {
            element.fmt_with_indent(f, &child_prefix, colors_enabled)?;
        }

        Ok(())
    }
}

impl PrettyPrint for ObjectLiteral {
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter<'_>,
        prefix: &str,
        colors_enabled: bool,
    ) -> fmt::Result {
        write_node_label(f, prefix, colors_enabled, "ObjectLiteral")?;
        write_tree_field_label(f, prefix, colors_enabled, "properties", true)?;

        let child_prefix = tree_child_prefix(prefix, true, colors_enabled);

        if self.properties.properties.is_empty() {
            return write_empty(f, &child_prefix, colors_enabled);
        }

        for property in &self.properties.properties {
            property.fmt_with_indent(f, &child_prefix, colors_enabled)?;
        }

        Ok(())
    }
}

impl PrettyPrint for ObjectProperty {
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter<'_>,
        prefix: &str,
        colors_enabled: bool,
    ) -> fmt::Result {
        write_node_label(f, prefix, colors_enabled, "Property")?;

        match self {
            ObjectProperty::Shorthand(shorthand) => write_tree_scalar_field(
                f,
                prefix,
                colors_enabled,
                "key",
                format!("'{}'", shorthand),
                true,
            ),
            ObjectProperty::KeyValue(key, expression) => {
                write_tree_scalar_field(
                    f,
                    prefix,
                    colors_enabled,
                    "key",
                    format!("'{}'", key),
                    false,
                )?;
                write_tree_field_label(
                    f,
                    prefix,
                    colors_enabled,
                    "value",
                    true,
                )?;
                expression.fmt_with_indent(
                    f,
                    &tree_child_prefix(prefix, true, colors_enabled),
                    colors_enabled,
                )
            }
        }
    }
}

impl PrettyPrint for BooleanLiteral {
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter<'_>,
        prefix: &str,
        colors_enabled: bool,
    ) -> fmt::Result {
        write_node_label(f, prefix, colors_enabled, "BooleanLiteral")?;
        write_tree_bool_scalar_field(
            f,
            prefix,
            colors_enabled,
            "value",
            self.value,
            true,
        )
    }
}

impl PrettyPrint for StringLiteral {
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter<'_>,
        prefix: &str,
        colors_enabled: bool,
    ) -> fmt::Result {
        write_node_label(f, prefix, colors_enabled, "StringLiteral")?;
        write_tree_scalar_field(
            f,
            prefix,
            colors_enabled,
            "value",
            format!("'{}'", self.value),
            true,
        )
    }
}

impl PrettyPrint for TemplateLiteral {
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter<'_>,
        prefix: &str,
        colors_enabled: bool,
    ) -> fmt::Result {
        write_node_label(f, prefix, colors_enabled, "TemplateLiteral")?;
        write_tree_scalar_field(
            f,
            prefix,
            colors_enabled,
            "value",
            format!("'{}'", self.value),
            true,
        )
    }
}

impl PrettyPrint for IntegerLiteral {
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter<'_>,
        prefix: &str,
        colors_enabled: bool,
    ) -> fmt::Result {
        write_node_label(f, prefix, colors_enabled, "IntegerLiteral")?;
        write_tree_number_scalar_field(
            f,
            prefix,
            colors_enabled,
            "value",
            self.value,
            true,
        )
    }
}

impl PrettyPrint for DecimalLiteral {
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter<'_>,
        prefix: &str,
        colors_enabled: bool,
    ) -> fmt::Result {
        write_node_label(f, prefix, colors_enabled, "DecimalLiteral")?;
        write_tree_number_scalar_field(
            f,
            prefix,
            colors_enabled,
            "value",
            self.value,
            true,
        )
    }
}

impl PrettyPrint for HexadecimalLiteral {
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter<'_>,
        prefix: &str,
        colors_enabled: bool,
    ) -> fmt::Result {
        write_node_label(f, prefix, colors_enabled, "HexadecimalLiteral")?;
        write_tree_number_scalar_field(
            f,
            prefix,
            colors_enabled,
            "value",
            self.value,
            true,
        )
    }
}

impl PrettyPrint for BinaryLiteral {
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter<'_>,
        prefix: &str,
        colors_enabled: bool,
    ) -> fmt::Result {
        write_node_label(f, prefix, colors_enabled, "BinaryLiteral")?;
        write_tree_number_scalar_field(
            f,
            prefix,
            colors_enabled,
            "value",
            self.value,
            true,
        )
    }
}

impl PrettyPrint for OctalLiteral {
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter<'_>,
        prefix: &str,
        colors_enabled: bool,
    ) -> fmt::Result {
        write_node_label(f, prefix, colors_enabled, "OctalLiteral")?;
        write_tree_number_scalar_field(
            f,
            prefix,
            colors_enabled,
            "value",
            self.value,
            true,
        )
    }
}

impl PrettyPrint for Use {
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter<'_>,
        prefix: &str,
        colors_enabled: bool,
    ) -> fmt::Result {
        write_node_label(f, prefix, colors_enabled, "Use")?;
        write_tree_scalar_field(
            f,
            prefix,
            colors_enabled,
            "path",
            self.path.to_string(),
            false,
        )?;

        write_tree_field_label(f, prefix, colors_enabled, "imports", true)?;
        let child_prefix = tree_child_prefix(prefix, true, colors_enabled);
        for import in &self.imports {
            import.fmt_with_indent(f, &child_prefix, colors_enabled)?;
        }
        Ok(())
    }
}

impl PrettyPrint for Directive {
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter<'_>,
        prefix: &str,
        colors_enabled: bool,
    ) -> fmt::Result {
        match self {
            Directive::Use(use_directive) => {
                use_directive.fmt_with_indent(f, prefix, colors_enabled)
            }
        }
    }
}

impl PrettyPrint for Directives {
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter<'_>,
        prefix: &str,
        colors_enabled: bool,
    ) -> fmt::Result {
        if self.items.is_empty() {
            return write_empty(f, prefix, colors_enabled);
        }

        for directive in &self.items {
            directive.fmt_with_indent(f, prefix, colors_enabled)?;
        }

        Ok(())
    }
}

impl PrettyPrint for Loop {
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter<'_>,
        prefix: &str,
        colors_enabled: bool,
    ) -> fmt::Result {
        write_node_label(f, prefix, colors_enabled, "Loop")?;
        write_tree_field_label(f, prefix, colors_enabled, "body", true)?;
        self.body.fmt_with_indent(
            f,
            &tree_child_prefix(prefix, true, colors_enabled),
            colors_enabled,
        )
    }
}
