use ast::Span;
use bytecode::Instruction;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CompilationError {
    /// An error indicating that an identifier was used but not defined in the
    /// symbol table.
    #[error("Unknown identifier: {identifier} at {span}")]
    UnknownIdentifier { identifier: String, span: Span },

    #[error(
        "A variable with name '{identifier}' already exists in this scope at {span}"
    )]
    DuplicateLocalDeclaration { identifier: String, span: Span },

    #[error(
        "Cannot read local variable '{identifier}' in its own initializer at {span}"
    )]
    LocalReadInOwnInitializer { identifier: String, span: Span },
}

#[derive(Debug, Error)]
pub enum FatalCompilationError {
    #[error("Instruction offset overflow")]
    InstructionOffsetOverflow,

    #[error("Instruction index out of bounds: {index}")]
    InstructionIndexOutOfBounds { index: usize },

    #[error("Unexpected instruction: expected {expected}, found {found}")]
    UnexpectedInstruction {
        expected: Instruction,
        found: Instruction,
    },

    #[error("Expected at least two pipe arms, but found fewer")]
    ExpectedAtLeastTwoPipeArms,
}
