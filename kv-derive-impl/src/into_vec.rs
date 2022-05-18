pub type KeyValueIterator = Box<dyn Iterator<Item = (String, String)>>;

pub trait IntoVec: Sized {
    fn into_iter(self) -> KeyValueIterator;

    fn into_vec(self) -> Vec<(String, String)> {
        self.into_iter().collect()
    }
}
