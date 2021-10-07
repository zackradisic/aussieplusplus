use super::environment::Environment;

pub trait RuntimePartialEq<Rhs: ?Sized = Self> {
    fn runtime_eq(&self, other: &Rhs) -> bool;

    fn runtime_ne(&self, other: &Rhs) -> bool {
        !self.runtime_eq(other)
    }
}
