struct Type;
impl Type {
    fn access(&self) {}
    fn mutate(&mut self) {}
    fn use_ownership(self) {}
}

mod captures;

fn main() {
    captures::captures();
}
