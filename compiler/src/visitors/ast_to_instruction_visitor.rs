use ast::{
    Directive, Dyadic, DyadicOperator, Expression, FunctionCall,
    FunctionCallArguments, FunctionParameter, FunctionParameters, Identifier,
    Literal, PipeArm, PipeArms, Program, VariableDeclaration,
};
use bytecode::{
    AssemblyBuilder, Constant, ConstantAddress, Instruction, Value,
};
use thiserror::Error;

use crate::{
    symbol::{Symbol, SymbolTable},
    visitors::visitor::Visitor,
};

pub struct AstToAssemblyVisitor {
    /// A symbol table for tracking variable and function names and their
    pub symbol_table: SymbolTable,

    /// An assembly builder for constructing the assembly representation of the
    /// program
    pub assembly_builder: AssemblyBuilder,
}

#[derive(Debug, Error)]
pub enum AstToAssemblyVisitorError {
    /// An error indicating that an identifier was used but not defined in the
    /// symbol table.
    #[error("Unknown identifier: {0}")]
    UnknownIdentifier(String),
}

impl AstToAssemblyVisitor {
    pub fn new() -> Self {
        Self {
            symbol_table: SymbolTable::new(),
            assembly_builder: AssemblyBuilder::new(),
        }
    }

    fn emit(&mut self, instruction: Instruction) {
        self.assembly_builder.add_instruction(instruction);
    }

    fn emit_constant(&mut self, constant: Constant) -> ConstantAddress {
        self.assembly_builder.push_constant(constant)
    }

    fn _emit_multiple(&mut self, instructions: Vec<Instruction>) {
        self.assembly_builder.add_instructions(instructions);
    }
}

impl Visitor<Program, AstToAssemblyVisitorError> for AstToAssemblyVisitor {
    fn visit(
        &mut self,
        program: &Program,
    ) -> Result<(), AstToAssemblyVisitorError> {
        for directive in &program.directives.items {
            self.visit(directive)?;
        }

        for expression in &program.body.items {
            self.visit(expression)?;
        }

        Ok(())
    }
}

impl Visitor<VariableDeclaration, AstToAssemblyVisitorError>
    for AstToAssemblyVisitor
{
    fn visit(
        &mut self,
        variable_declaration: &VariableDeclaration,
    ) -> Result<(), AstToAssemblyVisitorError> {
        println!("VariableDeclaration: {:?}", variable_declaration);

        match &variable_declaration.initial_value {
            Some(initializer) => self.visit(initializer.as_ref())?,
            None => self.emit(Instruction::Push(Value::Nil)),
        }

        match &variable_declaration.name {
            Identifier { id } => {
                let constant_address =
                    self.emit_constant(bytecode::Constant::String(id.clone()));

                self.symbol_table
                    .insert(id.clone(), Symbol::Global(constant_address));

                self.emit(Instruction::SetGlobal(constant_address));
            }
        }

        Ok(())
    }
}

impl Visitor<Literal, AstToAssemblyVisitorError> for AstToAssemblyVisitor {
    fn visit(
        &mut self,
        literal: &Literal,
    ) -> Result<(), AstToAssemblyVisitorError> {
        use Instruction::*;
        use Literal::*;

        match literal {
            Nil => self.emit(Push(Value::Nil)),
            Boolean(boolean_literal) => {
                self.emit(Push(boolean_literal.value.into()));
            }
            String(string_literal) => {
                let constant_address = self.emit_constant(
                    bytecode::Constant::String(string_literal.value.clone()),
                );

                self.emit(Constant(constant_address));
            }
            Template(_template_literal) => {
                todo!("Template literals are not yet supported");
            }
            Integer(integer_literal) => {
                // Directly emit the integer value as an immediate
                // operand of the PUSH instruction, instead of
                // first emitting a CONST instruction.
                self.emit(Push(integer_literal.value.into()));
            }
            Decimal(_decimal_literal) => {
                todo!("Decimal literals are not yet supported");
            }
            Hexadecimal(hexadecimal_literal) => {
                self.emit(Push(hexadecimal_literal.value.into()));
            }
            Binary(binary_literal) => {
                self.emit(Push(binary_literal.value.into()));
            }
            Octal(octal_literal) => {
                self.emit(Push(octal_literal.value.into()));
            }
            Array(_array_literal) => {
                todo!("Array literals are not yet supported");
            }
            Object(_object_literal) => {
                todo!("Object literals are not yet supported");
            }
        }

        Ok(())
    }
}

