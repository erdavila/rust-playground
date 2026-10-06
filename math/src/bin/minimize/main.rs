#![feature(generic_const_exprs)]

mod complex;
mod derivatives_check;
mod modulo_squared;
mod real;
mod vectorize;

use std::fmt::{self, Debug, Display};
use std::ops::{Add, Mul, Sub};

use math::{Apply, DefaultVariableName, Division, Function, Subtraction, TypedConst, Variable};

use crate::complex::Complex;
use crate::modulo_squared::{ModSqr, ModuloSquared};
use crate::vectorize::{Gradient, Vectorize, Vectorized};

/*
    x + y = 6
    xy = 36
*/

type Var = Variable<0, DefaultVariableName<0>>;
static VAR: Var = Variable::new0();

type Y1 = Subtraction<TypedConst<f64>, Var>;
static Y1: Y1 = Subtraction::new(TypedConst { value: 6.0 }, VAR);

type Y2 = Division<TypedConst<f64>, Var>;
static Y2: Y2 = Division::new(TypedConst { value: 36.0 }, VAR);

type Diff = Subtraction<Y1, Y2>;
static DIFF: Diff = Subtraction::new(Y1, Y2);

type Error = ModuloSquared<Diff>;
static ERROR: Error = ModuloSquared::new(DIFF);

fn main() {
    if true {
        derivatives_check::check();
    } else {
        println!("y1 = {Y1}");
        println!("y2 = {Y2}");
        println!("diff = {DIFF}");
        println!("error = {ERROR}");
    }

    println!();
    naive_descent();
    println!();
    gradient_descent();
}

fn reals() -> impl Iterator<Item = f64> {
    (-20..=20).step_by(2).map(f64::from)
}

fn naive_descent() {
    descent("Naive", &NaiveDescent, &NaiveDescent);
}

fn gradient_descent() {
    descent("Gradient", &GradientDescent::new(), &GradientDescent::new());
}

