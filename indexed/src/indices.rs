use crate::Len;

pub trait Indices<'a, Idx>: Len {
    type Indices: IntoIterator<Item = Idx>;

    fn indices(&'a self) -> Self::Indices;
}
