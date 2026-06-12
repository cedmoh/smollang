pub trait Visitor<T> {
    fn visit(&mut self, program: &T);
}
