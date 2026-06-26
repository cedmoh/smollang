use crate::{
    globals::Globals,
    locals::{Local, Locals},
    visitors::visitor::Visitor,
};
use ast::{
    Block, Directive, Dyadic, DyadicOperator, Expression, FunctionCall,
    FunctionCallArguments, FunctionParameter, FunctionParameters, Identifier,
    Literal, Loop, Pipe, PipeArm, PipeArms, Program, Span, Then,
    VariableDeclaration,
};
use bytecode::{
    AssemblyBuilder, Constant, ConstantAddress,
    Instruction::{self, Jump, JumpIfFalse},
    InstructionOffset, MemoryAddress, Value,
};
use thiserror::Error;

pub struct AstToAssemblyVisitor {
    /// A table of global variables and their corresponding constant addresses
    /// in the constant pool.
    pub globals: Globals,

    /// An assembly builder for constructing the assembly representation of the
    /// program
    pub assembly_builder: AssemblyBuilder,

    /// A vector of errors encountered during the AST to assembly conversion
    pub errors: Vec<CompilerError>,

    /// The set of locals currently in scope, ordered by declaration order.
    locals: Locals,

    /// The current lexical scope depth. Zero means top-level (global scope).
    scope_depth: usize,
}

#[derive(Debug, Error)]
pub enum CompilerError {
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
pub enum FatalCompilerError {
    #[error("Instruction offset overflow")]
    InstructionOffsetOverflow,
}

impl AstToAssemblyVisitor {
    pub fn new() -> Self {
        Self {
            globals: Globals::new(),
            assembly_builder: AssemblyBuilder::new(),
            errors: Vec::new(),
            locals: Locals::new(),
            scope_depth: 0,
        }
    }

    /// Add an error to the list of encountered non-fatal errors during the AST
    /// to assembly conversion.
    fn error(&mut self, error: CompilerError) {
        self.errors.push(error);
    }

    /// Emit an instruction to the assembly builder.
    fn emit(&mut self, instruction: Instruction) {
        self.assembly_builder.add_instruction(instruction);
    }

    /// Emit an instruction at a specific index in the assembly builder.
    fn emit_at(&mut self, index: usize, instruction: Instruction) {
        self.assembly_builder.insert_instruction(index, instruction);
    }

    /// Emit a constant to the assembly builder and return its address in the
    /// constant pool.
    fn emit_constant(&mut self, constant: Constant) -> ConstantAddress {
        self.assembly_builder.push_constant(constant)
    }

    /// Emit multiple instructions to the assembly builder.
    fn _emit_multiple(&mut self, instructions: Vec<Instruction>) {
        self.assembly_builder.add_instructions(instructions);
    }

    /// Begin a new lexical scope. This increases the scope depth and allows for
    /// tracking of local variables declared within this scope.
    fn begin_scope(&mut self) {
        self.scope_depth += 1;
    }

    /// End the current lexical scope. This decreases the scope depth and
    /// removes any local variables that were declared within this scope
    /// from the list of locals.
    fn end_scope(&mut self) {
        while self.locals.last().is_some_and(|local| {
            local.is_initialized_at_depth(self.scope_depth)
        }) {
            self.emit(Instruction::Pop);
            self.locals.pop();
        }

        if self.scope_depth > 0 {
            self.scope_depth -= 1;
        }
    }

    /// Declare a new local variable in the current scope.
    ///
    /// If a variable with the same name already exists in the current scope,
    /// an error is recorded.
    fn declare_local(&mut self, identifier: &Identifier) {
        if self.scope_depth == 0 {
            return;
        }

        let is_already_declared_in_same_scope = self
            .locals
            .iter()
            // Check the most recently declared locals first because they are
            // more likely to be in the same scope
            .rev()
            // Get only the locals that are in the current scope by walking
            // backwards through the list of locals until we find one that is
            // not in the current scope
            .take_while(|local| match local {
                Local::Initialized { depth, .. } => *depth == self.scope_depth,
                Local::Uninitialized { .. } => true,
            })
            // Check if any of the locals in the current scope have the same
            // name as the identifier being declared
            .any(|local| local.id() == &identifier.id);

        if is_already_declared_in_same_scope {
            self.error(CompilerError::DuplicateLocalDeclaration {
                identifier: identifier.id.clone(),
                span: identifier.span.clone(),
            });

            return;
        }

        self.locals.push(Local::Uninitialized {
            id: identifier.id.clone(),
        });
    }

