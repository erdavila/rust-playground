use core::fmt;
use std::cmp::Ordering;
use std::fmt::{Debug, Display};

use math::{Apply, Derivative, VariableNames};

use crate::vectorize::Vectorize;
use crate::{Complex, DIFF, ERROR, Function, Y1, Y2};

// TODO: remove statics
static REAL_VAR_NAME: &str = "x";
static COMPLEX_VAR_NAME: &str = "a+bi";
static COMPLEX_REAL_VAR_NAME: &str = "a";
static COMPLEX_IMAG_VAR_NAME: &str = "b";
static COMPLEX_VAR_NAMES: [&str; 2] = [COMPLEX_REAL_VAR_NAME, COMPLEX_IMAG_VAR_NAME];

macro_rules! print_indented {
    ($indentation:expr, $($tt:tt)*) => {{
        let indentation = $indentation;
        println!(
            "{}{}",
            fmt::from_fn(|f| {
                for _ in 0..indentation {
                    f.write_str("  ")?;
                }
                Ok(())
            }),
            format_args!($($tt)*),
        );
    }};
}

macro_rules! check_endomorphic_function {
    ($f:ident) => {
        let name = stringify!($f);
        println!("{name} = {}", $f);

        check_real_to_real_function!($f, name);
        check_complex_to_complex_function!($f, name);
    };
}

macro_rules! check_real_valued_function {
    ($f:ident) => {
        let name = stringify!($f);
        println!("{name} = {}", $f);

        check_real_to_real_function!($f, name);
        check_complex_to_real_function!($f, name);
    };
}

macro_rules! check_real_to_real_function {
    ($f:ident, $name:expr) => {
        print_function(&$f, $name, &[REAL_VAR_NAME], "R -> R", None, 1);

        let f_vec = $f.vectorize::<f64>();
        print_indented!(2, "vectorized:");
        print_function(&f_vec, $name, &[REAL_VAR_NAME], "R¹ -> R¹", None, 3);

        let f = f_vec.0;
        print_function(&f, $name, &[REAL_VAR_NAME], "R -> R", None, 4);
        check_one_var_derivative(&f, $name);
    };
}

macro_rules! check_complex_to_real_function {
    ($f:ident, $name:expr) => {
        print_function(
            &$f.rename_var_num::<0, _>(COMPLEX_VAR_NAME),
            $name,
            &[COMPLEX_VAR_NAME],
            "C -> R",
            None,
            1,
        );

        let f_vec = $f
            .vectorize::<Complex<f64>>()
            .rename_var_num::<0, _>(COMPLEX_REAL_VAR_NAME)
            .rename_var_num::<1, _>(COMPLEX_IMAG_VAR_NAME);
        print_indented!(2, "vectorized:");
        print_function(&f_vec, $name, &COMPLEX_VAR_NAMES, "R² -> R", None, 3);

        let f = f_vec.0;
        print_function(&f, $name, &COMPLEX_VAR_NAMES, "R² -> R", None, 4);
        check_two_vars_derivatives(&f, $name);
    };
}

macro_rules! check_complex_to_complex_function {
    ($f:ident, $name:expr) => {
        print_function(
            &$f.rename_var_num::<0, _>(COMPLEX_VAR_NAME),
            $name,
            &[COMPLEX_VAR_NAME],
            "C -> C",
            None,
            1,
        );

        let f_vec = $f
            .vectorize::<Complex<f64>>()
            .rename_var_num::<0, _>(COMPLEX_REAL_VAR_NAME)
            .rename_var_num::<1, _>(COMPLEX_IMAG_VAR_NAME);
        print_indented!(2, "vectorized:");
        let var_names = f_vec.var_names();
        print_function(&f_vec, $name, &COMPLEX_VAR_NAMES, "R² -> R²", None, 3);

        let f_real = f_vec.real;
        let f_real_name = format!("{}[r]", $name);
        print_function(
            &f_real,
            &f_real_name,
            &COMPLEX_VAR_NAMES,
            "R² -> R",
            None,
            4,
        );
        check_two_vars_derivatives(&f_real, &f_real_name);

        let f_imag = f_vec.imag;
        let f_imag_name = format!("{}[i]", $name);
        print_function(
            &f_imag,
            &f_imag_name,
            &COMPLEX_VAR_NAMES,
            "R² -> R",
            None,
            4,
        );
        check_two_vars_derivatives(&f_imag, &f_imag_name);
    };
}

