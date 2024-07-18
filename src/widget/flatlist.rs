pub trait FlatListSource {
    type Item;
    fn len(&self) -> usize;
    fn get(&self, index: usize) -> Option<Self::Item>;
}
