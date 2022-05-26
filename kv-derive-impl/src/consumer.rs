use std::marker::PhantomData;

/// Responsible for consuming the scalar value and modifying itself accordingly.
pub trait Consumer: Sized {
    /// Defines the scalar representation type.
    type Repr;

    type Target;

    fn init(&self, value: Self::Repr) -> Self::Target;

    /// Consume or accumulate the new value into itself.
    ///
    /// May consume one or more entries.
    fn consume(&self, target: &mut Self::Target, value: Self::Repr);
}

pub struct ScalarConsumer<T>(pub PhantomData<T>);

impl<T> Consumer for ScalarConsumer<T> {
    type Repr = T;
    type Target = T;

    #[inline]
    fn init(&self, value: Self::Repr) -> Self::Target {
        value
    }

    #[inline]
    fn consume(&self, target: &mut Self::Target, value: Self::Repr) {
        *target = value;
    }
}

pub struct OptionConsumer<T: Consumer>(pub T);

impl<T: Consumer> Consumer for OptionConsumer<T> {
    type Repr = T::Repr;
    type Target = Option<T::Target>;

    #[inline]
    fn init(&self, value: Self::Repr) -> Self::Target {
        Some(self.0.init(value))
    }

    #[inline]
    fn consume(&self, target: &mut Self::Target, value: Self::Repr) {
        *target = self.init(value);
    }
}

pub struct CollectionConsumer<T: Consumer>(pub T);

impl<T: Consumer> Consumer for CollectionConsumer<T> {
    type Repr = T::Repr;
    type Target = Vec<T::Target>;

    #[inline]
    fn init(&self, value: Self::Repr) -> Self::Target {
        vec![self.0.init(value)]
    }

    #[inline]
    fn consume(&self, target: &mut Self::Target, value: Self::Repr) {
        target.push(self.0.init(value));
    }
}