impl Visitor<Dyadic, AstToAssemblyVisitorError> for AstToAssemblyVisitor {
    fn visit(
        &mut self,
        dyadic: &Dyadic,
    ) -> Result<(), AstToAssemblyVisitorError> {
        // Note: The order of visiting the left and right expressions is
        // important, as it determines the order in which they are evaluated and
        // how their results are used by the operator.
        self.visit(&*dyadic.left)?;
        self.visit(&*dyadic.right)?;
        self.visit(&dyadic.operator)?;

        Ok(())
    }
}

impl Visitor<DyadicOperator, AstToAssemblyVisitorError>
    for AstToAssemblyVisitor
{
    fn visit(
        &mut self,
        operator: &DyadicOperator,
    ) -> Result<(), AstToAssemblyVisitorError> {
        use DyadicOperator::*;

        match operator {
            Add => self.emit(Instruction::Add),
            Subtract => self.emit(Instruction::Subtract),
            Multiply => self.emit(Instruction::Multiply),
            Divide => self.emit(Instruction::Divide),
            Equal => self.emit(Instruction::Equals),
            NotEqual => self.emit(Instruction::NotEquals),
            LessThan => self.emit(Instruction::LessThan),
            LessThanOrEqual => self.emit(Instruction::LessThanOrEqual),
            GreaterThan => self.emit(Instruction::GreaterThan),
            GreaterThanOrEqual => self.emit(Instruction::GreaterThanOrEqual),
            And => self.emit(Instruction::Equals),
            Or => self.emit(Instruction::NotEquals),
            AddAssign => todo!("AddAssign is not yet supported"),
            SubtractAssign => todo!("SubtractAssign is not yet supported"),
            MultiplyAssign => todo!("MultiplyAssign is not yet supported"),
            DivideAssign => todo!("DivideAssign is not yet supported"),
            Modulo => todo!("Modulo is not yet supported"),
            ModuloAssign => todo!("ModuloAssign is not yet supported"),
            Power => todo!("Power is not yet supported"),
            PowerAssign => todo!("PowerAssign is not yet supported"),
            AndAssign => todo!("AndAssign is not yet supported"),
            OrAssign => todo!("OrAssign is not yet supported"),
            RangeInclusive => todo!("RangeInclusive is not yet supported"),
            Range => todo!("Range is not yet supported"),
        }

        Ok(())
    }
}

impl Visitor<Directive, AstToAssemblyVisitorError> for AstToAssemblyVisitor {
    fn visit(
        &mut self,
        directive: &Directive,
    ) -> Result<(), AstToAssemblyVisitorError> {
        match directive {
            Directive::Use(use_directive) => {
                println!(
                    "Use({:?})({:?})",
                    use_directive.imports, use_directive.path
                )
            }
        }

        Ok(())
    }
}

impl Visitor<Identifier, AstToAssemblyVisitorError> for AstToAssemblyVisitor {
    fn visit(
        &mut self,
        identifier: &Identifier,
    ) -> Result<(), AstToAssemblyVisitorError> {
        println!("Identifier({:?})", identifier.id);
        // Assume all identifiers refer to global variables for now, and emit a
        // GETGB instruction to load the variable's value onto the stack.
        let constant_address = match self.symbol_table.get(&identifier.id) {
            Some(Symbol::Global(constant_address)) => *constant_address,
            None => {
                return Err(AstToAssemblyVisitorError::UnknownIdentifier(
                    identifier.id.clone(),
                ));
            }
        };

        self.emit(Instruction::GetGlobal(constant_address));

        Ok(())
    }
}

impl Visitor<FunctionCall, AstToAssemblyVisitorError> for AstToAssemblyVisitor {
    fn visit(
        &mut self,
        function_call: &FunctionCall,
    ) -> Result<(), AstToAssemblyVisitorError> {
        println!("FunctionCall");

        todo!()
    }
}

