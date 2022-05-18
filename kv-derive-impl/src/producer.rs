use std::iter;

use crate::{IntoRepr, IntoVec};

/// Responsible for producing the vector entries based on its value.
///
/// May produce none, one or many entries, depending on a specific type.
pub trait Producer {
    type Iter: Iterator<Item = (String, String)>;

    fn produce(self, key: &'static str) -> Self::Iter;
}

/// Scalar producer.
///
/// Produces exactly one key-value pair.
impl<T: IntoRepr> Producer for T {
    type Iter = iter::Once<(String, String)>;

    fn produce(self, key: &'static str) -> Self::Iter {
        iter::once((key.to_string(), self.into_repr()))
    }
}

/// Optional scaler producer.
///
/// Produces none or one key-value pair.
impl<T: IntoRepr + 'static> Producer for Option<T> {
    type Iter = iter::Map<
        std::option::IntoIter<(T, &'static str)>,
        fn((T, &'static str)) -> (String, String),
    >;

    fn produce(self, key: &'static str) -> Self::Iter {
        self
            // Move `key` in so that it need not to be captured.
            .zip(Some(key))
            .into_iter()
            .map(|(value, key)| (key.to_string(), value.into_repr()))
    }
}

/// Collection producer.
///
/// Produces as many key-value pairs as the number of the collection elements.
impl<T: IntoRepr + 'static> Producer for Vec<T> {
    type Iter = iter::Map<
        iter::Zip<iter::Repeat<&'static str>, std::vec::IntoIter<T>>,
        fn((&'static str, T)) -> (String, String),
    >;

    fn produce(self, key: &'static str) -> Self::Iter {
        iter::repeat(key)
            .zip(self.into_iter())
            .map(|(key, item)| (key.to_string(), item.into_repr()))
    }
}

/// Simple flattening producer.
///
/// Forwards all the key-value pairs from the inner structure.
pub struct FlatteningProducer<T>(pub T);

impl<T: IntoVec> Producer for FlatteningProducer<T> {
    type Iter = Box<dyn Iterator<Item = (String, String)>>;

    fn produce(self, _key: &'static str) -> Self::Iter {
        self.0.into_iter()
    }
}

/// Prefixed flattening producer.
///
/// Forwards all the key-value pairs from the inner structure,
/// but additionally prepends the keys with the prefix.
pub struct PrefixedFlatteningProducer<T>(pub T, pub &'static str);

impl<T: IntoVec> Producer for PrefixedFlatteningProducer<T> {
    type Iter = Box<dyn Iterator<Item = (String, String)>>;

    fn produce(self, _key: &'static str) -> Self::Iter {
        Box::new(
            self.0
                .into_iter()
                .map(move |(key, value)| (format!("{}{}", self.1, key), value)),
        )
    }
}
