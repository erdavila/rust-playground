use std::marker::PhantomData;

pub trait TryScan: Iterator + Sized {
    fn try_scan<S, F, B, E>(&mut self, init: S, f: F) -> TryScanIter<&mut Self, S, F, B, E>
    where
        F: FnMut(&mut S, Self::Item) -> Result<Option<B>, E>;
}

pub struct TryScanIter<I, S, F, B, E> {
    inner: I,
    state: S,
    f: F,
    phantom: PhantomData<(S, B, E)>,
}

impl<I, S, F, B, E> TryScanIter<I, S, F, B, E> {
    pub fn into_final(self) -> S {
        self.state
    }
}

impl<I, S, F, B, E> Iterator for TryScanIter<I, S, F, B, E>
where
    I: Iterator,
    F: FnMut(&mut S, I::Item) -> Result<Option<B>, E>,
{
    type Item = Result<B, E>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.inner.next() {
            Some(value) => match (self.f)(&mut self.state, value) {
                Ok(Some(x)) => Some(Ok(x)),
                Ok(None) => None,
                Err(err) => Some(Err(err)),
            },
            None => {
                todo!()
            }
        }
    }
}

impl<I> TryScan for I
where
    I: Iterator,
{
    fn try_scan<S, F, B, E>(&mut self, init: S, f: F) -> TryScanIter<&mut Self, S, F, B, E>
    where
        F: FnMut(&mut S, Self::Item) -> Result<Option<B>, E>,
    {
        TryScanIter {
            inner: self,
            state: init,
            f,
            phantom: PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    use super::*;

    #[test]
    fn no_error() {
        let values = [1, 2, 3, 4, 5];
        let mut iter = values.iter();

        let mut try_scan = iter.try_scan(0, |s, v| {
            *s += v;

            match (*s).cmp(&10) {
                Ordering::Less => Ok(Some(s.to_string())),
                Ordering::Equal => Ok(None),
                Ordering::Greater => Err("Passed 10!"),
            }
        });

        assert_eq!(try_scan.next(), Some(Ok("1".to_string())));
        assert_eq!(try_scan.next(), Some(Ok("3".to_string())));
        assert_eq!(try_scan.next(), Some(Ok("6".to_string())));
        assert_eq!(try_scan.next(), None);
        assert_eq!(try_scan.into_final(), 10);
        assert_eq!(iter.next(), Some(&5));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn with_error() {
        let values = [1, 2, 3, 4, 5];
        let mut iter = values.iter();

        let try_scan = iter.try_scan(1, |s, v| {
            *s += v;

            match (*s).cmp(&10) {
                Ordering::Less => Ok(Some(v.to_string())),
                Ordering::Equal => Ok(None),
                Ordering::Greater => Err("Passed 10!"),
            }
        });

        let result: Result<Vec<_>, _> = try_scan.collect();
        assert_eq!(result, Err("Passed 10!"));
        assert_eq!(iter.next(), Some(&5));
        assert_eq!(iter.next(), None);
    }
}
