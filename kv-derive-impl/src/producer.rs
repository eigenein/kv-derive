use crate::ToRepr;

/// Responsible for producing the vector entries based on its value.
///
/// May produce none, one or many entries, depending on a specific type.
pub trait Producer {
    fn produce(&self, output: &mut Vec<(&'static str, String)>, key: &'static str);
}

impl<T: ToRepr> Producer for T {
    fn produce(&self, output: &mut Vec<(&'static str, String)>, key: &'static str) {
        output.push((key, self.to_repr()));
    }
}

impl<T: ToRepr> Producer for Option<T> {
    fn produce(&self, output: &mut Vec<(&'static str, String)>, key: &'static str) {
        if let Some(value) = self {
            output.push((key, value.to_repr()));
        }
    }
}

impl<T: ToRepr> Producer for Vec<T> {
    fn produce(&self, output: &mut Vec<(&'static str, String)>, key: &'static str) {
        for item in self {
            output.push((key, item.to_repr()));
        }
    }
}
