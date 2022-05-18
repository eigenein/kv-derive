use std::iter;

use crate::into_vec::KeyValueIterator;
use crate::{IntoRepr, IntoVec};

/// Responsible for producing the vector entries based on its value.
///
/// May produce none, one or many entries, depending on a specific type.
pub trait Producer {
    fn produce(self, key: &'static str) -> KeyValueIterator;
}

impl<T: IntoRepr> Producer for T {
    fn produce(self, key: &'static str) -> KeyValueIterator {
        Box::new(iter::once((key.to_string(), self.into_repr())))
    }
}

impl<T: IntoRepr> Producer for Option<T> {
    fn produce(self, key: &'static str) -> KeyValueIterator {
        if let Some(value) = self {
            Box::new(iter::once((key.to_string(), value.into_repr())))
        } else {
            Box::new(iter::empty())
        }
    }
}

impl<T: IntoRepr + 'static> Producer for Vec<T> {
    fn produce(self, key: &'static str) -> KeyValueIterator {
        Box::new(
            self.into_iter()
                .map(|item| (key.to_string(), item.into_repr())),
        )
    }
}

pub struct FlatteningProducer<T>(pub T);

impl<T: IntoVec> Producer for FlatteningProducer<T> {
    fn produce(self, _key: &'static str) -> KeyValueIterator {
        self.0.into_iter()
    }
}