pub(crate) fn check() {
    println!();
    check_endomorphic_function!(Y1);
    check_endomorphic_function!(Y2);
    check_endomorphic_function!(DIFF);
    check_real_valued_function!(ERROR);
}

fn check_one_var_derivative<F, FD>(f: &F, name: &str)
where
    F: Derivative<0, Derivative = FD> + Apply<f64, Output = f64>,
    FD: Apply<f64, Output = f64> + VariableNames + Display,
{
    let f_deriv = f.derivative::<0>();
    print_function(&f_deriv, name, &[REAL_VAR_NAME], "R -> R", Some(0), 5);

    check_one_var_fn_derivative(f.as_fn(), f_deriv.as_fn(), REAL_VAR_NAME);
}

fn check_two_vars_derivatives<F, FD0, FD1>(f: &F, name: &str)
where
    F: Apply<(f64, f64), Output = f64>
        + Derivative<0, Derivative = FD0>
        + Derivative<1, Derivative = FD1>,
    FD0: Apply<(f64, f64), Output = f64> + VariableNames + Display,
    FD1: Apply<(f64, f64), Output = f64> + VariableNames + Display,
{
    let f_deriv_0 = f.derivative::<0>();
    print_function(&f_deriv_0, name, &COMPLEX_VAR_NAMES, "R² -> R", Some(0), 5);

    let f_deriv_1 = f.derivative::<1>();
    print_function(&f_deriv_1, name, &COMPLEX_VAR_NAMES, "R² -> R", Some(1), 5);

    check_two_vars_fn_derivative(
        f.as_two_vars_fn(),
        f_deriv_0.as_two_vars_fn(),
        f_deriv_1.as_two_vars_fn(),
        COMPLEX_REAL_VAR_NAME,
        COMPLEX_IMAG_VAR_NAME,
    );
}

fn print_function<F>(
    func: &F,
    name: &str,
    var_names: &[&str],
    signature: &str,
    deriv_var_index: Option<usize>,
    indent: usize,
) where
    F: VariableNames + Display,
{
    fn ansi(code: impl Display) -> impl Display {
        fmt::from_fn(move |f| write!(f, "\u{1b}[{code}m"))
    }

    fn with_ansi(code: impl Display, content: impl Display) -> impl Display {
        fmt::from_fn(move |f| {
            ansi(&code).fmt(f)?;
            content.fmt(f)?;
            ansi(0).fmt(f)?;
            Ok(())
        })
    }

    let name_and_params = format_args!(
        "{name}({func_params})",
        name = with_ansi(1, name),
        func_params = fmt::from_fn(|f| {
            for (i, name) in var_names.iter().enumerate() {
                if i > 0 {
                    f.write_str(", ")?;
                }
                f.write_str(name)?;
            }
            Ok(())
        })
    );
    let name_and_params: &dyn Display = match deriv_var_index {
        Some(i) => &format_args!("d{name_and_params}/d{deriv_var}", deriv_var = var_names[i]),
        None => &name_and_params,
    };

    let signature = format_args!(": {signature}");
    let signature = { with_ansi(36, signature) };

    print_indented!(
        indent,
        "{name_and_params} {eq} {func} {signature}",
        eq = with_ansi(33, "="),
        func = func,
    );
}

