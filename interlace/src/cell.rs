use crate::segment::Segment;

#[derive(Clone, Copy, Debug)]
pub struct Cell {
    pub segment: Segment,
    pub color_number: Option<usize>,
}

impl Cell {
    pub fn no_color(segment: Segment) -> Cell {
        Cell {
            segment,
            color_number: None,
        }
    }
}
