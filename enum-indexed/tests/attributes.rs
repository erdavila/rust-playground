use enum_indexed::indexed_struct::IndexedStruct;

#[test]
fn struct_name() {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, IndexedStruct)]
    #[indexed_struct(name = "MyIndexedStruct")]
    enum MyEnum {
        A,
        B,
        C,
    }

    let _ = MyIndexedStruct { a: 1, b: 2, c: 3 };
}

#[test]
fn additional_derives() {
    fn check_ord(_: impl Ord) {}

    // Single option.
    {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, IndexedStruct)]
        #[indexed_struct(derive(PartialOrd, Ord))]
        enum MyEnum {
            A,
            B,
            C,
        }

        let my_struct = MyEnumIndexed { a: 1, b: 2, c: 3 };

        check_ord(my_struct);
    }

    // Multiple options in single attribute.
    {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, IndexedStruct)]
        #[indexed_struct(derive(PartialOrd), derive(Ord))]
        enum MyEnum {
            A,
            B,
            C,
        }

        let my_struct = MyEnumIndexed { a: 1, b: 2, c: 3 };

        check_ord(my_struct);
    }

    // Multiple attributes.
    {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, IndexedStruct)]
        #[indexed_struct(derive(PartialOrd))]
        #[indexed_struct(derive(Ord))]
        enum MyEnum {
            A,
            B,
            C,
        }

        let my_struct = MyEnumIndexed { a: 1, b: 2, c: 3 };

        check_ord(my_struct);
    }
}

#[test]
fn skip_variant() {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, IndexedStruct)]
    enum MyEnum {
        Default,

        #[indexed_struct(skip = true)]
        SkipTrue,

        #[indexed_struct(skip = false)]
        SkipFalse,

        #[indexed_struct(skip)]
        Skip,
    }

    let _ = MyEnumIndexed {
        default: 1,
        skip_false: 2,
    };

    let _ = MyEnum::SkipTrue;
    let _ = MyEnum::Skip;
}

#[test]
fn field_name() {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, IndexedStruct)]
    enum MyEnum {
        A,
        #[indexed_struct(field = "some_name")]
        B,
        C,
    }

    let _ = MyEnumIndexed {
        a: 1,
        some_name: 2,
        c: 3,
    };
}

#[test]
fn field_attr() {
    // Single option.
    {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, IndexedStruct)]
        enum MyEnum {
            A,
            #[indexed_struct(attr(non_exhaustive, allow(unused)))]
            B,
            C,
        }
    }

    // Multiple options in single attribute.
    {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, IndexedStruct)]
        enum MyEnum {
            A,
            #[indexed_struct(attr(non_exhaustive), attr(allow(unused)))]
            B,
            C,
        }
    }

    // Multiple attributes.
    {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, IndexedStruct)]
        enum MyEnum {
            A,
            #[indexed_struct(attr(non_exhaustive))]
            #[indexed_struct(attr(allow(unused)))]
            B,
            C,
        }
    }
}
