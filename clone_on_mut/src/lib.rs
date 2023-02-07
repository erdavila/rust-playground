use std::borrow::{Borrow, BorrowMut};
use std::ops::{Deref, DerefMut};

pub enum CloneOnMut<'a, T>
where
    T: ToOwned + ?Sized,
{
    Borrowed(&'a T),
    Owned(<T as ToOwned>::Owned),
}
impl<T> CloneOnMut<'_, T>
where
    T: ToOwned + ?Sized,
{
    pub fn borrow(borrowed: &T) -> CloneOnMut<T> {
        CloneOnMut::Borrowed(borrowed)
    }

    pub fn own(owned: <T as ToOwned>::Owned) -> CloneOnMut<'static, T> {
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

    pub fn into_owned(mut self) -> <T as ToOwned>::Owned {
        self.ensure_owned();
        match self {
            CloneOnMut::Owned(owned) => owned,
            _ => unreachable!(),
        }
    }

    fn ensure_owned(&mut self) {
        if let CloneOnMut::Borrowed(borrowed) = self {
            let owned = borrowed.to_owned();
            *self = CloneOnMut::Owned(owned);
        };
    }
}
impl<T> Deref for CloneOnMut<'_, T>
where
    T: ToOwned + ?Sized,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self {
            CloneOnMut::Borrowed(borrowed) => borrowed,
            CloneOnMut::Owned(owned) => owned.borrow(),
        }
    }
}
impl<T> DerefMut for CloneOnMut<'_, T>
where
    T: ToOwned + ?Sized,
    <T as ToOwned>::Owned: BorrowMut<T>,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.ensure_owned();
        match self {
            CloneOnMut::Owned(owned) => owned.borrow_mut(),
            _ => unreachable!(),
        }
    }
}
impl<T> Clone for CloneOnMut<'_, T>
where
    T: ToOwned + ?Sized,
{
    fn clone(&self) -> Self {
        let borrowed = unsafe { &*(self.deref() as *const T) };
        CloneOnMut::Borrowed(borrowed)
    }
}
impl<T> Borrow<T> for CloneOnMut<'_, T>
where
    T: ToOwned + ?Sized,
{
    fn borrow(&self) -> &T {
        self.deref()
    }
}
impl<T> BorrowMut<T> for CloneOnMut<'_, T>
where
    T: ToOwned + ?Sized,
    <T as ToOwned>::Owned: BorrowMut<T>,
{
    fn borrow_mut(&mut self) -> &mut T {
        self.deref_mut()
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use super::*;

    struct Value {
        id: String,
        clone_count: RefCell<u32>,
        cloned_from_id: Option<String>,
    }
    impl Value {
        fn new(id: &str) -> Self {
            Value {
                id: id.to_string(),
                clone_count: RefCell::new(0),
                cloned_from_id: None,
            }
        }

        fn access(&self) {}

        fn access_mut(&mut self) {}
    }
    impl Clone for Value {
        fn clone(&self) -> Self {
            let sub_id = {
                let mut count = self.clone_count.borrow_mut();
                *count += 1;
                *count
            };

            Value {
                id: format!("{}.{}", self.id, sub_id),
                clone_count: RefCell::new(0),
                cloned_from_id: Some(self.id.clone()),
            }
        }
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

        let com: CloneOnMut<Value> = CloneOnMut::own(value);

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

        let com: CloneOnMut<Value> = CloneOnMut::own(value);

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
        let com: CloneOnMut<Value> = CloneOnMut::own(value);

        com.access();

        match com {
            CloneOnMut::Owned(owned) => assert_eq!(owned.id, "original"),
            _ => panic!("should be Owned(_)"),
        }
    }

    #[test]
    fn test_deref_mut_on_borrowed() {
        let value = Value::new("original");
        let mut com = CloneOnMut::borrow(&value);

        com.access_mut();

        match &com {
            CloneOnMut::Owned(owned) => assert_eq!(owned.cloned_from_id, Some(value.id)),
            _ => panic!("should be Owned(_)"),
        }
    }

    #[test]
    fn test_deref_mut_on_owned() {
        let value = Value::new("original");
        let mut com: CloneOnMut<Value> = CloneOnMut::own(value);

        com.access_mut();

        match &com {
            CloneOnMut::Owned(owned) => assert_eq!(owned.id, "original"),
            _ => panic!("should be Owned(_)"),
        }
    }

    #[test]
    fn test_clone_on_borrowed() {
        let value = Value::new("original");
        let com: CloneOnMut<Value> = CloneOnMut::borrow(&value);

        let clone = CloneOnMut::clone(&com);

        match clone {
            CloneOnMut::Borrowed(borrowed) => assert_eq!(borrowed.id, value.id),
            _ => panic!("should be Borrowed(_)"),
        }
    }

    #[test]
    fn test_clone_on_owned() {
        let value = Value::new("original");
        let com: CloneOnMut<Value> = CloneOnMut::own(value);

        let clone = CloneOnMut::clone(&com);

        match clone {
            CloneOnMut::Borrowed(borrowed) => assert_eq!(borrowed.id, "original"),
            _ => panic!("should be Borrowed(_)"),
        }
    }

    #[test]
    fn test_into_owned_on_borrowed() {
        let value = Value::new("original");
        let com: CloneOnMut<Value> = CloneOnMut::borrow(&value);

        let owned = com.into_owned();

        assert_eq!(owned.cloned_from_id, Some(value.id));
    }

    #[test]
    fn test_into_owned_on_owned() {
        let value = Value::new("original");
        let com: CloneOnMut<Value> = CloneOnMut::own(value);

        let owned = com.into_owned();

        assert_eq!(owned.id, "original");
    }
}
