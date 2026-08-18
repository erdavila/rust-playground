use enum_indexed::indexed_struct::IndexedStruct;

#[test]
fn multi_word_variant() {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, IndexedStruct)]
    enum MyEnum {
        MultiWordVariant,
    }

    let _ = MyEnumIndexed {
        multi_word_variant: 1,
    };
}

#[test]
fn type_args_variants() {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, IndexedStruct)]
    enum MyEnum {
        A,
        B,
        T,
        U,
        E,
        X,
    }

    let _ = MyEnumIndexed {
        a: 1,
        b: 2,
        t: 3,
        u: 4,
        e: 5,
        x: 6,
    };
}

#[test]
fn type_arg() {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, IndexedStruct)]
    enum T {
        Y,
        Z,
    }

    let _ = TIndexed { y: 1, z: 2 };
}

#[test]
fn type_arg_2() {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, IndexedStruct)]
    enum U {
        Y,
        Z,
    }

    let _ = UIndexed { y: 1, z: 2 };
}

#[test]
fn err_type_arg() {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, IndexedStruct)]
    enum E {
        Y,
        Z,
    }

    let _ = EIndexed { y: 1, z: 2 };
}

#[test]
fn type_arg_alternative() {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, IndexedStruct)]
    enum A {
        Y,
        Z,
    }

    let _ = AIndexed { y: 1, z: 2 };
}

#[test]
fn type_arg_2_alternative() {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, IndexedStruct)]
    enum B {
        Y,
        Z,
    }

    let _ = BIndexed { y: 1, z: 2 };
}

#[test]
fn err_type_arg_alternative() {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, IndexedStruct)]
    enum X {
        Y,
        Z,
    }

    let _ = XIndexed { y: 1, z: 2 };
}
