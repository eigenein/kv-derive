pub trait IntoVec: Sized {
    fn into_iter(self) -> Box<dyn Iterator<Item = (String, String)>>;

    fn into_vec(self) -> Vec<(String, String)> {
        self.into_iter().collect()
    }
}