    /// Mark the most recently declared local variable as initialized.
    /// This sets its depth to the current scope depth, indicating that
    /// it is now in scope and can be accessed.
    ///
    /// If there are no locals in the current scope, this
    /// function does nothing.
    fn mark_local_initialized(&mut self) {
        // TODO: Consider removing because locals can be declared in the global
        // scope as well
        if self.scope_depth == 0 {
            return;
        }

        if let Some(local) = self.locals.last_mut() {
            local.initialize(self.scope_depth);
        }
    }

    /// Resolve a local variable by its identifier.
    ///
    /// If the variable is found in the current scope or any enclosing scope,
    /// its memory address is returned.
    ///
    /// If the variable is not found, `None` is returned. If the variable is
    /// found but is being accessed in its own initializer, an error is
    /// recorded.
    fn resolve_local(
        &mut self,
        identifier: &Identifier,
    ) -> Option<MemoryAddress> {
        for (slot, local) in self.locals.iter().enumerate().rev() {
            if local.id() != &identifier.id {
                continue;
            }

            if !local.is_initialized() {
                self.error(CompilerError::LocalReadInOwnInitializer {
                    identifier: identifier.id.clone(),
                    span: identifier.span.clone(),
                });
            }

            return Some(slot.into());
        }

        None
    }

