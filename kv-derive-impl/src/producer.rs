use crate::{ToRepr, ToVec};

/// Responsible for producing the vector entries based on its value.
///
/// May produce none, one or many entries, depending on a specific type.
pub trait Producer {
    fn produce(&self, output: &mut Vec<(String, String)>, key: &'static str);
}

impl<T: ToRepr> Producer for T {
    fn produce(&self, output: &mut Vec<(String, String)>, key: &'static str) {
        output.push((key.to_string(), self.to_repr()));
    }
}

impl<T: ToRepr> Producer for Option<T> {
    fn produce(&self, output: &mut Vec<(String, String)>, key: &'static str) {
        if let Some(value) = self {
            output.push((key.to_string(), value.to_repr()));
        }
    }
}

impl<T: ToRepr> Producer for Vec<T> {
    fn produce(&self, output: &mut Vec<(String, String)>, key: &'static str) {
        for item in self {
            output.push((key.to_string(), item.to_repr()));
        }
    }
}

pub struct FlatteningProducer<T>(pub T);

impl<T: ToVec> Producer for FlatteningProducer<&T> {
    fn produce(&self, output: &mut Vec<(String, String)>, _key: &'static str) {
        self.0
            .to_vec()
            .into_iter()
            .for_each(|entry| output.push(entry));
    }
}
