use crate::cell::Cell;
use crate::grid::Grid;
use crate::interlace::Interlace;
use crate::position::{Position, PositionDelta};
use crate::segment::Segment;

pub struct Colorizer<'a> {
    grid: &'a mut Grid,
    interlace: &'a Interlace,
    number_of_colors: usize,
}

impl<'a> Colorizer<'a> {
    pub fn colorize(grid: &mut Grid, interlace: &Interlace) -> usize {
        let mut colorizer = Colorizer {
            grid,
            interlace,
            number_of_colors: 0,
        };

        colorizer.colorize_from(interlace.left_corner(), Direction::Right);
        colorizer.colorize_from(interlace.top_corner(), Direction::Down);
        colorizer.colorize_from(interlace.right_corner(), Direction::Left);

        for p in colorizer.positions_to_colorize() {
            colorizer.colorize_from(p, Direction::Right);
        }

        colorizer.number_of_colors
    }

    fn colorize_from(&mut self, position: Position, direction: Direction) -> bool {
        let cell = self.grid.at_mut(position);

        if cell.color_number.is_some() {
            false
        } else {
            let color_number = self.number_of_colors;

            cell.color_number = Some(color_number);

            let mut cursor = Cursor {
                position,
                direction,
                grid: self.grid,
            };
            while let Some(cell) = cursor.advance() {
                cell.color_number = Some(color_number);
            }

            self.number_of_colors += 1;
            true
        }
    }

    fn positions_to_colorize(&self) -> Vec<Position> {
        self.interlace
            .left_corner()
            .iter(PositionDelta::to_top_right())
            .skip(1)
            .take(self.interlace.n1 as usize)
            .collect()
    }
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}
impl Direction {
    fn position_delta(&self) -> PositionDelta {
        match self {
            Direction::Left => PositionDelta::to_left(),
            Direction::Right => PositionDelta::to_right(),
            Direction::Up => PositionDelta::to_top(),
            Direction::Down => PositionDelta::to_bottom(),
        }
    }

    fn through(&self, segment: Segment) -> Option<Direction> {
        let direction = match (self, segment) {
            (Direction::Left, Segment::Horizontal) => Direction::Left,
            (Direction::Left, Segment::UpAndRight) => Direction::Up,
            (Direction::Left, Segment::DownAndRight) => Direction::Down,
            (Direction::Right, Segment::Horizontal) => Direction::Right,
            (Direction::Right, Segment::UpAndLeft) => Direction::Up,
            (Direction::Right, Segment::DownAndLeft) => Direction::Down,
            (Direction::Up, Segment::Vertical) => Direction::Up,
            (Direction::Up, Segment::DownAndLeft) => Direction::Left,
            (Direction::Up, Segment::DownAndRight) => Direction::Right,
            (Direction::Down, Segment::Vertical) => Direction::Down,
            (Direction::Down, Segment::UpAndLeft) => Direction::Left,
            (Direction::Down, Segment::UpAndRight) => Direction::Right,
            _ => return None,
        };

        Some(direction)
    }
}

struct Cursor<'a> {
    position: Position,
    direction: Direction,
    grid: &'a mut Grid,
}
impl<'a> Cursor<'a> {
    fn advance(&mut self) -> Option<&mut Cell> {
        const STEPS: u8 = 2;

        for _ in 0..STEPS {
            match self.try_advance() {
                AdvanceResult::NonMatchingSegment => (),
                AdvanceResult::NotFound => return None,
                AdvanceResult::Found(cell) => return Some(unsafe { cell.as_mut().unwrap() }),
            }
        }

        panic!("Path continuation not found in {STEPS} steps");
    }

    fn try_advance(&mut self) -> AdvanceResult {
        self.position += self.direction.position_delta();

        let cell = self.grid.get_mut(self.position);

        let cell = match cell {
            Some(cell) => cell,
            None => return AdvanceResult::NotFound,
        };

        if let Some(next_direction) = self.direction.through(cell.segment) {
            if cell.color_number.is_some() {
                return AdvanceResult::NotFound;
            }

            self.direction = next_direction;
            AdvanceResult::Found(cell)
        } else {
            AdvanceResult::NonMatchingSegment
        }
    }
}

enum AdvanceResult {
    NonMatchingSegment,
    NotFound,
    Found(*mut Cell),
}
