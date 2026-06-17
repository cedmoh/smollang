use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum Constant {
    Nil,
    Int(i32),
    Float(f64),
    Boolean(bool),
    String(String),
}

impl Display for Constant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Constant::Nil => write!(f, "nil"),
            Constant::Int(int) => write!(f, "{}", int),
            Constant::Float(float) => write!(f, "{:.4}", float),
            Constant::Boolean(boolean) => match boolean {
                true => write!(f, "true"),
                false => write!(f, "false"),
            },
            Constant::String(string) => write!(f, "'{}'", string),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_display_constants() {
        assert_eq!(Constant::Nil.to_string(), "nil");
        assert_eq!(Constant::Int(42).to_string(), "42");
        assert_eq!(Constant::Float(3.141519).to_string(), "3.1415");
        assert_eq!(Constant::Boolean(true).to_string(), "true");
        assert_eq!(Constant::Boolean(false).to_string(), "false");
        assert_eq!(
            Constant::String("Hello, world!".into()).to_string(),
            "'Hello, world!'"
        );
    }
}
