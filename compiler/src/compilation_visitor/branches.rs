use ast::{
    Block, Directive, Dyadic, DyadicOperator, Expression, FunctionCall,
    FunctionCallArguments, FunctionDeclaration, FunctionParameter,
    FunctionParameters, Identifier, Literal, Loop, Pipe, PipeArm, PipeArms,
    Program, Then, VariableDeclaration,
};
use bytecode::{
    Instruction::{self, Jump, JumpIfFalse, Return},
    InstructionOffset, Value,
};

use crate::compilation_visitor::{
    CompilationError, CompilationVisitor, FatalCompilationError,
    VisitAndCompile,
};

impl VisitAndCompile<Block> for CompilationVisitor {
    fn visit(&mut self, block: &Block) -> Result<(), FatalCompilationError> {
        self.begin_scope();

        match block.body.items.is_empty() {
            true => {
                self.emit(Instruction::Push(Value::Nil));
            }
            false => {
                for expression in &block.body.items {
                    self.visit(expression)?;
                }
            }
        }

        self.end_scope();

        Ok(())
    }
}

impl VisitAndCompile<Program> for CompilationVisitor {
    fn visit(
        &mut self,
        program: &Program,
    ) -> Result<(), FatalCompilationError> {
        for directive in &program.directives.items {
            self.visit(directive)?;
        }

        for expression in &program.body.items {
            self.visit(expression)?;
        }

        Ok(())
    }
}

