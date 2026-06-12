use ast::{
    Directive, DyadicOperator, Expression, FunctionCallArguments,
    FunctionParameter, FunctionParameters, Identifier, Literal, PipeArm,
    PipeArms, Program,
};

use crate::visitors::visitor::Visitor;

pub struct AstToInstructionVisitor;

impl Visitor<Directive> for AstToInstructionVisitor {
    fn visit(&mut self, directive: &Directive) {
        match directive {
            Directive::Use(inner) => {
                println!("Use({:?})({:?})", inner.imports, inner.path)
            }
        }
    }
}

impl Visitor<Identifier> for AstToInstructionVisitor {
    fn visit(&mut self, identifier: &Identifier) {
        println!("Identifier({:?})", identifier.id);
    }
}

impl Visitor<DyadicOperator> for AstToInstructionVisitor {
    fn visit(&mut self, operator: &DyadicOperator) {
        println!("DyadicOperator({:?})", operator);
    }
}

impl Visitor<FunctionCallArguments> for AstToInstructionVisitor {
    fn visit(&mut self, arguments: &FunctionCallArguments) {
        println!("FunctionCallArguments");
        for expression in &arguments.expressions.items {
            self.visit(expression);
        }
    }
}

impl Visitor<FunctionParameters> for AstToInstructionVisitor {
    fn visit(&mut self, parameters: &FunctionParameters) {
        println!("FunctionParameters");
        for parameter in &parameters.items {
            self.visit(parameter);
        }
    }
}

impl Visitor<FunctionParameter> for AstToInstructionVisitor {
    fn visit(&mut self, program: &FunctionParameter) {
        println!("FunctionParameter({:?})", program.name);
    }
}

impl Visitor<PipeArms> for AstToInstructionVisitor {
    fn visit(&mut self, pipe_arms: &PipeArms) {
        println!("PipeArms");
        for arm in &pipe_arms.arms {
            self.visit(arm);
        }
    }
}

impl Visitor<PipeArm> for AstToInstructionVisitor {
    fn visit(&mut self, pipe_arm: &PipeArm) {
        println!("PipeArm");
        self.visit(&pipe_arm.expression);
    }
}

impl Visitor<Literal> for AstToInstructionVisitor {
    fn visit(&mut self, literal: &Literal) {
        match literal {
            Literal::Nil => println!("Nil"),
            Literal::Boolean(boolean_literal) => {
                println!("Boolean({:?})", boolean_literal.value)
            }
            Literal::String(string_literal) => {
                println!("String({:?})", string_literal.value)
            }
            Literal::Template(template_literal) => {
                println!("Template({:?})", template_literal.value)
            }
            Literal::Integer(integer_literal) => {
                println!("Integer({:?})", integer_literal.value)
            }
            Literal::Decimal(decimal_literal) => {
                println!("Decimal({:?})", decimal_literal.value)
            }
            Literal::Hexadecimal(hexadecimal_literal) => {
                println!("Hexadecimal({:?})", hexadecimal_literal.value)
            }
            Literal::Binary(binary_literal) => {
                println!("Binary({:?})", binary_literal.value)
            }
            Literal::Octal(octal_literal) => {
                println!("Octal({:?})", octal_literal.value)
            }
            Literal::Array(array_literal) => {
                println!("Array({:?})", array_literal.elements)
            }
            Literal::Object(object_literal) => {
                println!("Object({:?})", object_literal.properties)
            }
        }
    }
}

impl Visitor<Expression> for AstToInstructionVisitor {
    fn visit(&mut self, expression: &Expression) {
        match expression {
            Expression::Assignment(inner) => {
                println!("Assignment");
                self.visit(&inner.left);
                self.visit(&*inner.right);
            }
            Expression::Block(inner) => {
                println!("Block");
                for expression in &inner.body.items {
                    self.visit(expression);
                }
            }
            Expression::Dyadic(inner) => {
                println!("Dyadic");
                self.visit(&*inner.left);
                self.visit(&inner.operator);
                self.visit(&*inner.right);
            }
            Expression::FunctionCall(inner) => {
                println!("FunctionCall");
                self.visit(&*inner.callee);
                self.visit(&inner.arguments);
            }
            Expression::FunctionDeclaration(inner) => {
                println!("FunctionDeclaration({:?})", inner.name);

                if let Some(body) = &inner.body {
                    self.visit(&*body.body);
                }

                self.visit(&inner.params);
            }
            Expression::Then(inner) => {
                println!("Then");
                self.visit(&*inner.condition);
                self.visit(&*inner.then_body);
                if let Some(else_body) = &inner.else_body {
                    self.visit(&**else_body);
                }
            }
            Expression::Pipe(inner) => {
                println!("Pipe");
                self.visit(&inner.arms);
            }
            Expression::Identifier(inner) => {
                self.visit(inner);
            }
            Expression::Literal(inner) => {
                self.visit(inner);
            }
            Expression::Match(inner) => {
                todo!()
            }
            Expression::Member(inner) => {
                todo!()
            }
            Expression::Return(inner) => {
                todo!()
            }
            Expression::Break(inner) => {
                todo!()
            }
            Expression::Continue(inner) => {
                todo!()
            }
            Expression::Loop(inner) => {
                todo!()
            }
            Expression::VariableDeclaration(inner) => {
                todo!()
            }
        }
    }
}

impl Visitor<Program> for AstToInstructionVisitor {
    fn visit(&mut self, program: &Program) {
        for directive in &program.directives.items {
            self.visit(directive);
        }
    }
}