fn descent(name: &str, real: &impl Descent<f64>, complex: &impl Descent<Complex<f64>>) {
    println!("=== {name} Descent ===");
    println!();

    println!("== Real ==");
    let initial_reals = reals();
    real.solve(initial_reals);

    println!("== Complex ==");
    let initial_complexes =
        reals().flat_map(|real| reals().map(move |imag| Complex::new(real, imag)));
    complex.solve(initial_complexes);
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Minimum<T> {
    point: Point<T>,
    initial: T,
    turns: u16,
    status: Status,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Status {
    MaxTurns,
    ZeroError,
    Saturated,
}

trait Descent<T>
where
    T: Copy + Sub<T, Output = T> + ModSqr<Output = f64> + Display,
{
    fn minimize(&self, x: T) -> Minimum<T>;

    fn solve(&self, initial_values: impl IntoIterator<Item = T>)
    where
        T: Add<T, Output = T> + Mul<T, Output = T>,
        Y1: Apply<T, Output = T>,
        Y2: Apply<T, Output = T>,
    {
        let mut minimums = Vec::new();

        for x in initial_values {
            let minimum = self.minimize(x);
            Self::add_minimum(minimum, &mut minimums);
        }

        assert!(!minimums.is_empty());
        for Minimum {
            point: Point { x, error },
            initial: initial_value,
            turns,
            status,
        } in minimums
        {
            let y1 = Y1.apply(x);
            let y2 = Y2.apply(x);

            println!(
                "  x = {x:.5}, e = {error:?}, from {initial_value} with {turns} turns ({status:?})"
            );
            println!("    y1 = {y1:.5}");
            println!("      x * y1 = {:.5}", x * y1);
            println!("    y2 = {y2:.5}");
            println!("      x + y2 = {:.5}", x + y2);
        }
    }

    fn add_minimum(new: Minimum<T>, minimums: &mut Vec<Minimum<T>>) {
        const DISTANCE: f64 = 1.0;

        if new.point.error.is_nan() {
            return;
        }

        for min in minimums.iter_mut() {
            let distance = (new.point.x - min.point.x).mod_sqr();
            if distance < DISTANCE {
                if new.point.error < min.point.error {
                    *min = new;
                }
                return;
            }
        }

        minimums.push(new);
    }
}

trait NaiveDescentDeltas: Sized
where
    Self: 'static,
{
    const NAIVE_DESCENT_DELTAS: &[Self];
}
impl NaiveDescentDeltas for f64 {
    const NAIVE_DESCENT_DELTAS: &[Self] = &[-1.0, 1.0];
}
impl NaiveDescentDeltas for Complex<f64> {
    const NAIVE_DESCENT_DELTAS: &[Self] = &[
        Complex::new(-1.0, 0.0),
        Complex::new(0.0, -1.0),
        Complex::new(1.0, 0.0),
        Complex::new(0.0, 1.0),
    ];
}

struct NaiveDescent;
impl<T> Descent<T> for NaiveDescent
where
    T: Copy
        + Add<T, Output = T>
        + Sub<T, Output = T>
        + Mul<f64, Output = T>
        + ModSqr<Output = f64>
        + NaiveDescentDeltas
        + Display,
    Error: Apply<T, Output = f64>,
{
    fn minimize(&self, initial: T) -> Minimum<T> {
        const TURNS: u16 = 100;

        let mut point = Point::new(initial);
        let mut turns = 0;

        let mut delta_len = 1.0;
        let mut last_index = 0;

        'turns: for t in 0..TURNS {
            for i in 0..T::NAIVE_DESCENT_DELTAS.len() {
                let index = (last_index + i) % T::NAIVE_DESCENT_DELTAS.len();
                let x = point.x + T::NAIVE_DESCENT_DELTAS[index] * delta_len;
                let p = Point::new(x);

                if p.error < point.error {
                    point = p;
                    turns = t + 1;

                    if p.error == 0.0 {
                        return Minimum {
                            point,
                            initial,
                            turns,
                            status: Status::ZeroError,
                        };
                    }

                    last_index = index;
                    continue 'turns;
                }
            }

            delta_len /= 2.0;
        }

        Minimum {
            point,
            initial,
            turns,
            status: Status::MaxTurns,
        }
    }
}

struct GradientDescent<T>
where
    Error: Vectorized<T>,
    <Error as Vectorized<T>>::Vectorized: Gradient<T>,
{
    error_grad: <<Error as Vectorized<T>>::Vectorized as Gradient<T>>::Gradient,
}
impl<T> GradientDescent<T>
where
    Error: Vectorized<T>,
    <Error as Vectorized<T>>::Vectorized: Gradient<T>,
{
    fn new() -> Self {
        let error_vec = ERROR.vectorize::<T>();
        let error_grad = error_vec.gradient();
        GradientDescent { error_grad }
    }
}
impl<T, EV, EG> Descent<T> for GradientDescent<T>
where
    T: PartialEq + Sub<T, Output = T> + ModSqr<Output = f64> + Copy + Display,
    Error: Apply<T, Output = f64> + Vectorized<T, Vectorized = EV>,
    EV: Gradient<T, Gradient = EG>,
    EG: Apply<T, Output = T>,
    f64: Mul<T, Output = T>,
{
    fn minimize(&self, initial: T) -> Minimum<T> {
        const STEP: f64 = 0.2;

        const TURNS: u16 = 100;

        let mut point = Point::new(initial);
        let mut turns = 0;
        for t in 0..TURNS {
            let x = point.x - STEP * self.error_grad.apply(point.x);
            if x == point.x {
                return Minimum {
                    point,
                    initial,
                    turns,
                    status: Status::Saturated,
                };
            }
            point = Point::new(x);
            turns = t + 1;
            if point.error == 0.0 {
                return Minimum {
                    point,
                    initial,
                    turns,
                    status: Status::ZeroError,
                };
            }
        }

        Minimum {
            point,
            initial,
            turns,
            status: Status::MaxTurns,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Point<T> {
    x: T,
    error: f64,
}
impl<T> Point<T> {
    fn new(x: T) -> Self
    where
        T: Copy,
        Error: Apply<T, Output = f64>,
    {
        Point {
            x,
            error: ERROR.apply(x),
        }
    }
}
impl<T: Display> Display for Point<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Point {{ error({}) = {} }}", self.x, self.error)
    }
}
