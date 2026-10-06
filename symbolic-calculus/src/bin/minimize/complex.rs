use core::ops::{Add, Div, Mul, Sub};
use std::fmt::{self, Display, Write};
use std::ops::Neg;

use ops_traits_macros::{ops_bounds, ops_type};
use symbolic_calculus::util::Sqr;
use symbolic_calculus::{Input, One, Two, UniformInput, Zero};

use crate::modulo_squared::ModSqr;
use crate::real::Real;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Complex<R, I = R> {
    pub(crate) real: R,
    pub(crate) imag: I,
}

impl<R, I> Complex<R, I> {
    pub const fn new(real: R, imag: I) -> Self {
        Complex { real, imag }
    }

    pub fn from_tuple(tuple: (R, I)) -> Self {
        Complex {
            real: tuple.0,
            imag: tuple.1,
        }
    }

    pub fn into_tuple(self) -> (R, I) {
        (self.real, self.imag)
    }
}

impl<R, I> From<(R, I)> for Complex<R, I> {
    fn from(value: (R, I)) -> Self {
        Self::from_tuple(value)
    }
}

impl<R, I> From<Complex<R, I>> for (R, I) {
    fn from(value: Complex<R, I>) -> Self {
        value.into_tuple()
    }
}

impl<R: Default, I: Default> Default for Complex<R, I> {
    fn default() -> Self {
        Self {
            real: Default::default(),
            imag: Default::default(),
        }
    }
}

impl<R: Zero<Type = R>, I: Zero<Type = I>> Zero for Complex<R, I> {
    type Type = Self;
    const ZERO: Self::Type = Complex::new(R::ZERO, I::ZERO);
}

impl<R: One<Type = R>, I: Zero<Type = I>> One for Complex<R, I> {
    type Type = Self;
    const ONE: Self::Type = Complex::new(R::ONE, I::ZERO);
}

impl<R: Two<Type = R>, I: Zero<Type = I>> Two for Complex<R, I> {
    type Type = Self;
    const TWO: Self::Type = Complex::new(R::TWO, I::ZERO);
}

impl<R, I: Zero<Type = I>> From<R> for Complex<R, I> {
    fn from(value: R) -> Self {
        Complex {
            real: value,
            imag: I::ZERO,
        }
    }
}

pub(crate) struct RealWrapper<R>(pub(crate) R);

// -Complex<R, I>
#[ops_bounds(-R, -I)]
impl<R, I> Neg for Complex<R, I> {
    type Output = Complex<ops_type!(-R), ops_type!(-I)>;

    fn neg(self) -> Self::Output {
        let Complex { real: a, imag: b } = self;
        /*
           -(a+bi) = -a + (-b)i
        */
        Complex { real: -a, imag: -b }
    }
}

// Complex<R, I> + Complex<R2, I2>
#[ops_bounds(R + R2, I + I2)]
impl<R, I, R2, I2> Add<Complex<R2, I2>> for Complex<R, I> {
    type Output = Complex<ops_type!(R + R2), ops_type!(I + I2)>;

    fn add(self, rhs: Complex<R2, I2>) -> Self::Output {
        let Complex { real: a, imag: b } = self;
        let Complex { real: c, imag: d } = rhs;
        /*
           (a+bi) + (c+di) = (a+c) + (b+d)i
        */
        Complex {
            real: a + c,
            imag: b + d,
        }
    }
}

// Complex<R, I> + (S: Real)
#[ops_bounds(Complex<R, I> + RealWrapper<S>)]
impl<R, I, S: Real> Add<S> for Complex<R, I> {
    type Output = ops_type!(Complex<R, I> + RealWrapper<S>);

    fn add(self, rhs: S) -> Self::Output {
        self + RealWrapper(rhs)
    }
}

// Complex<R, I> + RealWrapper<S>
#[ops_bounds(R + S)]
impl<R, I, S> Add<RealWrapper<S>> for Complex<R, I> {
    type Output = Complex<ops_type!(R + S), I>;

