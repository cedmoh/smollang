#[derive(Debug)]
pub struct Locals {
    locals: Vec<Local>,
}

impl Locals {
    pub fn new() -> Self {
        Self { locals: Vec::new() }
    }

    pub fn push(&mut self, local: Local) {
        self.locals.push(local);
    }

    pub fn pop(&mut self) -> Option<Local> {
        self.locals.pop()
    }

    pub fn last(&self) -> Option<&Local> {
        self.locals.last()
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Local> {
        self.locals.iter()
    }

    pub fn last_mut(&mut self) -> Option<&mut Local> {
        self.locals.last_mut()
    }
}

#[derive(Debug, Clone)]
pub enum Local {
    /// Represents a local variable that has been initialized.
    Initialized {
        /// The name of the local variable.
        id: String,

        /// The depth of the local variable in the scope stack.
        depth: usize,
    },
    /// Represents a local variable that has not been initialized yet.
    Uninitialized { id: String },
}

impl Local {
    /// Returns the name of the local variable.
    pub fn id(&self) -> &String {
        match self {
            Local::Initialized { id, .. } => id,
            Local::Uninitialized { id } => id,
        }
    }

    /// Returns true if the local variable has been initialized at the given
    /// depth, false otherwise.
    pub fn is_initialized_at_depth(&self, depth: usize) -> bool {
        match self {
            Local::Initialized { depth: d, .. } => *d == depth,
            Local::Uninitialized { .. } => false,
        }
    }

    /// Returns true if the local variable has been initialized, false
    /// otherwise.
    pub fn is_initialized(&self) -> bool {
        matches!(self, Local::Initialized { .. })
    }

    /// Initializes the local variable with the given depth.
    pub fn initialize(&mut self, depth: usize) {
        if let Local::Uninitialized { id } = self {
            *self = Local::Initialized {
                id: id.clone(),
                depth,
            };
        }
    }
}
