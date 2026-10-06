use core::ops::{Add, Div, Mul, Sub};
use std::fmt::{self, Display, Write};

use math::util::Sqr;
use math::{Input, UniformInput, Zero};

use crate::real::Real;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Complex<R> {
    pub real: R,
    pub imag: R,
}

impl<R> Complex<R> {
    pub const fn new(real: R, imag: R) -> Self {
        Complex { real, imag }
    }
}

impl<R: Default> Default for Complex<R> {
    fn default() -> Self {
        Self {
            real: Default::default(),
            imag: Default::default(),
        }
    }
}

impl<R: Zero> From<R> for Complex<R> {
    fn from(value: R) -> Self {
        Complex {
            real: value,
            imag: R::ZERO,
        }
    }
}

// Complex<R> + Complex<S>
impl<R, S> Add<Complex<S>> for Complex<R>
where
    R: Add<S>,
{
    type Output = Complex<<R as Add<S>>::Output>;

    fn add(self, rhs: Complex<S>) -> Self::Output {
        Complex {
            real: self.real + rhs.real,
            imag: self.imag + rhs.imag,
        }
    }
}

// Complex<R> + (S: Real)
impl<R, S: Real> Add<S> for Complex<R>
where
    S: Into<Complex<S>>,
    Self: Add<Complex<S>>,
{
    type Output = <Self as Add<Complex<S>>>::Output;

    fn add(self, rhs: S) -> Self::Output {
        todo!()
    }
}

// Complex<R> - Complex<S>
impl<R, S> Sub<Complex<S>> for Complex<R>
where
    R: Sub<S>,
{
    type Output = Complex<<R as Sub<S>>::Output>;

    fn sub(self, rhs: Complex<S>) -> Self::Output {
        Complex {
            real: self.real - rhs.real,
            imag: self.imag - rhs.imag,
        }
    }
}

// Complex<R> - (S: Real)
impl<R, S: Real> Sub<S> for Complex<R>
where
    S: Into<Complex<S>>,
    Self: Sub<Complex<S>>,
{
    type Output = <Self as Sub<Complex<S>>>::Output;

    fn sub(self, rhs: S) -> Self::Output {
        todo!()
    }
}

// f64 - Complex<R>
impl<R> Sub<Complex<R>> for f64
where
    Complex<f64>: Sub<Complex<R>>,
{
    type Output = <Complex<f64> as Sub<Complex<R>>>::Output;

    fn sub(self, rhs: Complex<R>) -> Self::Output {
        Complex::from(self) - rhs
    }
}

// Complex<R> * Complex<S>
impl<R, S> Mul<Complex<S>> for Complex<R>
where
    R: Mul<S> + Clone,
    <R as Mul<S>>::Output: Sub + Add<Output = <<R as Mul<S>>::Output as Sub>::Output>,
    S: Clone,
{
    type Output = Complex<<<R as Mul<S>>::Output as Sub>::Output>;

    fn mul(self, rhs: Complex<S>) -> Self::Output {
        let Complex { real: a, imag: b } = self;
        let Complex { real: c, imag: d } = rhs;
        /*
           (a+bi).(c+di) = ac + adi + bci + bdi²
                         = (ac - bd) + (ad + bc)i
        */

        let real = a.clone() * c.clone() - b.clone() * d.clone();
        let imag = a * d + b * c;

        Complex { real, imag }
    }
}

// Complex<R> * (S: Real)
impl<R, S: Real> Mul<S> for Complex<R>
where
    R: Mul<S>,
    S: Clone,
{
    type Output = Complex<<R as Mul<S>>::Output>;

    fn mul(self, rhs: S) -> Self::Output {
        Complex {
            real: self.real * rhs.clone(),
            imag: self.imag * rhs,
        }
    }
}

// f64 * Complex<R>
impl<R> Mul<Complex<R>> for f64
where
    f64: Mul<R>,
{
    type Output = Complex<<f64 as Mul<R>>::Output>;

    fn mul(self, rhs: Complex<R>) -> Self::Output {
        Complex {
            real: self * rhs.real,
            imag: self * rhs.imag,
        }
    }
}

// Complex<R> / Complex<S>
impl<R, S> Div<Complex<S>> for Complex<R> {
    type Output = Self;

    fn div(self, rhs: Complex<S>) -> Self::Output {
        todo!()
    }
}

// Complex<R> / (S: Real)
impl<R, S: Real> Div<S> for Complex<R>
where
    R: Div<S>,
    S: Clone,
{
    type Output = Complex<<R as Div<S>>::Output>;

    fn div(self, rhs: S) -> Self::Output {
        Complex {
            real: self.real / rhs.clone(),
            imag: self.imag / rhs,
        }
    }
}

