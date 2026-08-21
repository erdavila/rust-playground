use crate::enum_map::EnumMapKey;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Size {
    F32,
    F64,
}
impl EnumMapKey<2> for Size {
    fn all() -> [Self; 2] {
        [Size::F32, Size::F64]
    }
}
