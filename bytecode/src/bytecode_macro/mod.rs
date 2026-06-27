/// TODO: Replace with a procedural macro
/// that uses a string parser to parse the instructions

#[macro_export]
macro_rules! bytecode {
    // Base case — no tokens left
    (@collect $p:ident) => {};

    // --- No-arg instructions ---
    (@collect $p:ident ADD $($rest:tt)*) => {
        $p.push($crate::Instruction::Add);
        bytecode!(@collect $p $($rest)*);
    };
    (@collect $p:ident SUB $($rest:tt)*) => {
        $p.push($crate::Instruction::Sub);
        bytecode!(@collect $p $($rest)*);
    };
    (@collect $p:ident MUL $($rest:tt)*) => {
        $p.push($crate::Instruction::Mul);
        bytecode!(@collect $p $($rest)*);
    };
    (@collect $p:ident DIV $($rest:tt)*) => {
        $p.push($crate::Instruction::Div);
        bytecode!(@collect $p $($rest)*);
    };
    (@collect $p:ident POP $($rest:tt)*) => {
        $p.push($crate::Instruction::Pop);
        bytecode!(@collect $p $($rest)*);
    };
    (@collect $p:ident DUP $($rest:tt)*) => {
        $p.push($crate::Instruction::Duplicate);
        bytecode!(@collect $p $($rest)*);
    };
    (@collect $p:ident DUP2 $($rest:tt)*) => {
        $p.push($crate::Instruction::DuplicateTwo);
        bytecode!(@collect $p $($rest)*);
    };
    (@collect $p:ident EQ $($rest:tt)*) => {
        $p.push($crate::Instruction::Eq);
        bytecode!(@collect $p $($rest)*);
    };
    (@collect $p:ident LT $($rest:tt)*) => {
        $p.push($crate::Instruction::Lt);
        bytecode!(@collect $p $($rest)*);
    };
    (@collect $p:ident GT $($rest:tt)*) => {
        $p.push($crate::Instruction::Gt);
        bytecode!(@collect $p $($rest)*);
    };
    (@collect $p:ident IN $($rest:tt)*) => {
        $p.push($crate::Instruction::In);
        bytecode!(@collect $p $($rest)*);
    };
    (@collect $p:ident OUT $($rest:tt)*) => {
        $p.push($crate::Instruction::Out);
        bytecode!(@collect $p $($rest)*);
    };
    (@collect $p:ident HALT $($rest:tt)*) => {
        $p.push($crate::Instruction::Halt);
        bytecode!(@collect $p $($rest)*);
    };
    (@collect $p:ident RET $($rest:tt)*) => {
        $p.push($crate::Instruction::Return);
        bytecode!(@collect $p $($rest)*);
    };

    // --- One-arg instructions ---
    (@collect $p:ident PUSH nil $($rest:tt)*) => {
        $p.push($crate::Instruction::Push($crate::Value::Nil));
        bytecode!(@collect $p $($rest)*);
    };
    (@collect $p:ident PUSH $x:tt $($rest:tt)*) => {
        $p.push($crate::Instruction::Push($x.into()));
        bytecode!(@collect $p $($rest)*);
    };
    (@collect $p:ident LOAD $x:tt $($rest:tt)*) => {
        $p.push($crate::Instruction::Load($x.into()));
        bytecode!(@collect $p $($rest)*);
    };
    (@collect $p:ident STORE $x:tt $($rest:tt)*) => {
        $p.push($crate::Instruction::Store($x.into()));
        bytecode!(@collect $p $($rest)*);
    };
    (@collect $p:ident SETLC $x:tt $($rest:tt)*) => {
        $p.push($crate::Instruction::SetLocal($x.into()));
        bytecode!(@collect $p $($rest)*);
    };
    (@collect $p:ident GETLC $x:tt $($rest:tt)*) => {
        $p.push($crate::Instruction::GetLocal($x.into()));
        bytecode!(@collect $p $($rest)*);
    };
    (@collect $p:ident CONST $x:tt $($rest:tt)*) => {
        $p.push($crate::Instruction::Constant($x.into()));
        bytecode!(@collect $p $($rest)*);
    };
    (@collect $p:ident JUMP - $x:literal $($rest:tt)*) => {
        $p.push($crate::Instruction::Jump((-$x as isize).into()));
        bytecode!(@collect $p $($rest)*);
    };
    (@collect $p:ident JUMP $x:tt $($rest:tt)*) => {
        $p.push($crate::Instruction::Jump($x.into()));
        bytecode!(@collect $p $($rest)*);
    };
    (@collect $p:ident JT - $x:literal $($rest:tt)*) => {
        $p.push($crate::Instruction::JumpIfTrue((-$x as isize).into()));
        bytecode!(@collect $p $($rest)*);
    };
    (@collect $p:ident JT $x:tt $($rest:tt)*) => {
        $p.push($crate::Instruction::JumpIfTrue($x.into()));
        bytecode!(@collect $p $($rest)*);
    };
    (@collect $p:ident JF - $x:literal $($rest:tt)*) => {
        $p.push($crate::Instruction::JumpIfFalse((-$x as isize).into()));
        bytecode!(@collect $p $($rest)*);
    };
    (@collect $p:ident JF $x:tt $($rest:tt)*) => {
        $p.push($crate::Instruction::JumpIfFalse($x.into()));
        bytecode!(@collect $p $($rest)*);
    };
    (@collect $p:ident CALL $x:tt $($rest:tt)*) => {
        $p.push($crate::Instruction::Call($x.into()));
        bytecode!(@collect $p $($rest)*);
    };
    (@collect $p:ident SETGB $x:tt $($rest:tt)*) => {
        $p.push($crate::Instruction::SetGlobal($x.into()));
        bytecode!(@collect $p $($rest)*);
    };
    (@collect $p:ident GETGB $x:tt $($rest:tt)*) => {
        $p.push($crate::Instruction::GetGlobal($x.into()));
        bytecode!(@collect $p $($rest)*);
    };

    // Prevent recursive fallback when an unknown token appears in @collect mode.
    (@collect $p:ident $unexpected:tt $($rest:tt)*) => {
        compile_error!(concat!(
            "Unknown bytecode instruction or operand: ",
            stringify!($unexpected)
        ));
    };

    // Entry point — must be last, as ($($tokens:tt)*) matches everything
    ($($tokens:tt)*) => {{
        let mut program = Vec::new();
        bytecode!(@collect program $($tokens)*);
        program
    }};
}
