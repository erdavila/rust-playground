struct Type;
impl Type {
    fn access(&self) {}
    fn mutate(&mut self) {}
    fn use_ownership(self) {}
}

mod captures;
mod emulation;
mod returning;

fn main() {
    captures::captures();
    returning::returning();
    emulation::emulation();
}
