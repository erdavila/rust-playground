#[cfg(test)]
mod tests {
    #[test]
    fn renamed_crate() {
        use renamed_crate::indexed_struct::{EnumIndexed, IndexedStruct};

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, IndexedStruct)]
        #[indexed_struct(enum_indexed_crate = "renamed_crate")]
        enum MyEnum {
            A,
            B,
            C,
        }

        let _ = <MyEnumIndexed<String> as EnumIndexed<String, 3>>::from_fn(|variant| {
            format!("{variant:?}")
        });
    }
}