    fn add(self, rhs: RealWrapper<S>) -> Self::Output {
        let Complex { real: a, imag: b } = self;
        let RealWrapper(x) = rhs;
        /*
           (a+bi) + x = (a+x) + bi
        */
        Complex {
            real: a + x,
            imag: b,
        }
    }
}

// f64 + Complex<R, I>
#[ops_bounds(RealWrapper<f64> + Complex<R, I>)]
impl<R, I> Add<Complex<R, I>> for f64 {
    type Output = ops_type!(RealWrapper<f64> + Complex<R, I>);

    fn add(self, rhs: Complex<R, I>) -> Self::Output {
        RealWrapper(self) + rhs
    }
}

// RealWrapper<S> + Complex<R, I>
#[ops_bounds(S + R)]
impl<S, R, I> Add<Complex<R, I>> for RealWrapper<S> {
    type Output = Complex<ops_type!(S + R), I>;

    fn add(self, rhs: Complex<R, I>) -> Self::Output {
        let RealWrapper(x) = self;
        let Complex { real: a, imag: b } = rhs;
        /*
           x + (a+bi) = (x+a) + bi
        */
        Complex {
            real: x + a,
            imag: b,
        }
    }
}

// Complex<R, I> - Complex<R2, I2>
#[ops_bounds(R - R2, I - I2)]
impl<R, I, R2, I2> Sub<Complex<R2, I2>> for Complex<R, I> {
    type Output = Complex<ops_type!(R - R2), ops_type!(I - I2)>;

    fn sub(self, rhs: Complex<R2, I2>) -> Self::Output {
        let Complex { real: a, imag: b } = self;
        let Complex { real: c, imag: d } = rhs;
        /*
           (a+bi) - (c+di) = (a-c) + (b-d)i
        */
        Complex {
            real: a - c,
            imag: b - d,
        }
    }
}

// Complex<R, I> - (S: Real)
#[ops_bounds(Complex<R, I> - RealWrapper<S>)]
impl<R, I, S: Real> Sub<S> for Complex<R, I> {
    type Output = ops_type!(Complex<R, I> - RealWrapper<S>);

    fn sub(self, rhs: S) -> Self::Output {
        self - RealWrapper(rhs)
    }
}

// Complex<R, I> - RealWrapper<S>
#[ops_bounds(R - S)]
impl<R, I, S: Real> Sub<RealWrapper<S>> for Complex<R, I> {
    type Output = Complex<ops_type!(R - S), I>;

    fn sub(self, rhs: RealWrapper<S>) -> Self::Output {
        let Complex { real: a, imag: b } = self;
        let RealWrapper(x) = rhs;
        /*
           (a+bi) - x = (a-x) + bi
        */
        Complex {
            real: a - x,
            imag: b,
        }
    }
}

// f64 - Complex<R, I>
#[ops_bounds(RealWrapper<f64> - Complex<R, I>)]
impl<R, I> Sub<Complex<R, I>> for f64 {
    type Output = ops_type!(RealWrapper<f64> - Complex<R, I>);

    fn sub(self, rhs: Complex<R, I>) -> Self::Output {
        RealWrapper(self) - rhs
    }
}

// RealWrapper<S> - Complex<R, I>
#[ops_bounds(S - R, -I)]
impl<S, R, I> Sub<Complex<R, I>> for RealWrapper<S> {
    type Output = Complex<ops_type!(S - R), ops_type!(-I)>;

    fn sub(self, rhs: Complex<R, I>) -> Self::Output {
        let RealWrapper(x) = self;
        let Complex { real: a, imag: b } = rhs;
        /*
           x - (a+bi) = (x-a) + (-b)i
        */
        Complex {
            real: x - a,
            imag: -b,
        }
    }
}

