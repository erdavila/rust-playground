use crate::cell::Cell;
use crate::position::Position;

use self::vec_grid::VecGrid;

mod vec_grid;

pub struct Grid {
    n1: u8,
    n2: u8,
    big_table: VecGrid<Option<Cell>>,
    small_table: VecGrid<Option<Cell>>,
}

impl Grid {
    pub fn new(n1: u8, n2: u8) -> Self {
        Grid {
            n1,
            n2,
            big_table: VecGrid::new(n1 as usize + 2, n2 as usize + 2, None),
            small_table: VecGrid::new(n1 as usize + 1, n2 as usize + 1, None),
        }
    }

    pub fn size(&self) -> u8 {
        Self::calculate_size(self.n1, self.n2)
    }

    fn calculate_size(n1: u8, n2: u8) -> u8 {
        3 + n1 + n2
    }

    pub fn at_mut(&mut self, position: Position) -> &mut Cell {
        let internal_position = self.internal_position(position);
        let table = self.table_mut(internal_position.table_id);

        table[internal_position.index]
            .as_mut()
            .expect("Tried to access uninitialized cell")
    }

    pub fn set_at(&mut self, position: Position, cell: Cell) {
        let internal_position = self.internal_position(position);
        let table = self.table_mut(internal_position.table_id);

        let option = &mut table[internal_position.index];
        assert!(option.is_none(), "Cell is already initialized");

        *option = Some(cell);
    }

    pub fn get(&self, position: Position) -> Option<&Cell> {
        let internal_position = self.internal_position(position);
        let table = self.table(internal_position.table_id);

        table
            .get(internal_position.index)
            .map(|cell| cell.as_ref().expect("Cell is not initialized"))
    }

    pub fn get_mut(&mut self, position: Position) -> Option<&mut Cell> {
        let internal_position = self.internal_position(position);
        let table = self.table_mut(internal_position.table_id);

        table
            .get_mut(internal_position.index)
            .map(|cell| cell.as_mut().expect("Cell is not initialized"))
    }

    #[expect(clippy::cast_sign_loss)]
    fn internal_position(&self, position: Position) -> InternalPosition {
        let table_id = TableId::from(self.n1, position.row, position.col);
        let index = match table_id {
            TableId::Big => {
                let row = (self.n1.cast_signed() + position.row - position.col + 1) / 2;
                let col = position.row - row;
                (row as usize, col as usize)
            }
            TableId::Small => {
                let row = (self.n1.cast_signed() + position.row - position.col) / 2;
                let col = position.row - row - 1;
                (row as usize, col as usize)
            }
        };

        InternalPosition { table_id, index }
    }

    fn table(&self, table_id: TableId) -> &VecGrid<Option<Cell>> {
        match table_id {
            TableId::Big => &self.big_table,
            TableId::Small => &self.small_table,
        }
    }

    fn table_mut(&mut self, table_id: TableId) -> &mut VecGrid<Option<Cell>> {
        match table_id {
            TableId::Big => &mut self.big_table,
            TableId::Small => &mut self.small_table,
        }
    }

    pub fn rows_iter(&self) -> RowsIter<'_> {
        RowsIter {
            grid: self,
            next_row_index: 0,
        }
    }
}

struct InternalPosition {
    table_id: TableId,
    index: (usize, usize),
}

#[derive(Clone, Copy)]
enum TableId {
    Big,
    Small,
}
impl TableId {
    fn from(n1: u8, row: i8, col: i8) -> Self {
        let n1_is_even = n1.is_multiple_of(2);
        let row_col_sum_is_even = (row + col) % 2 == 0;

        if n1_is_even == row_col_sum_is_even {
            TableId::Small
        } else {
            TableId::Big
        }
    }
}

pub struct RowsIter<'a> {
    grid: &'a Grid,
    next_row_index: u8,
}

impl<'a> Iterator for RowsIter<'a> {
    type Item = Row<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next_row_index < self.grid.size() {
            let row_index = self.next_row_index;
            self.next_row_index += 1;
            Some(Row {
                grid: self.grid,
                row_index,
            })
        } else {
            None
        }
    }
}

pub struct Row<'a> {
    grid: &'a Grid,
    row_index: u8,
}

impl<'a> IntoIterator for Row<'a> {
    type Item = Option<&'a Cell>;

    type IntoIter = RowIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        RowIter {
            grid: self.grid,
            row_index: self.row_index,
            next_col_index: 0,
        }
    }
}

pub struct RowIter<'a> {
    grid: &'a Grid,
    row_index: u8,
    next_col_index: u8,
}

impl<'a> Iterator for RowIter<'a> {
    type Item = Option<&'a Cell>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next_col_index < self.grid.size() {
            let col_index = self.next_col_index;
            self.next_col_index += 1;

