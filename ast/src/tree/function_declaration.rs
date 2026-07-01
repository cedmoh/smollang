use crate::{Expression, Identifier, Span};

/// Represents a function declaration.
///
/// # Examples
///
/// Empty function declaration:
///
/// ```smollang
/// empty ||
/// ```
///
/// Function declaration with parameters and body:
///     
/// ```smollang
/// add |x,y| x + y
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct FunctionDeclaration {
    /// The optional name of the function. If the function is anonymous, this
    /// will be `None`.
    pub name: Option<Identifier>,

    /// The parameters of the function declaration.
    pub params: FunctionParameters,

    /// The body of the function declaration. If the function has no body, this
    /// will be `None`.
    pub body: Option<FunctionBody>,

    /// The location of the AST node in the source code
    pub span: Span,
}

impl FunctionDeclaration {
    pub fn new(
        name: Option<Identifier>,
        params: FunctionParameters,
        body: Option<FunctionBody>,
        span: Span,
    ) -> Self {
        Self {
            name,
            params,
            body,
            span,
        }
    }

    /// Creates a synthetic function declaration with a dummy span.
    pub fn synthetic(
        name: Option<Identifier>,
        params: FunctionParameters,
        body: Option<FunctionBody>,
    ) -> Self {
        Self::new(name, params, body, Span::DUMMY)
    }

    pub fn builder() -> FunctionDeclarationBuilder {
        FunctionDeclarationBuilder::new()
    }
}

/// Represents a function parameter.
#[derive(Debug, Clone, PartialEq)]
pub struct FunctionParameter {
    // TODO: Add support for patterns as function parameters
    pub name: Identifier,
}

impl FunctionParameter {
    pub fn new(name: Identifier) -> Self {
        Self { name }
    }
}

/// Represents the parameters of a function declaration.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct FunctionParameters {
    pub items: Vec<FunctionParameter>,
}

impl FunctionParameters {
    pub fn new(items: Vec<FunctionParameter>) -> Self {
        Self { items }
    }
}

/// Represents the body of a function declaration.
#[derive(Debug, Clone, PartialEq)]
pub struct FunctionBody {
    pub body: Box<Expression>,
}

impl FunctionBody {
    pub fn new(body: Expression) -> Self {
        Self {
            body: Box::new(body),
        }
    }
}

pub struct FunctionDeclarationBuilder {
    name: Option<Identifier>,
    params: FunctionParameters,
    body: Option<FunctionBody>,
    span: Option<Span>,
}

impl FunctionDeclarationBuilder {
    pub fn new() -> Self {
        Self {
            name: None,
            params: FunctionParameters::default(),
            body: None,
            span: None,
        }
    }

    pub fn with_name(mut self, name: Identifier) -> Self {
        self.name = Some(name);
        self
    }

    pub fn with_params(mut self, params: Vec<FunctionParameter>) -> Self {
        self.params = FunctionParameters::new(params);
        self
    }

    pub fn with_body(mut self, body: impl Into<Expression>) -> Self {
        self.body = Some(FunctionBody::new(body.into()));
        self
    }

    pub fn name(&mut self, name: Identifier) -> &mut Self {
        self.name = Some(name);
        self
    }

    pub fn body(&mut self, body: impl Into<Expression>) -> &mut Self {
        self.body = Some(FunctionBody::new(body.into()));
        self
    }

    pub fn add_param(&mut self, param: FunctionParameter) -> &mut Self {
        self.params.items.push(param);
        self
    }

    pub fn span(&mut self, span: Span) -> &mut Self {
        self.span = Some(span);
        self
    }

    pub fn with_span(mut self, span: Span) -> Self {
        self.span = Some(span);
        self
    }

    pub fn build(self) -> FunctionDeclaration {
        FunctionDeclaration::new(
            self.name,
            self.params,
            self.body,
            self.span.unwrap_or(Span::DUMMY),
        )
    }
}