// Complex<R, I> * Complex<R2, I2>
#[ops_bounds(
    R * R2 - I * I2,
    R * I2 + I * R2,
    R: Clone, I: Clone, R2: Clone, I2: Clone,
)]
impl<R, I, R2, I2> Mul<Complex<R2, I2>> for Complex<R, I> {
    type Output = Complex<ops_type!(R * R2 - I * I2), ops_type!(R * I2 + I * R2)>;

    fn mul(self, rhs: Complex<R2, I2>) -> Self::Output {
        let Complex { real: a, imag: b } = self;
        let Complex { real: c, imag: d } = rhs;
        /*
           (a+bi).(c+di) = ac + adi + bci + bdi²
                         = (ac - bd) + (ad + bc)i
        */
        Complex {
            real: a.clone() * c.clone() - b.clone() * d.clone(),
            imag: a * d + b * c,
        }
    }
}

// Complex<R, I> * (S: Real)
#[ops_bounds(Complex<R, I> * RealWrapper<S>)]
impl<R, I, S: Real> Mul<S> for Complex<R, I> {
    type Output = ops_type!(Complex<R, I> * RealWrapper<S>);

    fn mul(self, rhs: S) -> Self::Output {
        self * RealWrapper(rhs)
    }
}

// Complex<R, I> * RealWrapper<S>
#[ops_bounds(R * S, I * S, S: Clone)]
impl<R, I, S: Real> Mul<RealWrapper<S>> for Complex<R, I> {
    type Output = Complex<ops_type!(R * S), ops_type!(I * S)>;

    fn mul(self, rhs: RealWrapper<S>) -> Self::Output {
        let Complex { real: a, imag: b } = self;
        let RealWrapper(x) = rhs;
        /*
           (a+bi)x = ax + bxi
        */
        Complex {
            real: a * x.clone(),
            imag: b * x,
        }
    }
}

// f64 * Complex<R, I>
#[ops_bounds(RealWrapper<f64> * Complex<R, I>)]
impl<R, I> Mul<Complex<R, I>> for f64 {
    type Output = ops_type!(RealWrapper<f64> * Complex<R, I>);

    fn mul(self, rhs: Complex<R, I>) -> Self::Output {
        RealWrapper(self) * rhs
    }
}

// RealWrapper<S> * Complex<R, I>
#[ops_bounds(S * R, S * I, S: Clone)]
impl<S, R, I> Mul<Complex<R, I>> for RealWrapper<S> {
    type Output = Complex<ops_type!(S * R), ops_type!(S * I)>;

    fn mul(self, rhs: Complex<R, I>) -> Self::Output {
        let RealWrapper(x) = self;
        let Complex { real: a, imag: b } = rhs;
        /*
           x(a+bi) = xa + xbi
        */
        Complex {
            real: x.clone() * a,
            imag: x * b,
        }
    }
}

// Complex<R, I> / Complex<R2, I2>
#[ops_bounds(
    R2 ^ 2 + I2 ^ 2 = D,
    (R * R2 + I * I2) / D,
    (I * R2 - R * I2) / D,
    R: Clone, I: Clone, R2: Clone, I2: Clone, D: Clone,
)]
impl<R, I, R2, I2, D> Div<Complex<R2, I2>> for Complex<R, I> {
    type Output = Complex<ops_type!((R * R2 + I * I2) / D), ops_type!((I * R2 - R * I2) / D)>;

    fn div(self, rhs: Complex<R2, I2>) -> Self::Output {
        let Complex { real: a, imag: b } = self;
        let Complex { real: c, imag: d } = rhs;
        /*
           self   a+bi   (a+bi)(c-di)   ac + bd   bc - ad
           ---- = ---- = ------------ = ------- + -------i
           rhs    c+di   (c+di)(c-di)   c² + d²   c² + d²
        */
        let denominator = c.clone().sqr() + d.clone().sqr();
        Complex {
            real: (a.clone() * c.clone() + b.clone() * d.clone()) / denominator.clone(),
            imag: (b * c - a * d) / denominator,
        }
    }
}

