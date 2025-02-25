use multi_binary_heap::{Facet, FacetsTuple, MultiBinaryHeap, Priority};

struct Container0 {
    heap: MultiBinaryHeap<Person, (Name, Youngest, Oldest)>,
}

struct Container1<Fs: FacetsTuple<Person>> {
    heap: MultiBinaryHeap<Person, Fs>,
}

struct Container2<T, Fs: FacetsTuple<T>> {
    heap: MultiBinaryHeap<T, Fs>,
}

fn main() {
    let mbh = MultiBinaryHeap::with_facets((Name, Youngest, Oldest));
    let _: MultiBinaryHeap<Person, (Name, Youngest, Oldest)> = MultiBinaryHeap::new();

    Container0 { heap: mbh.clone() };
    Container1 { heap: mbh.clone() };
    Container2 { heap: mbh };
}

#[derive(Clone)]
struct Person {
    name: String,
    age: u8,
}

#[derive(Clone)]
struct Name;
impl Facet<Person> for Name {
    type Output<'a> = &'a str;

    const PRIORITY: Priority = Priority::Min;

    fn facet<'a>(&self, elem: &'a Person) -> Self::Output<'a> {
        &elem.name
    }
}
impl Default for Name {
    fn default() -> Self {
        Self
    }
}

#[derive(Clone)]
struct Youngest;
impl Facet<Person> for Youngest {
    type Output<'a> = u8;

    const PRIORITY: Priority = Priority::Min;

    fn facet<'a>(&self, elem: &'a Person) -> Self::Output<'a> {
        elem.age
    }
}
impl Default for Youngest {
    fn default() -> Self {
        Self
    }
}

#[derive(Clone)]
struct Oldest;
impl Facet<Person> for Oldest {
    type Output<'a> = u8;

    const PRIORITY: Priority = Priority::Max;

    fn facet<'a>(&self, elem: &'a Person) -> Self::Output<'a> {
        elem.age
    }
}
impl Default for Oldest {
    fn default() -> Self {
        Self
    }
}
