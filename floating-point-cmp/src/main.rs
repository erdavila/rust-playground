use std::cmp::Ordering;
use std::fmt::{self, Display};

mod ansi {
    const FOREGROUND: u8 = 30;

    pub(crate) const BOLD: u8 = 0b001;
    pub(crate) const GREEN: u8 = FOREGROUND + 0b010;
    pub(crate) const BLUE: u8 = FOREGROUND + 0b100;
    pub(crate) const CYAN: u8 = FOREGROUND + 0b110;
}

macro_rules! format_ansi {
    ($ansi_content:expr, $format:literal) => {{
        let ansi_content = $ansi_content;
        fmt::from_fn(move |f| {
            if let Some(code) = &ansi_content.ansi_code {
                write!(
                    f,
                    "{}{}{}",
                    ansi(code),
                    format_args!($format, ansi_content.content),
                    ansi(0),
                )
            } else {
                write!(f, $format, ansi_content.content)
            }
        })
    }};
}

fn ansi(code: impl Display) -> impl Display {
    fmt::from_fn(move |f| write!(f, "\x1b[{code}m"))
}

fn main() {
    let mut values: Vec<_> = [0.0, 1.0, f64::INFINITY, f64::NAN]
        .into_iter()
        .flat_map(|n| [-n, n])
        .collect();
    values.sort_by(f64::total_cmp);

    print_line(
        AnsiContent::<_, usize>::new_no_ansi(""),
        values
            .iter()
            .map(|value| AnsiContent::new(real_formatted(*value), ansi::BOLD)),
    );

    for x in &values {
        print_line(
            AnsiContent::new(real_formatted(*x), ansi::BOLD),
            values.iter().map(|y| match x.partial_cmp(y) {
                Some(Ordering::Less) => AnsiContent::new("<", ansi::BLUE),
                Some(Ordering::Equal) => AnsiContent::new("==", ansi::CYAN),
                Some(Ordering::Greater) => AnsiContent::new(">", ansi::GREEN),
                None => AnsiContent::new_no_ansi("·"),
            }),
        );
    }
    println!();
}

struct AnsiContent<C, A> {
    content: C,
    ansi_code: Option<A>,
}
impl<C, A> AnsiContent<C, A> {
    const fn new(content: C, ansi_code: A) -> Self {
        AnsiContent {
            content,
            ansi_code: Some(ansi_code),
        }
    }

    const fn new_no_ansi(content: C) -> Self {
        AnsiContent {
            content,
            ansi_code: None,
        }
    }
}

fn print_line(
    header: AnsiContent<impl Display, impl Display>,
    columns: impl IntoIterator<Item = AnsiContent<impl Display, impl Display>>,
) {
    print!("{}  ", format_ansi!(header, "{:>4}"));
    for column in columns {
        print!("{}", format_ansi!(column, "{:^7}"));
    }
    println!();
}

fn real_formatted(n: f64) -> String {
    let mut s = n.to_string();
    if n.is_sign_negative() && !s.starts_with('-') {
        s.insert(0, '-');
    }
    s
}
