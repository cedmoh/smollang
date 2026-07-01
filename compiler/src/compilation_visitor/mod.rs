use crate::{
    globals::Globals,
    locals::{Local, Locals},
};
use ast::Span;
use bytecode::{
    Assembly, Constant, ConstantAddress, FunctionObject, FunctionType,
    Instruction::{self, Halt},
    MemoryAddress,
};
mod branches;
mod compilation_error;

pub use compilation_error::{CompilationError, FatalCompilationError};

/// A visitor trait for traversing the AST and applying an operation to it.
pub trait VisitAndCompile<T> {
    /// Visits a node in the AST and applies an operation to it.
    fn visit(&mut self, program: &T) -> Result<(), FatalCompilationError>;
}

/// A visitor that traverses the AST and converts it into an assembly
/// representation.
pub struct CompilationVisitor {
    /// A table of global variables and their corresponding constant addresses
    /// in the constant pool.
    pub globals: Globals,

    /// The set of locals currently in scope, ordered by declaration order.
    pub locals: Locals,

    /// The current lexical scope depth. Zero means top-level (global scope).
    pub scope_depth: usize,

    /// A vector of errors encountered during the AST to assembly conversion
    pub errors: Vec<CompilationError>,

    pub function: FunctionObject,

    pub _function_type: FunctionType,
}

impl CompilationVisitor {
    pub fn new() -> Self {
        Self {
            globals: Globals::new(),
            errors: Vec::new(),
            locals: Locals::with_reserved_first_local(),
            scope_depth: 0,
            function: FunctionObject::new(
                bytecode::StringObject::new("<script>".to_string()),
                0,
                Assembly::new(),
            ),
            _function_type: FunctionType::TopLevel,
        }
    }

    pub fn current_chunk(&self) -> &Assembly {
        &self.function.chunk
    }

    pub fn current_chunk_mut(&mut self) -> &mut Assembly {
        &mut self.function.chunk
    }

    pub fn build(mut self) -> Result<FunctionObject, FatalCompilationError> {
        self.function.chunk.instructions.push(Halt);

        Ok(self.function)
    }

    /// Emit a debug instruction to the assembly builder.
    fn _debug(&mut self) {
        self.emit(Instruction::Debug);
    }

    /// Add an error to the list of encountered non-fatal errors during the AST
    /// to assembly conversion.
    fn error(&mut self, error: CompilationError) {
        self.errors.push(error);
    }

    /// Emit an instruction to the assembly builder.
    fn emit(&mut self, instruction: Instruction) {
        self.current_chunk_mut().instructions.push(instruction);
    }

    /// Get the number of instructions in the current chunk.
    fn instruction_count(&self) -> usize {
        self.current_chunk().instructions.len()
    }

    /// Get a mutable reference to an instruction at the given index in the
    /// current chunk. If the index is out of bounds, a fatal compilation error
    /// is returned.
    fn get_instruction_mut(
        &mut self,
        index: usize,
    ) -> Result<&mut Instruction, FatalCompilationError> {
        self.current_chunk_mut()
            .instructions
            .get_mut(index)
            .ok_or(FatalCompilationError::InstructionIndexOutOfBounds { index })
    }

    /// Emit a constant to the assembly builder and return its address in the
    /// constant pool.
    fn emit_constant(&mut self, constant: Constant) -> ConstantAddress {
        self.current_chunk_mut().constants.push(constant);
        ConstantAddress::new(self.current_chunk().constants.len() - 1)
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
    fn declare_local(&mut self, identifier: &String, span: &Span) {
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
            .any(|local| local.id() == identifier);

        if is_already_declared_in_same_scope {
            self.error(CompilationError::DuplicateLocalDeclaration {
                identifier: identifier.clone(),
                span: span.clone(),
            });

            return;
        }

        self.locals.push(Local::Uninitialized {
            id: identifier.clone(),
        });
    }

    fn _define_variable(&mut self, identifier: &String) {
        if self.scope_depth == 0 {
            // Global variable
            let constant_address = self.global_name_constant(identifier);
            self.emit(Instruction::SetGlobal(constant_address));
            return;
        }

        // Local variable
        self.mark_local_initialized();
    }

    /// Mark the most recently declared local variable as initialized.
    /// This sets its depth to the current scope depth, indicating that
    /// it is now in scope and can be accessed.
    ///
    /// If there are no locals in the current scope, this
    /// function does nothing.
    fn mark_local_initialized(&mut self) {
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
    /// If the variable is not found, `None` is returned.
    ///
    /// If the variable is found but is being accessed in its own initializer,
    /// an error is recorded.
    fn resolve_local(
        &mut self,
        identifier: &String,
        span: &Span,
    ) -> Option<MemoryAddress> {
        for (slot, local) in self.locals.iter().enumerate().rev() {
            if local.id() != identifier {
                continue;
            }

            if !local.is_initialized() {
                self.error(CompilationError::LocalReadInOwnInitializer {
                    identifier: identifier.clone(),
                    span: span.clone(),
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