// f64 / Complex<R>
impl<R> Div<Complex<R>> for f64
where
    R: Sqr + Clone,
    <R as Sqr>::Output: Add,
    <<R as Sqr>::Output as Add>::Output: Clone,
    f64: Mul<R>,
    <f64 as Mul<R>>::Output: Div<<<R as Sqr>::Output as Add>::Output>,
{
    type Output =
        Complex<<<f64 as Mul<R>>::Output as Div<<<R as Sqr>::Output as Add>::Output>>::Output>;

    fn div(self, rhs: Complex<R>) -> Self::Output {
        let x = self;
        let Complex { real: a, imag: b } = rhs;
        /*
           x / (a+bi) = x.(a-bi) / [(a+bi).(a-bi)]
                      = xa - xbi / (a² + b²)
                      = xa/(a² + b²) - [xb/(a² + b²)]i
        */

        let divisor = a.clone().sqr() + b.clone().sqr();

        let real = x * a / divisor.clone();
        let imag = -x * b / divisor;

        Complex { real, imag }
    }
}

impl<R: Real + Display> Display for Complex<R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn is_positive_zero<R: Real>(n: &R) -> bool {
            n.is_zero() && n.is_sign_positive()
        }

        fn is_neg_nan_sign_needed<R: Real>(n: &R) -> bool {
            n.is_nan() && n.is_sign_negative() && !R::IS_NEG_NAN_SIGN_DISPLAYED
        }

        let display_imag = !is_positive_zero(&self.imag);
        let display_real = !is_positive_zero(&self.real) || !display_imag;

        let imag_sign = if is_neg_nan_sign_needed(&self.imag)
            || (self.imag.is_abs_one() && self.imag.is_sign_negative())
        {
            Some('-')
        } else if display_real && !self.imag.is_sign_negative() && !self.imag.is_zero() {
            Some('+')
        } else {
            None
        };
        let real_sign = is_neg_nan_sign_needed(&self.real).then_some('-');

        let display_imag_value = !self.imag.is_abs_one();

        if display_real {
            if let Some(sign) = real_sign {
                f.write_char(sign)?;
            }
            self.real.fmt(f)?;
        }
        if display_imag {
            if let Some(sign) = imag_sign {
                f.write_char(sign)?;
            }
            if display_imag_value {
                self.imag.fmt(f)?;
            }
            f.write_char('i')?;
        }

        Ok(())
    }
}

impl<R> Input for Complex<R> {}

impl<R> UniformInput for Complex<R> {
    type VariableType = Self;
}

#[cfg(test)]
mod tests {
    use super::*;

    mod complex {
        use super::*;

        #[test]
        fn display() {
            struct Expected {
                value: f64,
                sign: Option<char>,
                must_display: bool,
            }
            impl Expected {
                fn real_display(&self) -> impl Display {
                    self.display(false)
                }

                fn imag_display(&self) -> impl Display {
                    self.display(true)
                }

                fn display(&self, imag: bool) -> impl Display {
                    fmt::from_fn(move |f| {
                        if self.must_display {
                            if let Some(sign) = self.sign {
                                f.write_char(sign)?;
                            }

                            if self.value.is_nan() {
                                f.write_str("NaN")?;
                            } else if self.value.is_infinite() {
                                f.write_str("inf")?;
                            } else {
                                let abs = self.value.abs();
                                if !imag || !abs.is_abs_one() {
                                    abs.fmt(f)?;
                                }
                            }

                            if imag {
                                f.write_char('i')?;
                            }
                        }
                        Ok(())
                    })
                }
            }
            impl From<f64> for Expected {
                fn from(value: f64) -> Self {
                    let (sign, must_display) = if value.is_sign_negative() {
                        (Some('-'), true)
                    } else if value > 0.0 || value.is_nan() {
                        (Some('+'), true)
                    } else {
                        (None, false)
                    };

                    Expected {
                        value,
                        sign,
                        must_display,
                    }
                }
            }

            let values = [
                -f64::NAN,
                f64::NEG_INFINITY,
                -1.0,
                -0.0,
                0.0,
                1.0,
                f64::INFINITY,
                f64::NAN,
            ];

            for real in values {
                for imag in values {
                    let c = Complex::new(real, imag);

                    let mut expected_real = Expected::from(c.real);
                    let mut expected_imag = Expected::from(c.imag);

                    if !expected_imag.must_display {
                        expected_real.must_display = true;
                    }

                    if expected_real.sign == Some('+') {
                        expected_real.sign = None;
                    }
                    if expected_imag.sign == Some('+') && !expected_real.must_display {
                        expected_imag.sign = None;
                    }

                    let expected = format!(
                        "{}{}",
                        expected_real.real_display(),
                        expected_imag.imag_display()
                    );

                    let display = c.to_string();

                    assert_eq!(display, expected, "{c:?}");
                }
            }
        }
    }

    mod real_for_f64 {
        use super::*;

        #[test]
        fn is_zero() {
            assert!(!(-f64::NAN).is_zero());
            assert!(!f64::NEG_INFINITY.is_zero());
            assert!(!(-1.0_f64).is_zero());
            assert!((-0.0_f64).is_zero());
            assert!(0.0_f64.is_zero());
            assert!(!1.0_f64.is_zero());
            assert!(!f64::INFINITY.is_zero());
            assert!(!f64::NAN.is_zero());
        }
    }
}
