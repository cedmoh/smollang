use ast::{
    Directive, Dyadic, DyadicOperator, Expression, FunctionCallArguments,
    FunctionParameter, FunctionParameters, Identifier, Literal, PipeArm,
    PipeArms, Program,
};
use bytecode::{Assembly, AssemblyBuilder, Instruction, Value};

use crate::visitors::visitor::Visitor;

pub struct AstToAssemblyVisitor {
    pub assembly_builder: AssemblyBuilder,
}

impl AstToAssemblyVisitor {
    pub fn new() -> Self {
        Self {
            assembly_builder: AssemblyBuilder::new(),
        }
    }

    fn emit(&mut self, instruction: Instruction) {
        self.assembly_builder.add_instruction(instruction);
    }

    fn _emit_multiple(&mut self, instructions: Vec<Instruction>) {
        self.assembly_builder.add_instructions(instructions);
    }
}

impl Visitor<Program> for AstToAssemblyVisitor {
    fn visit(&mut self, program: &Program) {
        for directive in &program.directives.items {
            self.visit(directive);
        }

        for expression in &program.body.items {
            self.visit(expression);
        }
    }
}

impl Visitor<Literal> for AstToAssemblyVisitor {
    fn visit(&mut self, literal: &Literal) {
        use Instruction::*;
        use Literal::*;

        match literal {
            Nil => self.emit(Push(Value::Nil)),
            Boolean(boolean_literal) => {
                self.emit(Push(boolean_literal.value.into()));
            }
            String(_string_literal) => {
                todo!("String literals are not yet supported");
            }
            Template(_template_literal) => {
                todo!("Template literals are not yet supported");
            }
            Integer(integer_literal) => {
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
    }
}

impl Visitor<Dyadic> for AstToAssemblyVisitor {
    fn visit(&mut self, dyadic: &Dyadic) {
        // Note: The order of visiting the left and right expressions is
        // important, as it determines the order in which they are evaluated and
        // how their results are used by the operator.
        self.visit(&*dyadic.left);
        self.visit(&*dyadic.right);
        self.visit(&dyadic.operator);
    }
}

impl Visitor<DyadicOperator> for AstToAssemblyVisitor {
    fn visit(&mut self, operator: &DyadicOperator) {
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
    }
}

impl Visitor<Directive> for AstToAssemblyVisitor {
    fn visit(&mut self, directive: &Directive) {
        match directive {
            Directive::Use(use_directive) => {
                println!(
                    "Use({:?})({:?})",
                    use_directive.imports, use_directive.path
                )
            }
        }
    }
}

impl Visitor<Identifier> for AstToAssemblyVisitor {
    fn visit(&mut self, identifier: &Identifier) {
        println!("Identifier({:?})", identifier.id);
    }
}

impl Visitor<FunctionCallArguments> for AstToAssemblyVisitor {
    fn visit(&mut self, arguments: &FunctionCallArguments) {
        println!("FunctionCallArguments");
        for expression in &arguments.expressions.items {
            self.visit(expression);
        }
    }
}

impl Visitor<FunctionParameters> for AstToAssemblyVisitor {
    fn visit(&mut self, parameters: &FunctionParameters) {
        println!("FunctionParameters");
        for parameter in &parameters.items {
            self.visit(parameter);
        }
    }
}

impl Visitor<FunctionParameter> for AstToAssemblyVisitor {
    fn visit(&mut self, program: &FunctionParameter) {
        println!("FunctionParameter({:?})", program.name);
    }
}

impl Visitor<PipeArms> for AstToAssemblyVisitor {
    fn visit(&mut self, pipe_arms: &PipeArms) {
        println!("PipeArms");
        for arm in &pipe_arms.arms {
            self.visit(arm);
        }
    }
}

impl Visitor<PipeArm> for AstToAssemblyVisitor {
    fn visit(&mut self, pipe_arm: &PipeArm) {
        println!("PipeArm");
        self.visit(&pipe_arm.expression);
    }
}
impl Visitor<Expression> for AstToAssemblyVisitor {
    fn visit(&mut self, expression: &Expression) {
        match expression {
            Expression::Assignment(assignment) => {
                println!("Assignment");
                self.visit(&assignment.left);
                self.visit(&*assignment.right);
            }
            Expression::Block(block) => {
                println!("Block");
                for expression in &block.body.items {
                    self.visit(expression);
                }
            }
            Expression::Dyadic(dyadic) => {
                self.visit(dyadic);
            }
            Expression::FunctionCall(function_call) => {
                println!("FunctionCall");
                self.visit(&*function_call.callee);
                self.visit(&function_call.arguments);
            }
            Expression::FunctionDeclaration(function_declaration) => {
                if let Some(body) = &function_declaration.body {
                    self.visit(&*body.body);
                }

                self.visit(&function_declaration.params);
            }
            Expression::Then(then_expression) => {
                self.visit(&*then_expression.condition);
                self.visit(&*then_expression.then_body);
                if let Some(else_body) = &then_expression.else_body {
                    self.visit(&**else_body);
                }
            }
            Expression::Pipe(pipe) => {
                self.visit(&pipe.arms);
            }
            Expression::Identifier(identifier) => {
                self.visit(identifier);
            }
            Expression::Literal(literal) => {
                self.visit(literal);
            }
            Expression::Match(_match_expression) => {
                todo!("Visiting match expressions is not yet supported");
            }
            Expression::Member(_member) => {
                todo!("Visiting member expressions is not yet supported");
            }
            Expression::Return(_return_expression) => {
                todo!("Visiting return expressions is not yet supported");
            }
            Expression::Break(_break_expression) => {
                todo!("Visiting break expressions is not yet supported");
            }
            Expression::Continue(_continue_expression) => {
                todo!("Visiting continue expressions is not yet supported");
            }
            Expression::Loop(_loop_expression) => {
                todo!("Visiting loop expressions is not yet supported");
            }
            Expression::VariableDeclaration(_variable_declaration) => {
                todo!("Visiting variable declarations is not yet supported");
            }
        }
    }
}