    /// Get the constant address for a global variable by its identifier.
    ///
    /// If the variable is already in the symbol table, its constant address is
    /// returned. If the variable is not in the symbol table, a new constant
    /// is created for it and its address is returned.
    fn global_name_constant(&mut self, id: &str) -> ConstantAddress {
        if let Some(constant_address) = self.globals.get(id) {
            *constant_address
        } else {
            let constant_address =
                self.emit_constant(bytecode::Constant::String(id.to_string()));

            self.globals.insert(id.to_string(), constant_address);

            constant_address
        }
    }
}

impl Visitor<Block, FatalCompilerError> for AstToAssemblyVisitor {
    fn visit(&mut self, block: &Block) -> Result<(), FatalCompilerError> {
        self.begin_scope();

        for expression in &block.body.items {
            self.visit(expression)?;
        }

        self.end_scope();

        Ok(())
    }
}

impl Visitor<Program, FatalCompilerError> for AstToAssemblyVisitor {
    fn visit(&mut self, program: &Program) -> Result<(), FatalCompilerError> {
        for directive in &program.directives.items {
            self.visit(directive)?;
        }

        for expression in &program.body.items {
            self.visit(expression)?;
        }

        Ok(())
    }
}

impl Visitor<VariableDeclaration, FatalCompilerError> for AstToAssemblyVisitor {
    fn visit(
        &mut self,
        variable_declaration: &VariableDeclaration,
    ) -> Result<(), FatalCompilerError> {
        // Declare the variable
        self.declare_local(&variable_declaration.name);

        // Initialize the variable before marking it as initialized to prevent
        // reading the variable in its own initializer
        match &variable_declaration.initial_value {
            Some(initializer) => self.visit(initializer.as_ref())?,
            None => self.emit(Instruction::Push(Value::Nil)),
        }

        if self.scope_depth > 0 {
            self.mark_local_initialized();
        } else {
            let constant_address =
                self.global_name_constant(&variable_declaration.name.id);

            self.emit(Instruction::SetGlobal(constant_address));
        }

        Ok(())
    }
}

impl Visitor<Then, FatalCompilerError> for AstToAssemblyVisitor {
    fn visit(
        &mut self,
        then_expression: &Then,
    ) -> Result<(), FatalCompilerError> {
        self.visit(then_expression.condition.as_ref())?;

        // <- This is where the JumpIfFalse instruction will be inserted after
        // the body is visited
        let body_start = self.assembly_builder.instruction_length();

        self.visit(then_expression.then_body.as_ref())?;
        let body_end = self.assembly_builder.instruction_length();

        let diff = body_end - body_start; // The number of instructions in the then body
        let mut diff = diff + 1; // +1 for the JumpIfFalse instruction itself
        if then_expression.else_body.is_some() {
            diff += 1; // +1 for the Jump instruction that will be inserted after the then body
        }

        self.emit_at(
            body_start,
            JumpIfFalse(InstructionOffset::new(
                (diff).try_into().map_err(|_| {
                    FatalCompilerError::InstructionOffsetOverflow
                })?,
            )),
        );

        if let Some(else_body) = &then_expression.else_body {
            let else_start = self.assembly_builder.instruction_length();
            self.visit(else_body.as_ref())?;
            let else_end = self.assembly_builder.instruction_length();

            let diff = else_end - else_start; // The number of instructions in the else body
            let diff = diff + 1; // +1 for the Jump instruction itself

            self.emit_at(
                else_start,
                Jump(InstructionOffset::new((diff).try_into().map_err(
                    |_| FatalCompilerError::InstructionOffsetOverflow,
                )?)),
            );
        }

        Ok(())
    }
}

impl Visitor<Literal, FatalCompilerError> for AstToAssemblyVisitor {
    fn visit(&mut self, literal: &Literal) -> Result<(), FatalCompilerError> {
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

impl Visitor<Loop, FatalCompilerError> for AstToAssemblyVisitor {
    fn visit(
        &mut self,
        loop_expression: &Loop,
    ) -> Result<(), FatalCompilerError> {
        let loop_start = self.assembly_builder.instruction_length();
        self.visit(&*loop_expression.body)?;
        let loop_end = self.assembly_builder.instruction_length();

        let difference: isize = loop_start as isize - loop_end as isize;
        self.emit(Instruction::Jump(difference.into()));

        Ok(())
    }
}

impl Visitor<Dyadic, FatalCompilerError> for AstToAssemblyVisitor {
    fn visit(&mut self, dyadic: &Dyadic) -> Result<(), FatalCompilerError> {
        // Note: The order of visiting the left and right expressions is
        // important, as it determines the order in which they are evaluated and
        // how their results are used by the operator.
        self.visit(&*dyadic.left)?;
        self.visit(&*dyadic.right)?;
        self.visit(&dyadic.operator)?;

        Ok(())
    }
}

impl Visitor<DyadicOperator, FatalCompilerError> for AstToAssemblyVisitor {
    fn visit(
        &mut self,
        operator: &DyadicOperator,
    ) -> Result<(), FatalCompilerError> {
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

impl Visitor<Directive, FatalCompilerError> for AstToAssemblyVisitor {
    fn visit(
        &mut self,
        directive: &Directive,
    ) -> Result<(), FatalCompilerError> {
        match directive {
            Directive::Use(use_directive) => {
                todo!(
                    "Use directives are not yet supported: {:?}",
                    use_directive
                );
            }
        }
    }
}

impl Visitor<Identifier, FatalCompilerError> for AstToAssemblyVisitor {
    fn visit(
        &mut self,
        identifier: &Identifier,
    ) -> Result<(), FatalCompilerError> {
        if let Some(local_slot) = self.resolve_local(identifier) {
            self.emit(Instruction::GetLocal(local_slot));
            return Ok(());
        }

        let constant_address = match self.globals.get(&identifier.id) {
            Some(constant_address) => *constant_address,
            None => {
                self.error(CompilerError::UnknownIdentifier {
                    identifier: identifier.id.clone(),
                    span: identifier.span.clone(),
                });

                return Ok(());
            }
        };

        self.emit(Instruction::GetGlobal(constant_address));

        Ok(())
    }
}

impl Visitor<FunctionCall, FatalCompilerError> for AstToAssemblyVisitor {
    fn visit(
        &mut self,
        function_call: &FunctionCall,
    ) -> Result<(), FatalCompilerError> {
        match function_call.callee.as_ref() {
            Expression::Identifier(identifier) if &identifier.id == "print" => {
                function_call
                    .arguments
                    .expressions
                    .items
                    .iter()
                    .try_for_each(|expression| {
                        self.visit(expression)?;
                        self.emit(Instruction::Out);
                        Ok(())
                    })?;
            }
            Expression::Identifier(identifier) if &identifier.id == "scan" => {
                function_call
                    .arguments
                    .expressions
                    .items
                    .iter()
                    .try_for_each(|expression| {
                        self.visit(expression)?;
                        self.emit(Instruction::Out);
                        Ok(())
                    })?;

                self.emit(Instruction::In);
            }
            _ => todo!(),
        }

        Ok(())
    }
}

impl Visitor<FunctionCallArguments, FatalCompilerError>
    for AstToAssemblyVisitor
{
    fn visit(
        &mut self,
        arguments: &FunctionCallArguments,
    ) -> Result<(), FatalCompilerError> {
        for expression in &arguments.expressions.items {
            self.visit(expression)?;
        }

        Ok(())
    }
}

impl Visitor<FunctionParameters, FatalCompilerError> for AstToAssemblyVisitor {
    fn visit(
        &mut self,
        parameters: &FunctionParameters,
    ) -> Result<(), FatalCompilerError> {
        for parameter in &parameters.items {
            self.visit(parameter)?;
        }

        Ok(())
    }
}

impl Visitor<FunctionParameter, FatalCompilerError> for AstToAssemblyVisitor {
    fn visit(
        &mut self,
        _program: &FunctionParameter,
    ) -> Result<(), FatalCompilerError> {
        Ok(())
    }
}

impl Visitor<PipeArms, FatalCompilerError> for AstToAssemblyVisitor {
    fn visit(
        &mut self,
        pipe_arms: &PipeArms,
    ) -> Result<(), FatalCompilerError> {
        for arm in &pipe_arms.arms {
            self.visit(arm)?;
        }

        Ok(())
    }
}

impl Visitor<PipeArm, FatalCompilerError> for AstToAssemblyVisitor {
    fn visit(&mut self, pipe_arm: &PipeArm) -> Result<(), FatalCompilerError> {
        self.visit(&pipe_arm.expression)?;

        Ok(())
    }
}

impl Visitor<Pipe, FatalCompilerError> for AstToAssemblyVisitor {
    fn visit(&mut self, pipe: &Pipe) -> Result<(), FatalCompilerError> {
        if pipe.arms.arms.is_empty() {
            return Ok(());
        }

        // Open a new scope for the pipe to isolate the 'it' variable
        self.begin_scope();

        // Process the first arm and store its result in 'it'
        self.visit(&pipe.arms.arms[0])?;

        let it_identifier = Identifier::synthetic("it".to_string());
        self.declare_local(&it_identifier);
        self.mark_local_initialized();

        // Process remaining arms
        let remaining_arms = &pipe.arms.arms[1..];
        for (index, arm) in remaining_arms.iter().enumerate() {
            self.visit(arm)?;

            // After each arm except the last, store the result back into 'it'
            // for the next arm to use
            let is_last_arm = index == remaining_arms.len() - 1;
            if !is_last_arm {
                if let Some(it_slot) = self.resolve_local(&it_identifier) {
                    self.emit(Instruction::SetLocal(it_slot));
                }
            }
        }

        // Close the scope
        self.end_scope();

        Ok(())
    }
}

impl Visitor<Expression, FatalCompilerError> for AstToAssemblyVisitor {
    fn visit(
        &mut self,
        expression: &Expression,
    ) -> Result<(), FatalCompilerError> {
        use Expression::*;

        match expression {
            Assignment(assignment) => {
                self.visit(&*assignment.right)?;

                if let Some(local_slot) = self.resolve_local(&assignment.left) {
                    self.emit(Instruction::SetLocal(local_slot));
                } else {
                    let constant_address =
                        match self.globals.get(&assignment.left.id) {
                            Some(constant_address) => *constant_address,
                            None => {
                                self.error(CompilerError::UnknownIdentifier {
                                    identifier: assignment.left.id.clone(),
                                    span: assignment.left.span.clone(),
                                });

                                return Ok(());
                            }
                        };

                    self.emit(Instruction::SetGlobal(constant_address));
                }
            }
            Block(block) => {
                self.begin_scope();

                for expression in &block.body.items {
                    self.visit(expression)?;
                }

                self.end_scope();
            }
            Dyadic(dyadic) => {
                self.visit(dyadic)?;
            }
            FunctionCall(function_call) => {
                self.visit(function_call)?;
            }
            FunctionDeclaration(function_declaration) => {
                if let Some(body) = &function_declaration.body {
                    self.visit(&*body.body)?;
                }

                self.visit(&function_declaration.params)?;
            }
            Then(then_expression) => {
                self.visit(then_expression)?;
            }
            Pipe(pipe) => {
                self.visit(pipe)?;
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
            Loop(loop_expression) => self.visit(loop_expression)?,
            VariableDeclaration(variable_declaration) => {
                self.visit(variable_declaration)?;
            }
        }

        Ok(())
    }
}
