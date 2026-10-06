pub(crate) trait Real {
    const IS_NEG_NAN_SIGN_DISPLAYED: bool;

    fn is_zero(&self) -> bool;
    fn is_abs_one(&self) -> bool;
    fn is_nan(&self) -> bool;

    fn is_sign_negative(&self) -> bool;
    fn is_sign_positive(&self) -> bool;
}
impl Real for f64 {
    const IS_NEG_NAN_SIGN_DISPLAYED: bool = false;

    fn is_zero(&self) -> bool {
        *self == 0.0
    }

    #[expect(clippy::float_cmp)]
    fn is_abs_one(&self) -> bool {
        self.abs() == 1.0
    }

    fn is_nan(&self) -> bool {
        f64::is_nan(*self)
    }

    fn is_sign_negative(&self) -> bool {
        f64::is_sign_negative(*self)
    }

    fn is_sign_positive(&self) -> bool {
        f64::is_sign_positive(*self)
    }
}
