mod convert;

use core::cmp::Ordering;
use core::fmt;
use core::fmt::Display;
use core::marker::PhantomData;

pub use convert::*;
use ops_traits_macros::ops_type;
use symbolic_calculus_macros::Function;

use crate::{Output, impl_ops_for};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperationAssociativity {
    LeftToRight,
    RightToLeft,
}

#[derive(Clone)]
pub struct MergedVarNames<'a, IL, IR> {
    iter_left: IL,
    iter_right: IR,
    next_left: Option<(usize, &'a dyn Display)>,
    next_right: Option<(usize, &'a dyn Display)>,
}
impl<IL, IR> MergedVarNames<'_, IL, IR> {
    pub const fn new(iter_left: IL, iter_right: IR) -> Self {
        MergedVarNames {
            iter_left,
            iter_right,
            next_left: None,
            next_right: None,
        }
    }
}
impl<'a, IL, IR> Iterator for MergedVarNames<'a, IL, IR>
where
    IL: Iterator<Item = (usize, &'a dyn Display)>,
    IR: Iterator<Item = (usize, &'a dyn Display)>,
{
    type Item = (usize, &'a dyn Display);

    fn next(&mut self) -> Option<Self::Item> {
        if self.next_left.is_none() {
            self.next_left = self.iter_left.next();
        }
        if self.next_right.is_none() {
            self.next_right = self.iter_right.next();
        }

        match (self.next_left, self.next_right) {
            (Some((l_num, _)), Some((r_num, _))) => match l_num.cmp(&r_num) {
                Ordering::Less => self.next_left.take(),
                Ordering::Equal => {
                    self.next_left = None;
                    self.next_right.take()
                }
                Ordering::Greater => self.next_right.take(),
            },
            (Some(_), None) => self.next_left.take(),
            (None, Some(_)) => self.next_right.take(),
            (None, None) => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Function)]
pub struct FixedType<T>(PhantomData<T>);
impl<T, I> Output<I> for FixedType<T> {
    type Output = T;
}
impl_ops_for!(FixedType<T>);

pub trait Sqr {
    type Output;

    fn sqr(self) -> Self::Output;
}
impl Sqr for f64 {
    type Output = ops_type!(f64 * _);

    fn sqr(self) -> Self::Output {
        self * self
    }
}

pub enum Alternative<T, U> {
    One(T),
    Two(U),
}
impl<T1, T2> Alternative<T1, T2> {
    pub fn first(value: T1) -> Self {
        Alternative::One(value)
    }

    pub fn second_last(value: T2) -> Self {
        Alternative::Two(value)
    }
}
impl<T1, T2, T3> Alternative<T1, Alternative<T2, T3>> {
    pub fn second(value: T2) -> Self {
        Alternative::Two(Alternative::first(value))
    }

    pub fn third_last(value: T3) -> Self {
        Alternative::Two(Alternative::second_last(value))
    }
}
impl<T1, T2, T3, T4> Alternative<T1, Alternative<T2, Alternative<T3, T4>>> {
    pub fn third(value: T3) -> Self {
        Alternative::Two(Alternative::second(value))
    }

    pub fn fourth_last(value: T4) -> Self {
        Alternative::Two(Alternative::third_last(value))
    }
}
impl<T: Display, U: Display> Display for Alternative<T, U> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Alternative::One(one) => one.fmt(f),
            Alternative::Two(two) => two.fmt(f),
        }
    }
}
impl<T, U, It> Iterator for Alternative<T, U>
where
    T: Iterator<Item = It>,
    U: Iterator<Item = It>,
{
    type Item = It;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Alternative::One(one) => one.next(),
            Alternative::Two(two) => two.next(),
        }
    }
}
