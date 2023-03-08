use std::iter;
use std::ops::{Add, AddAssign, Mul};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Position {
    pub row: i8,
    pub col: i8,
}

impl Position {
    pub fn iter(&self, direction: PositionDelta) -> impl Iterator<Item = Position> {
        iter::successors(Some(*self), move |p| Some(*p + direction))
    }
}

#[derive(Clone, Copy)]
pub struct PositionDelta {
    rows: i8,
    cols: i8,
}

impl PositionDelta {
    pub fn to_left() -> Self {
        PositionDelta {
            rows: 0,
            cols: Self::left_cols_delta(),
        }
    }
    pub fn to_right() -> Self {
        PositionDelta {
            rows: 0,
            cols: Self::right_cols_delta(),
        }
    }

    pub fn to_top() -> Self {
        PositionDelta {
            rows: Self::top_rows_delta(),
            cols: 0,
        }
    }

    pub fn to_bottom() -> Self {
        PositionDelta {
            rows: Self::bottom_rows_delta(),
            cols: 0,
        }
    }

    pub fn to_top_right() -> Self {
        PositionDelta {
            rows: Self::top_rows_delta(),
            cols: Self::right_cols_delta(),
        }
    }

    pub fn to_bottom_right() -> Self {
        PositionDelta {
            rows: Self::bottom_rows_delta(),
            cols: Self::right_cols_delta(),
        }
    }

    fn left_cols_delta() -> i8 {
        -1
    }

    fn top_rows_delta() -> i8 {
        -1
    }

    fn right_cols_delta() -> i8 {
        1
    }

    fn bottom_rows_delta() -> i8 {
        1
    }
}

impl Mul<i8> for PositionDelta {
    type Output = PositionDelta;

    fn mul(self, rhs: i8) -> Self::Output {
        PositionDelta {
            rows: self.rows * rhs,
            cols: self.cols * rhs,
        }
    }
}

impl Add<PositionDelta> for Position {
    type Output = Position;

    fn add(self, delta: PositionDelta) -> Self::Output {
        Position {
            row: self.row + delta.rows,
            col: self.col + delta.cols,
        }
    }
}

impl AddAssign<PositionDelta> for Position {
    fn add_assign(&mut self, delta: PositionDelta) {
        *self = *self + delta;
    }
}
