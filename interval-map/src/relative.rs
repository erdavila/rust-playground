use core::cmp::Ordering;

#[derive(Debug)]
pub enum Relative<K> {
    LeftInfinity,
    Before(K),
    At(K),
    After(K),
    RightInfinity,
}
impl<K> Relative<K> {
    pub fn as_ref(&self) -> Relative<&K> {
        match self {
            Relative::LeftInfinity => Relative::LeftInfinity,
            Relative::Before(x) => Relative::Before(x),
            Relative::At(x) => Relative::At(x),
            Relative::After(x) => Relative::After(x),
            Relative::RightInfinity => Relative::RightInfinity,
        }
    }
}
impl<K: Ord> PartialEq for Relative<K> {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other).is_eq()
    }
}
impl<K: Ord> Eq for Relative<K> {}
impl<K: Ord> PartialOrd for Relative<K> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl<K: Ord> Ord for Relative<K> {
    fn cmp(&self, other: &Self) -> Ordering {
        #[expect(clippy::unnested_or_patterns)]
        match (self, other) {
            (Relative::Before(this), Relative::At(other))
            | (Relative::Before(this), Relative::After(other))
            | (Relative::At(this), Relative::After(other)) => this.cmp(other).ne_or_less(),

            (Relative::Before(this), Relative::Before(other))
            | (Relative::At(this), Relative::At(other))
            | (Relative::After(this), Relative::After(other)) => this.cmp(other),

            (Relative::At(this), Relative::Before(other))
            | (Relative::After(this), Relative::Before(other))
            | (Relative::After(this), Relative::At(other)) => this.cmp(other).ne_or_greater(),

            (Relative::LeftInfinity, Relative::LeftInfinity)
            | (Relative::RightInfinity, Relative::RightInfinity) => Ordering::Equal,

            (Relative::LeftInfinity, _) | (_, Relative::RightInfinity) => Ordering::Less,

            (_, Relative::LeftInfinity) | (Relative::RightInfinity, _) => Ordering::Greater,
        }
    }
}

impl<K> PartialEq<Relative<&K>> for Relative<K> {
    fn eq(&self, other: &Relative<&K>) -> bool {
        todo!()
    }
}
impl<K: Ord> PartialOrd<Relative<&K>> for Relative<K> {
    fn partial_cmp(&self, other: &Relative<&K>) -> Option<Ordering> {
        self.as_ref().partial_cmp(other)
    }
}

impl<K> PartialEq<Relative<K>> for Relative<&K> {
    fn eq(&self, other: &Relative<K>) -> bool {
        todo!()
    }
}
impl<K> PartialOrd<Relative<K>> for Relative<&K> {
    fn partial_cmp(&self, other: &Relative<K>) -> Option<Ordering> {
        todo!()
    }
}

impl<K> PartialEq<K> for Relative<K> {
    fn eq(&self, other: &K) -> bool {
        todo!()
    }
}
impl<K: Ord> PartialOrd<K> for Relative<K> {
    fn partial_cmp(&self, other: &K) -> Option<Ordering> {
        self.partial_cmp(&Relative::At(other))
    }
}

trait OrderingExt {
    fn ne_or_less(self) -> Ordering;
    fn ne_or_greater(self) -> Ordering;
}
impl OrderingExt for Ordering {
    fn ne_or_less(self) -> Ordering {
        if self.is_ne() { self } else { Ordering::Less }
    }

    fn ne_or_greater(self) -> Ordering {
        if self.is_ne() {
            self
        } else {
            Ordering::Greater
        }
    }
}
