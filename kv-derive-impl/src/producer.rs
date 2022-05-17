use crate::ToRepr;

/// Produces the vector entries for itself.
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