            Some(self.grid.get(Position {
                row: self.row_index.cast_signed(),
                col: col_index.cast_signed(),
            }))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::interlace::Interlace;
    use crate::segment::Segment;

    use super::*;

    #[test]
    fn calculate_size() {
        assert_eq!(Grid::calculate_size(0, 0), 3);
        assert_eq!(Grid::calculate_size(2, 5), 10);
    }

    struct Fixture {
        grid: Grid,
        inside_position: Position,
        outside_position: Position,
    }
    impl Fixture {
        fn new() -> Self {
            let interlace = Interlace::new(2, 5);

            Fixture {
                grid: Grid::new(interlace.n1, interlace.n2),
                inside_position: interlace.left_corner(),
                outside_position: Position { row: 0, col: 0 },
            }
        }

        fn initialize_cell_at(&mut self, position: Position, segment: Segment) {
            let internal_position = self.grid.internal_position(position);
            let table = self.grid.table_mut(internal_position.table_id);
            table[internal_position.index] = Some(Cell::no_color(segment));
        }

        fn cell_at(&self, position: Position) -> &Cell {
            let internal_position = self.grid.internal_position(position);
            let table = self.grid.table(internal_position.table_id);
            table[internal_position.index].as_ref().unwrap()
        }
    }

    #[test]
    fn at_mut_on_initialized_cell() {
        let mut fixture = Fixture::new();
        fixture.initialize_cell_at(fixture.inside_position, Segment::Horizontal);

        let result: &mut Cell = fixture.grid.at_mut(fixture.inside_position);

        assert_eq!(result.segment, Segment::Horizontal);
    }

    #[test]
    #[should_panic = "Tried to access uninitialized cell"]
    fn at_mut_on_uninitialized_cell() {
        let mut fixture = Fixture::new();

        fixture.grid.at_mut(fixture.inside_position);
    }

    #[test]
    #[should_panic = "index out of bounds"]
    fn at_mut_on_cell_outside_the_interlace() {
        let mut fixture = Fixture::new();

        fixture.grid.at_mut(fixture.outside_position);
    }

    #[test]
    #[should_panic = "Cell is already initialized"]
    fn set_at_on_initialized_cell() {
        let mut fixture = Fixture::new();
        fixture.initialize_cell_at(fixture.inside_position, Segment::Horizontal);

        fixture
            .grid
            .set_at(fixture.inside_position, Cell::no_color(Segment::Vertical))
    }

    #[test]
    fn set_at_on_uninitialized_cell() {
        let mut fixture = Fixture::new();

        fixture
            .grid
            .set_at(fixture.inside_position, Cell::no_color(Segment::Vertical));

        assert_eq!(
            fixture.cell_at(fixture.inside_position).segment,
            Segment::Vertical
        );
    }

    #[test]
    #[should_panic = "index out of bounds"]
    fn set_at_on_cell_outside_the_interlace() {
        let mut fixture = Fixture::new();

        fixture
            .grid
            .set_at(fixture.outside_position, Cell::no_color(Segment::Vertical));
    }

    #[test]
    fn get_on_initialized_cell() {
        let mut fixture = Fixture::new();
        fixture.initialize_cell_at(fixture.inside_position, Segment::Horizontal);

        let result = fixture.grid.get(fixture.inside_position);

        let cell = result.expect("result should be Some(_)");
        assert_eq!(cell.segment, Segment::Horizontal);
    }

    #[test]
    #[should_panic = "Cell is not initialized"]
    fn get_on_uninitialized_cell() {
        let fixture = Fixture::new();

        fixture.grid.get(fixture.inside_position);
    }

    #[test]
    fn get_on_cell_outside_the_interlace() {
        let fixture = Fixture::new();

        let result = fixture.grid.get(fixture.outside_position);

        assert!(result.is_none());
    }

    #[test]
    fn get_mut_on_initialized_cell() {
        let mut fixture = Fixture::new();
        fixture.initialize_cell_at(fixture.inside_position, Segment::Horizontal);

        let result: Option<&mut Cell> = fixture.grid.get_mut(fixture.inside_position);

        let cell = result.expect("result should be Some(_)");
        assert_eq!(cell.segment, Segment::Horizontal);
    }

    #[test]
    #[should_panic = "Cell is not initialized"]
    fn get_mut_on_uninitialized_cell() {
        let mut fixture = Fixture::new();

        fixture.grid.get_mut(fixture.inside_position);
    }

    #[test]
    fn get_mut_on_cell_outside_the_interlace() {
        let mut fixture = Fixture::new();

        let result: Option<&mut Cell> = fixture.grid.get_mut(fixture.outside_position);

        assert!(result.is_none());
    }
}
