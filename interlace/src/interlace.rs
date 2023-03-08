use crate::position::Position;

pub struct Interlace {
    pub n1: u8,
    pub n2: u8,
}

impl Interlace {
    pub fn new(n1: u8, n2: u8) -> Self {
        Interlace { n1, n2 }
    }

    pub fn required_grid_size(&self) -> u8 {
        3 + self.n1 + self.n2
    }

    pub fn left_corner(&self) -> Position {
        Position {
            row: (self.n1 + 1) as i8,
            col: 0,
        }
    }

    pub fn top_corner(&self) -> Position {
        Position {
            row: 0,
            col: (self.n1 + 1) as i8,
        }
    }

    pub fn right_corner(&self) -> Position {
        Position {
            row: (self.n2 + 1) as i8,
            col: (self.n1 + self.n2 + 2) as i8,
        }
    }

    pub fn bottom_corner(&self) -> Position {
        Position {
            row: (self.n1 + self.n2 + 2) as i8,
            col: (self.n2 + 1) as i8,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn required_grid_size() {
        assert_eq!(Interlace::new(0, 0).required_grid_size(), 3);
        assert_eq!(Interlace::new(2, 5).required_grid_size(), 10);
    }

    #[test]
    fn corners() {
        let interlace = Interlace::new(0, 0);
        assert_eq!(interlace.left_corner(), Position { row: 1, col: 0 });
        assert_eq!(interlace.top_corner(), Position { row: 0, col: 1 });
        assert_eq!(interlace.right_corner(), Position { row: 1, col: 2 });
        assert_eq!(interlace.bottom_corner(), Position { row: 2, col: 1 });

        let interlace = Interlace::new(2, 5);
        assert_eq!(interlace.left_corner(), Position { row: 3, col: 0 });
        assert_eq!(interlace.top_corner(), Position { row: 0, col: 3 });
        assert_eq!(interlace.right_corner(), Position { row: 6, col: 9 });
        assert_eq!(interlace.bottom_corner(), Position { row: 9, col: 6 });
    }
}
