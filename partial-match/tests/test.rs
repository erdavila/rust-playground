use partial_match::partial_match;

enum E {
    A,
    B(bool),
    C,
    D,
}

#[test]
fn partial() {
    macro_rules! apply_partial_match {
        ($e:expr) => {
            partial_match!(($e) {
                E::A => 1,
                E::B(b) if b => 2,
                E::C => {
                    let n = 3;
                    n
                }
            })
        };
    }

    assert_eq!(apply_partial_match!(E::A), Some(1));
    assert_eq!(apply_partial_match!(E::B(true)), Some(2));
    assert_eq!(apply_partial_match!(E::C), Some(3));
    assert_eq!(apply_partial_match!(E::D), None);
}

#[test]
fn total() {
    macro_rules! apply_partial_match {
        ($e:expr) => {
            partial_match!(($e) {
                E::A => 1,
                E::B(b) if b => 2,
                E::B(_) => 3,
                E::C => {
                    let n = 4;
                    n
                }
                E::D => 5
            })
        };
    }

    assert_eq!(apply_partial_match!(E::A), Some(1));
    assert_eq!(apply_partial_match!(E::B(true)), Some(2));
    assert_eq!(apply_partial_match!(E::B(false)), Some(3));
    assert_eq!(apply_partial_match!(E::C), Some(4));
    assert_eq!(apply_partial_match!(E::D), Some(5));
}
