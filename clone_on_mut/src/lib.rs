use std::ops::Deref;

pub enum CloneOnMut<'a, T> {
    Borrowed(&'a T),
    Owned(T),
}
impl<T> CloneOnMut<'_, T> {
    pub fn borrow(borrowed: &T) -> CloneOnMut<T> {
        CloneOnMut::Borrowed(borrowed)
    }

    pub fn own(owned: T) -> CloneOnMut<'static, T> {
        CloneOnMut::Owned(owned)
    }

    pub fn is_borrowed(&self) -> bool {
        match self {
            CloneOnMut::Borrowed(_) => true,
            CloneOnMut::Owned(_) => false,
        }
    }

    pub fn is_owned(&self) -> bool {
        !self.is_borrowed()
    }
}
impl<T> Deref for CloneOnMut<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self {
            CloneOnMut::Borrowed(borrowed) => borrowed,
            CloneOnMut::Owned(owned) => owned,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Value {
        id: String,
    }
    impl Value {
        fn new(id: &str) -> Self {
            Value { id: id.to_string() }
        }

        fn access(&self) {}
    }

    #[test]
    fn test_borrow() {
        let value = Value::new("original");

        let com = CloneOnMut::borrow(&value);

        match com {
            CloneOnMut::Borrowed(borrowed) => assert_eq!(borrowed.id, value.id),
            _ => panic!("should be Borrowed(_)"),
        }
    }

    #[test]
    fn test_own() {
        let value = Value::new("original");

        let com = CloneOnMut::own(value);

        match com {
            CloneOnMut::Owned(owned) => assert_eq!(owned.id, "original"),
            _ => panic!("should be Owned(_)"),
        }
    }

    #[test]
    fn test_is_borrowed() {
        let value = Value::new("original");

        let com = CloneOnMut::borrow(&value);

        assert!(com.is_borrowed());
        assert!(!com.is_owned());
    }

    #[test]
    fn test_is_owned() {
        let value = Value::new("original");

        let com = CloneOnMut::own(value);

        assert!(com.is_owned());
        assert!(!com.is_borrowed());
    }

    #[test]
    fn test_deref_on_borrowed() {
        let value = Value::new("original");
        let com = CloneOnMut::borrow(&value);

        com.access();

        match com {
            CloneOnMut::Borrowed(borrowed) => assert_eq!(borrowed.id, value.id),
            _ => panic!("should be Borrowed(_)"),
        }
    }

    #[test]
    fn test_deref_on_owned() {
        let value = Value::new("original");
        let com = CloneOnMut::own(value);

        com.access();

        match com {
            CloneOnMut::Owned(owned) => assert_eq!(owned.id, "original"),
            _ => panic!("should be Owned(_)"),
        }
    }
}