impl VisitAndCompile<VariableDeclaration> for CompilationVisitor {
    fn visit(
        &mut self,
        variable_declaration: &VariableDeclaration,
    ) -> Result<(), FatalCompilationError> {
        // Declare the variable
        self.declare_local(
            &variable_declaration.name.id,
            &variable_declaration.name.span,
        );

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

impl VisitAndCompile<Then> for CompilationVisitor {
    fn visit(
        &mut self,
        then_expression: &Then,
    ) -> Result<(), FatalCompilationError> {
        self.visit(then_expression.condition.as_ref())?;

        let jump_if_false_index = self.assembly_builder.instruction_length();
        self.emit(Instruction::JumpIfFalse(InstructionOffset::DUMMY));

        let body_start = self.assembly_builder.instruction_length();
        self.visit(then_expression.then_body.as_ref())?;
        let body_end = self.assembly_builder.instruction_length();

        let diff = body_end - body_start; // The number of instructions in the then body
        let diff = diff + 1; // +1 for the JumpIfFalse instruction itself

        let JumpIfFalse(offset) =
            self.edit_instruction_at(jump_if_false_index)?
        else {
            return Err(FatalCompilationError::UnexpectedInstruction {
                expected: Instruction::JumpIfFalse(InstructionOffset::DUMMY),
                found: Instruction::JumpIfFalse(InstructionOffset::DUMMY),
            });
        };

        *offset =
            InstructionOffset::new((diff).try_into().map_err(|_| {
                FatalCompilationError::InstructionOffsetOverflow
            })?);

        if let Some(else_body) = &then_expression.else_body {
            let jump_index = self.assembly_builder.instruction_length();
            self.emit(Instruction::Jump(InstructionOffset::DUMMY));

            let else_start = self.assembly_builder.instruction_length();
            self.visit(else_body.as_ref())?;
            let else_end = self.assembly_builder.instruction_length();

            let diff = else_end - else_start; // The number of instructions in the else body
            let diff = diff + 1; // +1 for the Jump instruction itself

            let Jump(offset) = self.edit_instruction_at(jump_index)? else {
                return Err(FatalCompilationError::UnexpectedInstruction {
                    expected: Instruction::Jump(InstructionOffset::DUMMY),
                    found: Instruction::Jump(InstructionOffset::DUMMY),
                });
            };

            *offset =
                InstructionOffset::new((diff).try_into().map_err(|_| {
                    FatalCompilationError::InstructionOffsetOverflow
                })?);
        }

        Ok(())
    }
}

impl VisitAndCompile<Literal> for CompilationVisitor {
    fn visit(
        &mut self,
        literal: &Literal,
    ) -> Result<(), FatalCompilationError> {
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

impl VisitAndCompile<Loop> for CompilationVisitor {
    fn visit(
        &mut self,
        loop_expression: &Loop,
    ) -> Result<(), FatalCompilationError> {
        let loop_start = self.assembly_builder.instruction_length();
        self.visit(&*loop_expression.body)?;
        let loop_end = self.assembly_builder.instruction_length();

        let difference: isize = loop_start as isize - loop_end as isize;
        self.emit(Instruction::Jump(difference.into()));

        Ok(())
    }
}

impl VisitAndCompile<Dyadic> for CompilationVisitor {
    fn visit(&mut self, dyadic: &Dyadic) -> Result<(), FatalCompilationError> {
        use DyadicOperator::*;
        // Note: The order of visiting the left and right expressions is
        // important, as it determines the order in which they are evaluated and
        // how their results are used by the operator.

        match &dyadic.operator {
            And => {
                self.visit(dyadic.left.as_ref())?;
                self.emit(Instruction::JumpIfFalse(InstructionOffset::DUMMY));
                let jump_if_false_index =
                    self.assembly_builder.instruction_length() - 1;

                self.visit(dyadic.right.as_ref())?;

                let jump_if_false_offset =
                    self.assembly_builder.instruction_length()
                        - jump_if_false_index;
                let JumpIfFalse(offset) =
                    self.edit_instruction_at(jump_if_false_index)?
                else {
                    return Err(FatalCompilationError::UnexpectedInstruction {
                        expected: Instruction::JumpIfFalse(
                            InstructionOffset::DUMMY,
                        ),
                        found: Instruction::JumpIfFalse(
                            InstructionOffset::DUMMY,
                        ),
                    });
                };

                *offset = InstructionOffset::new(
                    (jump_if_false_offset).try_into().map_err(|_| {
                        FatalCompilationError::InstructionOffsetOverflow
                    })?,
                );
            }
            Or => {
                self.visit(dyadic.left.as_ref())?;
                self.emit(Instruction::JumpIfFalse(InstructionOffset::DUMMY));
                let jump_if_false_index =
                    self.assembly_builder.instruction_length() - 1;

                self.visit(dyadic.right.as_ref())?;

                let jump_if_false_offset =
                    self.assembly_builder.instruction_length()
                        - jump_if_false_index;
                let JumpIfFalse(offset) =
                    self.edit_instruction_at(jump_if_false_index)?
                else {
                    return Err(FatalCompilationError::UnexpectedInstruction {
                        expected: Instruction::JumpIfFalse(
                            InstructionOffset::DUMMY,
                        ),
                        found: Instruction::JumpIfFalse(
                            InstructionOffset::DUMMY,
                        ),
                    });
                };

                *offset = InstructionOffset::new(
                    (jump_if_false_offset).try_into().map_err(|_| {
                        FatalCompilationError::InstructionOffsetOverflow
                    })?,
                );
            }
            _ => {
                self.visit(dyadic.left.as_ref())?;
                self.visit(dyadic.right.as_ref())?;
                self.visit(&dyadic.operator)?;
            }
        }

        Ok(())
    }
}

impl VisitAndCompile<DyadicOperator> for CompilationVisitor {
    fn visit(
        &mut self,
        operator: &DyadicOperator,
    ) -> Result<(), FatalCompilationError> {
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
            Modulo => self.emit(Instruction::Modulo),
            ModuloAssign => todo!("ModuloAssign is not yet supported"),
            Power => self.emit(Instruction::Power),
            PowerAssign => todo!("PowerAssign is not yet supported"),
            AndAssign => todo!("AndAssign is not yet supported"),
            OrAssign => todo!("OrAssign is not yet supported"),
            RangeInclusive => todo!("RangeInclusive is not yet supported"),
            Range => todo!("Range is not yet supported"),
        }

        Ok(())
    }
}

impl VisitAndCompile<Directive> for CompilationVisitor {
    fn visit(
        &mut self,
        directive: &Directive,
    ) -> Result<(), FatalCompilationError> {
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

impl VisitAndCompile<Identifier> for CompilationVisitor {
    fn visit(
        &mut self,
        identifier: &Identifier,
    ) -> Result<(), FatalCompilationError> {
        // First, check if the identifier is a local variable in the current scope
        if let Some(local_slot) =
            self.resolve_local(&identifier.id, &identifier.span)
        {
            self.emit(Instruction::GetLocal(local_slot));
            return Ok(());
        }

        // If not found in locals, check if it's a global variable
        let constant_address = match self.globals.get(&identifier.id) {
            Some(constant_address) => *constant_address,
            None => {
                self.error(CompilationError::UnknownIdentifier {
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

impl VisitAndCompile<FunctionDeclaration> for CompilationVisitor {
    fn visit(
        &mut self,
        function_declaration: &FunctionDeclaration,
    ) -> Result<(), FatalCompilationError> {
        let span = function_declaration.span.clone();
        let _name =
            function_declaration.name.clone().unwrap_or(Identifier::new(
                format!("<fn {}:{}>", span.start, span.end),
                span.clone(),
            ));

        // Assume that the function is not being used as an expression.
        // TODO: Handle function expressions and closures in the future.
        match &function_declaration.body {
            Some(body) => {
                self.visit(body.body.as_ref())?;
                self.emit(Return);
            }
            None => self.emit(Return),
        }

        Ok(())
    }
}

impl VisitAndCompile<FunctionCall> for CompilationVisitor {
    fn visit(
        &mut self,
        function_call: &FunctionCall,
    ) -> Result<(), FatalCompilationError> {
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

impl VisitAndCompile<FunctionCallArguments> for CompilationVisitor {
    fn visit(
        &mut self,
        arguments: &FunctionCallArguments,
    ) -> Result<(), FatalCompilationError> {
        for expression in &arguments.expressions.items {
            self.visit(expression)?;
        }

        Ok(())
    }
}

impl VisitAndCompile<FunctionParameters> for CompilationVisitor {
    fn visit(
        &mut self,
        parameters: &FunctionParameters,
    ) -> Result<(), FatalCompilationError> {
        for parameter in &parameters.items {
            self.visit(parameter)?;
        }

        Ok(())
    }
}

impl VisitAndCompile<FunctionParameter> for CompilationVisitor {
    fn visit(
        &mut self,
        _program: &FunctionParameter,
    ) -> Result<(), FatalCompilationError> {
        Ok(())
    }
}

impl VisitAndCompile<PipeArms> for CompilationVisitor {
    fn visit(
        &mut self,
        pipe_arms: &PipeArms,
    ) -> Result<(), FatalCompilationError> {
        for arm in &pipe_arms.arms {
            self.visit(arm)?;
        }

        Ok(())
    }
}

impl VisitAndCompile<PipeArm> for CompilationVisitor {
    fn visit(
        &mut self,
        pipe_arm: &PipeArm,
    ) -> Result<(), FatalCompilationError> {
        self.visit(&pipe_arm.expression)?;

        Ok(())
    }
}

impl VisitAndCompile<Pipe> for CompilationVisitor {
    fn visit(&mut self, pipe: &Pipe) -> Result<(), FatalCompilationError> {
        if pipe.arms.arms.is_empty() {
            return Ok(());
        }

        // Open a new scope for the pipe to isolate the 'it' variable
        self.begin_scope();

        let (first_arm, remaining_arms) = pipe
            .arms
            .arms
            .split_first()
            .ok_or(FatalCompilationError::ExpectedAtLeastTwoPipeArms)?;

        // Process the first arm and put it on the stack
        self.visit(first_arm)?;

        // Declare the 'it' variable in the current scope
        // and initialize it with the value from the first arm
        let it_identifier =
            Identifier::new("it".to_string(), pipe.span.clone());
        self.declare_local(&it_identifier.id, &it_identifier.span);
        self.mark_local_initialized();

        if let Some(it_slot) =
            self.resolve_local(&it_identifier.id, &it_identifier.span)
        {
            self.emit(Instruction::SetLocal(it_slot));
        }

        // Process remaining arms
        for (index, arm) in remaining_arms.iter().enumerate() {
            self.visit(arm)?;

            // After each arm except the last, store the result back into 'it'
            // for the next arm to use
            let is_last_arm = index == remaining_arms.len() - 1;
            if !is_last_arm {
                if let Some(it_slot) =
                    self.resolve_local(&it_identifier.id, &it_identifier.span)
                {
                    self.emit(Instruction::SetLocal(it_slot));
                }
            }
        }

        // Remove pipe-local bindings without emitting POP so the final value of
        // the pipe remains on the stack as the expression result.
        while self.locals.last().is_some_and(|local| {
            local.is_initialized_at_depth(self.scope_depth)
        }) {
            self.locals.pop();
        }

        if self.scope_depth > 0 {
            self.scope_depth -= 1;
        }

        Ok(())
    }
}

impl VisitAndCompile<Expression> for CompilationVisitor {
    fn visit(
        &mut self,
        expression: &Expression,
    ) -> Result<(), FatalCompilationError> {
        use Expression::*;

        match expression {
            Assignment(assignment) => {
                self.visit(&*assignment.right)?;

                if let Some(local_slot) = self
                    .resolve_local(&assignment.left.id, &assignment.left.span)
                {
                    self.emit(Instruction::SetLocal(local_slot));
                } else {
                    let constant_address = match self
                        .globals
                        .get(&assignment.left.id)
                    {
                        Some(constant_address) => *constant_address,
                        None => {
                            self.error(CompilationError::UnknownIdentifier {
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
                self.visit(block)?;
            }
            Dyadic(dyadic) => {
                self.visit(dyadic)?;
            }
            FunctionCall(function_call) => {
                self.visit(function_call)?;
            }
            FunctionDeclaration(function_declaration) => {
                self.visit(function_declaration)?
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
