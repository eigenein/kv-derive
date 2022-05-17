/// Converts a value to its string representation.
/// The opposite of [`crate::from_repr::FromRepr`].
pub trait ToRepr {
    fn to_repr(&self) -> String;
}

macro_rules! impl_repr {
    ($type:ty) => {
        impl ToRepr for $type {
            fn to_repr(&self) -> String {
                self.to_string()
            }
        }
    };
}

impl_repr!(String);
impl_repr!(&str);

impl_repr!(i8);
impl_repr!(u8);
impl_repr!(i16);
impl_repr!(u16);
impl_repr!(i32);
impl_repr!(u32);
impl_repr!(i64);
impl_repr!(u64);
impl_repr!(i128);
impl_repr!(u128);
impl_repr!(isize);
impl_repr!(usize);

impl_repr!(f32);
impl_repr!(f64);
