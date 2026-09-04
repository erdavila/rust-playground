use std::collections::{HashMap, HashSet};
use std::env;

// https://en.wikipedia.org/wiki/Happy_number

#[derive(Clone, Copy)]
struct Solution {
    is_happy: bool,
    from_previous: u32,
}

fn main() {
    let (max, base) = process_args();

    let mut solutions = HashMap::new();

    for num in 1..=max {
        Evaluator::new(num, base, &mut solutions).evaluate();
    }
}

fn process_args() -> (u32, Base) {
    let mut args = env::args();
    args.next();

    let max = args
        .next()
        .expect("Missing arguments")
        .parse()
        .expect("Invalid argument");

    let base = args
        .next()
        .map_or(10, |x| x.parse().expect("Invalid base argument"));

    (max, Base(base))
}

struct Evaluator<'a> {
    num: u32,
    base: Base,
    seen: HashSet<u32>,
    solutions: &'a mut HashMap<u32, Solution>,
}
impl<'a> Evaluator<'a> {
    fn new(num: u32, base: Base, solutions: &'a mut HashMap<u32, Solution>) -> Self {
        Evaluator {
            num,
            base,
            seen: HashSet::new(),
            solutions,
        }
    }

    fn evaluate(mut self) {
        println!("{}", self.base.fmt(self.num));

        let mut n = self.num;
        loop {
            if n == 1 {
                return self.conclude(Conclusion::DefinitionOfHappiness);
            }

            if let Some(prev_sol) = self.solutions.get(&n).copied() {
                return self.conclude(Conclusion::FromPreviousSolution(prev_sol));
            }

            if self.seen.contains(&n) {
                return self.conclude(Conclusion::Loop);
            }

            self.seen.insert(n);

            let mut digits: Vec<_> = n.digits(self.base).collect();
            digits.reverse();
            print!(
                "  {} = ",
                digits
                    .iter()
                    .map(|d| format!("{d}²"))
                    .collect::<Vec<_>>()
                    .join(" + ")
            );

            let squares: Vec<_> = digits.iter().map(|d| d * d).collect();
            if squares.len() > 1 {
                print!(
                    "{} = ",
                    squares
                        .iter()
                        .map(ToString::to_string)
                        .collect::<Vec<_>>()
                        .join(" + ")
                );
            }

            n = squares.into_iter().sum();
            println!("{}", self.base.fmt(n));
        }
    }

    fn conclude(self, conclusion: Conclusion) {
        println!(
            "  {} happy! (by {})",
            if conclusion.is_happy() {
                "IS"
            } else {
                "is NOT"
            },
            conclusion.reason(),
        );

        let solution_num = Solution {
            is_happy: conclusion.is_happy(),
            from_previous: self.num,
        };
        for s in self.seen {
            self.solutions.insert(s, solution_num);
        }
    }
}

#[derive(Clone, Copy)]
enum Conclusion {
    DefinitionOfHappiness,
    FromPreviousSolution(Solution),
    Loop,
}
impl Conclusion {
    fn is_happy(self) -> bool {
        match self {
            Conclusion::DefinitionOfHappiness => true,
            Conclusion::FromPreviousSolution(solution) => solution.is_happy,
            Conclusion::Loop => false,
        }
    }

    fn reason(self) -> String {
        match self {
            Conclusion::DefinitionOfHappiness => String::from("definition of happiness"),
            Conclusion::FromPreviousSolution(solution) => {
                format!("solution of {}", solution.from_previous)
            }
            Conclusion::Loop => String::from("loop"),
        }
    }
}

#[derive(Clone, Copy)]
struct Base(u32);

impl Base {
    fn fmt(self, value: u32) -> String {
        if self.0 == 10 {
            format!("{value}")
        } else {
            let mut n = value;
            let mut output = Vec::new();
            loop {
                let d = n % self.0;
                n /= self.0;

                let c = std::char::from_digit(d, self.0).unwrap();
                output.push(c);

                if n == 0 {
                    break;
                }
            }

            format!("{value}[{}]", output.into_iter().rev().collect::<String>())
        }
    }
}

trait Digits {
    type Output: Iterator<Item = Self>;

    #[cfg(test)]
    fn digits_10(&self) -> Self::Output {
        self.digits(Base(10))
    }

    fn digits(&self, base: Base) -> Self::Output;
}

impl Digits for u32 {
    type Output = DigitsIter;

    fn digits(&self, base: Base) -> Self::Output {
        DigitsIter {
            value: Some(*self),
            base,
        }
    }
}

struct DigitsIter {
    value: Option<u32>,
    base: Base,
}
impl Iterator for DigitsIter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        match self.value {
            Some(0) => {
                self.value = None;
                Some(0)
            }
            Some(n) => {
                let digit = n % self.base.0;
                let rem = n / self.base.0;
                self.value = (rem != 0).then_some(rem);
                Some(digit)
            }
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Base, Digits};

    #[test]
    fn digits() {
        assert_eq!(0u32.digits_10().collect::<Vec<_>>(), vec![0]);
        assert_eq!(10u32.digits_10().collect::<Vec<_>>(), vec![0, 1]);
        assert_eq!(103u32.digits_10().collect::<Vec<_>>(), vec![3, 0, 1]);
    }

    #[test]
    fn base_fmt() {
        assert_eq!(Base(2).fmt(0), "0[0]".to_owned());
        assert_eq!(Base(2).fmt(1), "1[1]".to_owned());
        assert_eq!(Base(2).fmt(2), "2[10]".to_owned());
        assert_eq!(Base(2).fmt(3), "3[11]".to_owned());
        assert_eq!(Base(2).fmt(4), "4[100]".to_owned());

        assert_eq!(Base(5).fmt(0), "0[0]".to_owned());
        assert_eq!(Base(5).fmt(4), "4[4]".to_owned());
        assert_eq!(Base(5).fmt(5), "5[10]".to_owned());
        assert_eq!(Base(5).fmt(6), "6[11]".to_owned());
        assert_eq!(Base(5).fmt(9), "9[14]".to_owned());
        assert_eq!(Base(5).fmt(10), "10[20]".to_owned());
        assert_eq!(Base(5).fmt(11), "11[21]".to_owned());

        assert_eq!(Base(10).fmt(0), "0".to_owned());
        assert_eq!(Base(10).fmt(1), "1".to_owned());
        assert_eq!(Base(10).fmt(9), "9".to_owned());
        assert_eq!(Base(10).fmt(10), "10".to_owned());
        assert_eq!(Base(10).fmt(11), "11".to_owned());
        assert_eq!(Base(10).fmt(20), "20".to_owned());
    }
}