impl Visitor<FunctionCallArguments, AstToAssemblyVisitorError>
    for AstToAssemblyVisitor
{
    fn visit(
        &mut self,
        arguments: &FunctionCallArguments,
    ) -> Result<(), AstToAssemblyVisitorError> {
        println!("FunctionCallArguments");
        for expression in &arguments.expressions.items {
            self.visit(expression)?;
        }

        Ok(())
    }
}

impl Visitor<FunctionParameters, AstToAssemblyVisitorError>
    for AstToAssemblyVisitor
{
    fn visit(
        &mut self,
        parameters: &FunctionParameters,
    ) -> Result<(), AstToAssemblyVisitorError> {
        println!("FunctionParameters");
        for parameter in &parameters.items {
            self.visit(parameter)?;
        }

        Ok(())
    }
}

impl Visitor<FunctionParameter, AstToAssemblyVisitorError>
    for AstToAssemblyVisitor
{
    fn visit(
        &mut self,
        program: &FunctionParameter,
    ) -> Result<(), AstToAssemblyVisitorError> {
        println!("FunctionParameter({:?})", program.name);

        Ok(())
    }
}

impl Visitor<PipeArms, AstToAssemblyVisitorError> for AstToAssemblyVisitor {
    fn visit(
        &mut self,
        pipe_arms: &PipeArms,
    ) -> Result<(), AstToAssemblyVisitorError> {
        println!("PipeArms");
        for arm in &pipe_arms.arms {
            self.visit(arm)?;
        }

        Ok(())
    }
}

impl Visitor<PipeArm, AstToAssemblyVisitorError> for AstToAssemblyVisitor {
    fn visit(
        &mut self,
        pipe_arm: &PipeArm,
    ) -> Result<(), AstToAssemblyVisitorError> {
        println!("PipeArm");
        self.visit(&pipe_arm.expression)?;

        Ok(())
    }
}
impl Visitor<Expression, AstToAssemblyVisitorError> for AstToAssemblyVisitor {
    fn visit(
        &mut self,
        expression: &Expression,
    ) -> Result<(), AstToAssemblyVisitorError> {
        use Expression::*;

        match expression {
            Assignment(assignment) => {
                println!("Assignment");
                self.visit(&assignment.left)?;
                self.visit(&*assignment.right)?;
            }
            Block(block) => {
                println!("Block");
                for expression in &block.body.items {
                    self.visit(expression)?;
                }
            }
            Dyadic(dyadic) => {
                self.visit(dyadic)?;
            }
            FunctionCall(function_call) => {
                println!("FunctionCall");
                self.visit(function_call)?;
            }
            FunctionDeclaration(function_declaration) => {
                if let Some(body) = &function_declaration.body {
                    self.visit(&*body.body)?;
                }

                self.visit(&function_declaration.params)?;
            }
            Then(then_expression) => {
                self.visit(&*then_expression.condition)?;
                self.visit(&*then_expression.then_body)?;
                if let Some(else_body) = &then_expression.else_body {
                    self.visit(&**else_body)?;
                }
            }
            Pipe(pipe) => {
                self.visit(&pipe.arms)?;
            }
            Identifier(identifier) => {
                self.visit(identifier)?;
            }
            Literal(literal) => {
                self.visit(literal)?;
            }
            Match(_match_expression) => {
                todo!("Visiting match expressions is not yet supported");
            }
            Member(_member) => {
                todo!("Visiting member expressions is not yet supported");
            }
            Return(_return_expression) => {
                todo!("Visiting return expressions is not yet supported");
            }
            Break(_break_expression) => {
                todo!("Visiting break expressions is not yet supported");
            }
            Continue(_continue_expression) => {
                todo!("Visiting continue expressions is not yet supported");
            }
            Loop(_loop_expression) => {
                todo!("Visiting loop expressions is not yet supported");
            }
            VariableDeclaration(variable_declaration) => {
                self.visit(variable_declaration)?;
            }
        }

        Ok(())
    }
}
