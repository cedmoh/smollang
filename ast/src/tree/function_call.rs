use super::Expression;

#[derive(Debug, Clone, PartialEq)]
pub struct Call {
    pub callee: Box<Expression>,
    pub arguments: CallArguments,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct CallArguments {
    pub items: Vec<Expression>,
}