fn check_one_var_fn_derivative(f: impl Fn(f64) -> f64, f_deriv: impl Fn(f64) -> f64, var: &str) {
    let mut ok = true;
    for value in reals() {
        if let Err(values) = check_derivative(&f, &f_deriv, value) {
            let Values {
                out_plus,
                out_minus,
                deriv_calculated,
                deriv,
                error,
            } = values;
            println!(
                "failed for {var}={value}: f({var}+)={out_plus}, f({var}-)={out_minus}, f'*({var})={deriv_calculated}, f'({var})={deriv}, e={error}"
            );
            ok = false;
        }
    }
    if ok {
        // print_indented!(6, "OK");
    }
}

fn check_two_vars_fn_derivative(
    f: impl Fn(f64, f64) -> f64,
    f_deriv_x: impl Fn(f64, f64) -> f64,
    f_deriv_y: impl Fn(f64, f64) -> f64,
    var_x: &str,
    var_y: &str,
) {
    fn fix_a(a: f64, f: impl Fn(f64, f64) -> f64) -> impl Fn(f64) -> f64 {
        move |b| f(a, b)
    }

    fn fix_b(b: f64, f: impl Fn(f64, f64) -> f64) -> impl Fn(f64) -> f64 {
        move |a| f(a, b)
    }

    let mut ok = true;
    for a in reals() {
        for b in reals() {
            if let Err(values) = check_derivative(fix_b(b, &f), fix_b(b, &f_deriv_x), a) {
                let Values {
                    out_plus,
                    out_minus,
                    deriv_calculated,
                    deriv,
                    error,
                } = values;
                println!(
                    "failed for ⟨{var_x}, {var_y}⟩=⟨{a}, {b}⟩: f({var_x}+, {var_y})={out_plus}, f({var_x}-, {var_y})={out_minus}, f'*({var_x}, {var_y})={deriv_calculated}, f'({var_x}, {var_y})={deriv}, e={error}"
                );
                ok = false;
            }

            if let Err(values) = check_derivative(fix_a(a, &f), fix_a(a, &f_deriv_y), b) {
                let Values {
                    out_plus,
                    out_minus,
                    deriv_calculated,
                    deriv,
                    error,
                } = values;
                println!(
                    "failed for ⟨{var_x}, {var_y}⟩=⟨{a}, {b}⟩: f({var_x}, {var_y}+)={out_plus}, f({var_x}, {var_y}-)={out_minus}, f'*({var_x}, {var_y})={deriv_calculated}, f'({var_x}, {var_y})={deriv}, e={error}"
                );
                ok = false;
            }
        }
    }
    if ok {
        // print_indented!(6, "OK");
    }
}

fn reals() -> impl Iterator<Item = f64> {
    (-10..=10).map(f64::from)
}

const STEP: f64 = 1e-6;
const TOLERANCE: f64 = 1e-4;

struct Values {
    out_plus: f64,
    out_minus: f64,
    deriv_calculated: f64,
    deriv: f64,
    error: f64,
}

fn check_derivative(
    f: impl Fn(f64) -> f64,
    f_deriv: impl Fn(f64) -> f64,
    value: f64,
) -> Result<(), Values> {
    let out_plus = f(value + STEP);
    let out_minus = f(value - STEP);
    let deriv_calculated = (out_plus - out_minus) / (2.0 * STEP);

    let deriv = f_deriv(value);

    let error = difference_error(deriv_calculated, deriv);

    if let Some(Ordering::Greater) = error.partial_cmp(&TOLERANCE) {
        Err(Values {
            out_plus,
            out_minus,
            deriv_calculated,
            deriv,
            error,
        })
    } else {
        Ok(())
    }
}

fn difference_error(value1: f64, value2: f64) -> f64 {
    (value1 - value2).abs()
        / [1.0, value1.abs(), value2.abs()]
            .into_iter()
            .max_by_key(|&n| TotalOrderF64(n))
            .unwrap()
}

#[derive(Debug, Clone, Copy)]
struct TotalOrderF64(f64);
impl Ord for TotalOrderF64 {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.total_cmp(&other.0)
    }
}
impl PartialOrd for TotalOrderF64 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Eq for TotalOrderF64 {}
impl PartialEq for TotalOrderF64 {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other).is_eq()
    }
}
