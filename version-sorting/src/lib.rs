#![no_std]
use core::{cmp::Ordering, iter::Peekable, str::Chars};

// As specified in https://doc.rust-lang.org/style-guide/index.html#sorting.
#[must_use]
pub fn version_sorting(a: &&str, b: &&str) -> Ordering {
    let mut vs = VersionSorting::new(a, b);
    result_to_ordering(vs.compare())
}

type Result = core::result::Result<(), NonEqual>;

fn ordering_to_result(ord: Ordering) -> Result {
    match ord {
        Ordering::Equal => Ok(()),
        Ordering::Less => Err(NonEqual::Less),
        Ordering::Greater => Err(NonEqual::Greater),
    }
}

fn result_to_ordering(res: Result) -> Ordering {
    match res {
        Ok(()) => Ordering::Equal,
        Err(NonEqual::Less) => Ordering::Less,
        Err(NonEqual::Greater) => Ordering::Greater,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum NonEqual {
    Less,
    Greater,
}

impl NonEqual {
    fn reverse(self) -> Self {
        match self {
            NonEqual::Less => NonEqual::Greater,
            NonEqual::Greater => NonEqual::Less,
        }
    }
}

struct VersionSorting<'a> {
    a_chars: Peekable<Chars<'a>>,
    b_chars: Peekable<Chars<'a>>,
    leading_zeroes: Ordering,
}

impl<'a> VersionSorting<'a> {
    fn new(a: &'a str, b: &'a str) -> Self {
        VersionSorting {
            a_chars: a.chars().peekable(),
            b_chars: b.chars().peekable(),
            leading_zeroes: Ordering::Equal,
        }
    }

    fn compare(&mut self) -> Result {
        loop {
            if self.a_chars.peek().is_some_and(char::is_ascii_digit)
                && self.b_chars.peek().is_some_and(char::is_ascii_digit)
            {
                self.process_digits()?;
            } else {
                let a_char_opt = self.a_chars.next();
                let b_char_opt = self.b_chars.next();

                match (a_char_opt, b_char_opt) {
                    (Some(a_char), Some(b_char)) => {
                        if a_char == '_' {
                            Self::compare_underscore_to(b_char)?;
                        } else if b_char == '_' {
                            Self::compare_underscore_to(a_char).map_err(NonEqual::reverse)?;
                        } else {
                            ordering_to_result(a_char.cmp(&b_char))?;
                        }
                    }
                    _ => {
                        return ordering_to_result(
                            a_char_opt.cmp(&b_char_opt).then(self.leading_zeroes),
                        );
                    }
                }
            }
        }
    }

    fn process_digits(&mut self) -> Result {
        let a_zeroes = Self::skip_zeroes(&mut self.a_chars);
        let b_zeroes = Self::skip_zeroes(&mut self.b_chars);

        self.leading_zeroes = self.leading_zeroes.then(a_zeroes.cmp(&b_zeroes).reverse());

        let mut value_ord = Ordering::Equal;

        loop {
            let a_digit_opt = self.a_chars.next_if(char::is_ascii_digit);
            let b_digit_opt = self.b_chars.next_if(char::is_ascii_digit);

            match (a_digit_opt, b_digit_opt) {
                (Some(a_digit), Some(b_digit)) => {
                    value_ord = value_ord.then_with(|| a_digit.cmp(&b_digit));
                }
                _ => return ordering_to_result(a_digit_opt.cmp(&b_digit_opt).then(value_ord)),
            }
        }
    }

    fn skip_zeroes(chars: &mut Peekable<Chars<'a>>) -> usize {
        let mut zeroes = 0;

        while chars.next_if(|c| *c == '0').is_some() {
            zeroes += 1;
        }

        zeroes
    }

    fn compare_underscore_to(c: char) -> Result {
        let ord = match c {
            ' ' => Ordering::Greater,
            '_' => Ordering::Equal,
            _ => Ordering::Less,
        };

        ordering_to_result(ord)
    }
}
