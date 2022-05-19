/// Converts a value to its string representation.
/// The opposite of [`crate::from_repr::FromRepr`].
pub trait IntoRepr {
    fn into_repr(self) -> String;
}

impl IntoRepr for String {
    fn into_repr(self) -> String {
        self
    }
}

macro_rules! impl_repr {
    ($type:ty) => {
        impl IntoRepr for $type {
            fn into_repr(self) -> String {
                self.to_string()
            }
        }
    };
}

impl_repr!(&str);

impl_repr!(bool);

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

impl_repr!(std::net::IpAddr);
impl_repr!(std::net::Ipv4Addr);
impl_repr!(std::net::Ipv6Addr);
impl_repr!(std::net::SocketAddr);