// Complex<R, I> / (S: Real)
#[ops_bounds(Complex<R, I> / RealWrapper<S>)]
impl<R, I, S: Real> Div<S> for Complex<R, I> {
    type Output = ops_type!(Complex<R, I> / RealWrapper<S>);

    fn div(self, rhs: S) -> Self::Output {
        self / RealWrapper(rhs)
    }
}

// Complex<R, I> / RealWrapper<S>
#[ops_bounds(R / S, I / S, S: Clone)]
impl<R, I, S: Real> Div<RealWrapper<S>> for Complex<R, I> {
    type Output = Complex<ops_type!(R / S), ops_type!(I / S)>;

    fn div(self, rhs: RealWrapper<S>) -> Self::Output {
        let Complex { real: a, imag: b } = self;
        let RealWrapper(x) = rhs;
        /*
           a+bi    a     b
           ---- = --- + ---i
            x      x     x
        */
        Complex {
            real: a / x.clone(),
            imag: b / x,
        }
    }
}

// f64 / Complex<R, I>
#[ops_bounds(RealWrapper<f64> / Complex<R, I>)]
impl<R, I> Div<Complex<R, I>> for f64 {
    type Output = ops_type!(RealWrapper<f64> / Complex<R, I>);

    fn div(self, rhs: Complex<R, I>) -> Self::Output {
        RealWrapper(self) / rhs
    }
}

// RealWrapper<S> / Complex<R, I>
#[ops_bounds(
    R ^ 2 + I ^ 2 = D,
    S * R / D,
    -S * I / D,
    S: Clone, R: Clone, I: Clone, D: Clone,
)]
impl<S, R, I, D> Div<Complex<R, I>> for RealWrapper<S> {
    type Output = Complex<ops_type!(S * R / D), ops_type!(-S * I / D)>;

    fn div(self, rhs: Complex<R, I>) -> Self::Output {
        let RealWrapper(x) = self;
        let Complex { real: a, imag: b } = rhs;
        /*
             x          x.(a-bi)   xa - xbi     xa        -xb
            ---- = ------------- = -------- = ------- + ------- i
            a+bi   (a+bi).(a-bi)   a² + b²    a² + b²   a² + b²
        */
        let denominator = a.clone().sqr() + b.clone().sqr();
        Complex {
            real: x.clone() * a / denominator.clone(),
            imag: -x * b / denominator,
        }
    }
}

// Complex<R, I>²
#[ops_bounds(
    R ^ 2 - I ^ 2,
    // a * b
    R * I = T,
    // 2 * (a * b)
    T::Type * T,
    T: Two,
    R: Clone, I: Clone,
)]
impl<R, I, T> Sqr for Complex<R, I> {
    type Output = Complex<ops_type!(R ^ 2 - I ^ 2), ops_type!(T::Type * T)>;

    fn sqr(self) -> Self::Output {
        let Complex { real: a, imag: b } = self;
        /*
           (a+bi)² = a² + 2abi + (bi)² = (a² - b²) + (2ab)i
        */
        Complex {
            real: a.clone().sqr() - b.clone().sqr(),
            imag: T::TWO * (a * b),
        }
    }
}

// |Complex<R, I>|²
#[ops_bounds(R ^ 2 + I ^ 2)]
impl<R, I> ModSqr for Complex<R, I> {
    type Output = ops_type!(R ^ 2 + I ^ 2);

    fn mod_sqr(self) -> Self::Output {
        self.real.sqr() + self.imag.sqr()
    }
}

impl<R: Real + Display, I: Real + Display> Display for Complex<R, I> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn is_positive_zero<T: Real>(n: &T) -> bool {
            n.is_zero() && n.is_sign_positive()
        }

        fn is_neg_nan_sign_needed<T: Real>(n: &T) -> bool {
            n.is_nan() && n.is_sign_negative() && !T::IS_NEG_NAN_SIGN_DISPLAYED
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

impl<R, I> Input for Complex<R, I> {}

impl<R, I> UniformInput for Complex<R, I> {
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
