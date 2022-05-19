use crate::from_repr::FromRepr;

/// Responsible for consuming the scalar value and modifying itself accordingly.
pub trait Consumer: Sized {
    /// Defines the scalar representation type.
    type Repr: FromRepr;

    fn init(value: Self::Repr) -> Self;

    /// Consume or accumulate the new value into itself.
    ///
    /// May consume one or more entries.
    fn consume(&mut self, value: Self::Repr);
}

impl<T: FromRepr> Consumer for T {
    type Repr = T;

    fn init(value: T) -> Self {
        value
    }

    fn consume(&mut self, value: T) {
        *self = value;
    }
}

impl<T: FromRepr> Consumer for Option<T> {
    type Repr = T;

    fn init(value: T) -> Self {
        Some(value)
    }

    fn consume(&mut self, value: T) {
        *self = Some(value);
    }
}

impl<T: FromRepr> Consumer for Vec<T> {
    type Repr = T;

    fn init(value: T) -> Self {
        vec![value]
    }

    fn consume(&mut self, value: T) {
        self.push(value);
    }
}
