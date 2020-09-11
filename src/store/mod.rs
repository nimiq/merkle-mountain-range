pub mod memory;

pub trait Store<H> {
    fn push(&mut self, elem: H);
    fn append(&mut self, elems: Vec<H>) {
        for elem in elems {
            self.push(elem);
        }
    }
    fn remove_back(&mut self, num_elems: usize);
    fn get(&self, pos: usize) -> Option<H>;
    fn len(&self) -> usize;
}
