use crate::{
    ast_builder::match_rule_to_expression_builder,
    rule_parser::{OPERATION_PRATT_PARSER, Rule},
};
use ast::{Dyadic, DyadicOperator, Expression};
use pest::iterators::Pair;
use thiserror::Error;

/// Converts the pest rules of a parsed operation expression into an AST
/// representation.
///
/// # Examples
///
/// ```pest
/// - operation
///   - operand > literal > ...
///   - addition: "+"
///   - operand > identifier > ...
/// ```
pub fn build_operation_expression(
    pair: Pair<Rule>,
) -> Result<Dyadic, BuildOperationExpressionError> {
    use BuildOperationExpressionError::*;
    use Rule::{operand, operation};

    let rule = pair.as_rule();

    if rule != operation {
        return Err(RuleIsNotAnOperation(rule));
    };

    // Flatten `operator` wrappers into concrete operator tokens so Pratt can
    // apply precedence by rule.

    let expression = OPERATION_PRATT_PARSER
        .map_primary(|primary| match primary.as_rule() {
            operand => {
                let inner_operand =
                    primary.into_inner().next().ok_or(EmptyOperand)?;

                match_rule_to_expression_builder(inner_operand)
                    .map_err(|e| BuildOperandExpressionError(e.to_string()))
            }
            unknown_rule => Err(UnexpectedRuleInsteadOfOperand(unknown_rule)),
        })
        .map_infix(|lhs, op, rhs| {
            let lhs = lhs?;
            let rhs = rhs?;

            let dyadic_operator = build_dyadic_operator(op)
                .map_err(|e| BuildDyadicOperatorError(e.to_string()))?;

            Ok((Dyadic::new(dyadic_operator, lhs, rhs)).into())
        })
        .parse(pair.into_inner());

    match expression? {
        Expression::Dyadic(dyadic) => Ok(dyadic),
        _ => Err(PrattParserDidNotReturnDyadic),
    }
}

#[derive(Debug, PartialEq, Error)]
#[non_exhaustive]
pub enum BuildOperationExpressionError {
    /// The first rule is not an operation.
    #[error("Expected an operation expression, but found rule: {0:?}")]
    RuleIsNotAnOperation(Rule),

    /// The operand rule did not contain an inner expression.
    #[error("The operand is empty and does not contain an expression.")]
    EmptyOperand,

    /// An error occurred while building an inner expression.
    #[error("An error occurred while building an inner expression: {0}")]
    BuildOperandExpressionError(String),

    /// An unexpected rule was found instead of an operand.
    #[error("Expected an operand, but found rule: {0:?}")]
    UnexpectedRuleInsteadOfOperand(Rule),

    /// An error occurred while building a dyadic operator.
    #[error("An error occurred while building a dyadic operator: {0}")]
    BuildDyadicOperatorError(String),

    /// The Pratt parser completed, but did not produce a dyadic expression.
    #[error("Expected Pratt parser to return a dyadic expression.")]
    PrattParserDidNotReturnDyadic,
}

fn build_dyadic_operator(
    pair: Pair<Rule>,
) -> Result<DyadicOperator, BuildDyadicOperatorError> {
    use BuildDyadicOperatorError::*;
    use DyadicOperator::*;
    use Rule::{
        addition, division, equals, exponent, greater_than,
        greater_than_or_equals, less_than, less_than_or_equals, logic_and,
        logic_or, modulo, multiplication, not_equals, range, range_inclusive,
        subtraction,
    };

    Ok(match pair.as_rule() {
        addition => Add,
        subtraction => Subtract,
        multiplication => Multiply,
        division => Divide,
        exponent => Power,
        modulo => Modulo,
        greater_than_or_equals => GreaterThanOrEqual,
        greater_than => GreaterThan,
        less_than_or_equals => LessThanOrEqual,
        less_than => LessThan,
        equals => Equal,
        not_equals => NotEqual,
        logic_and => And,
        logic_or => Or,
        range_inclusive => RangeInclusive,
        range => Range,
        //
        x => {
            return Err(RuleIsNotAnOperator(x));
        }
    })
}

#[derive(Debug, PartialEq, Error)]
#[non_exhaustive]
pub enum BuildDyadicOperatorError {
    /// The rule is not an operator.
    #[error("Expected an operator, but found rule: {0:?}")]
    RuleIsNotAnOperator(Rule),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rule_parser::parse_string_to_rule;
    use ast::{Expression, Identifier};

    #[test]
    fn should_build_operation_when_given_simple_addition() {
        // Arrange
        let input = "a + b";

        let operation_rule = parse_string_to_rule(input, Rule::operation)
            .expect("Expected input to be parsed to rules successfully.")
            .next()
            .expect("Expected input to contain an operation.");

        // Act
        let operation = build_operation_expression(operation_rule);

        // Assert
        let expected = Dyadic::new(
            DyadicOperator::Add,
            Identifier::new("a".to_string()),
            Identifier::new("b".to_string()),
        );

        assert_eq!(operation, Ok(expected));
    }

    #[test]
    fn should_respect_precedence_when_given_addition_and_multiplication() {
        // Arrange
        let input = "a + b * c";

        let operation_rule = parse_string_to_rule(input, Rule::operation)
            .expect("Expected input to be parsed to rules successfully.")
            .next()
            .expect("Expected input to contain an operation.");

        // Act
        let operation = build_operation_expression(operation_rule);

        // Assert
        let expected = Dyadic::new(
            DyadicOperator::Add,
            Identifier::new("a".to_string()),
            Expression::Dyadic(Dyadic::new(
                DyadicOperator::Multiply,
                Identifier::new("b".to_string()),
                Identifier::new("c".to_string()),
            )),
        );

        assert_eq!(operation, Ok(expected));
    }

    #[test]
    fn should_return_error_when_rule_is_not_operation() {
        // Arrange
        let input = "identifier";

        let identifier_rule = parse_string_to_rule(input, Rule::identifier)
            .expect("Expected input to be parsed to rules successfully.")
            .next()
            .expect("Expected input to contain an identifier.");

        // Act
        let result = build_operation_expression(identifier_rule);

        // Assert
        assert!(matches!(
            result,
            Err(BuildOperationExpressionError::RuleIsNotAnOperation(_))
        ));
    }
}
