use std::iter;

use crate::{IntoRepr, IntoVec};

/// Responsible for producing the vector entries based on its value.
///
/// May produce none, one or many entries, depending on a specific type.
pub trait Producer {
    type Iter: Iterator<Item = (String, String)>;

    fn produce(self, key: &'static str) -> Self::Iter;
}

impl<T: IntoRepr> Producer for T {
    type Iter = iter::Once<(String, String)>;

    fn produce(self, key: &'static str) -> Self::Iter {
        iter::once((key.to_string(), self.into_repr()))
    }
}

impl<T: IntoRepr> Producer for Option<T> {
    type Iter = Box<dyn Iterator<Item = (String, String)>>;

    fn produce(self, key: &'static str) -> Self::Iter {
        if let Some(value) = self {
            Box::new(iter::once((key.to_string(), value.into_repr())))
        } else {
            Box::new(iter::empty())
        }
    }
}

impl<T: IntoRepr + 'static> Producer for Vec<T> {
    type Iter = Box<dyn Iterator<Item = (String, String)>>;

    fn produce(self, key: &'static str) -> Self::Iter {
        Box::new(
            self.into_iter()
                .map(|item| (key.to_string(), item.into_repr())),
        )
    }
}

pub struct FlatteningProducer<T>(pub T);

impl<T: IntoVec> Producer for FlatteningProducer<T> {
    type Iter = Box<dyn Iterator<Item = (String, String)>>;

    fn produce(self, _key: &'static str) -> Self::Iter {
        self.0.into_iter()
    }
}
