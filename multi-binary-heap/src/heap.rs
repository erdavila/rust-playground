use crate::Facet;

pub(crate) struct Heap<T, F: Facet<T>> {
    _phantom: std::marker::PhantomData<(T, F)>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {}
}
