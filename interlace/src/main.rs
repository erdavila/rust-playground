use std::env;

use cell::Cell;
use colorizer::Colorizer;
use grid::Grid;
use interlace::Interlace;
use position::PositionDelta;
use segment::Segment;

mod cell;
mod colorizer;
mod grid;
mod interlace;
mod position;
mod segment;

fn main() {
    let args: Vec<u8> = env::args()
        .skip(1)
        .take(2)
        .map(|arg| arg.parse().unwrap())
        .collect();
    let n1 = args[0];
    let n2 = args[1];

    let interlace = Interlace::new(n1, n2);

    let mut grid = Grid::new(n1, n2);

    make_paths(&interlace, &mut grid);

    let number_of_colors = Colorizer::colorize(&mut grid, &interlace);
    println!("Number of colors: {number_of_colors}");

    print_grid(&grid);
}

fn make_paths(interlace: &Interlace, grid: &mut Grid) {
    grid.set_at(interlace.left_corner(), Cell::no_color(Segment::Horizontal));
    grid.set_at(interlace.top_corner(), Cell::no_color(Segment::Vertical));
    grid.set_at(
        interlace.right_corner(),
        Cell::no_color(Segment::Horizontal),
    );
    grid.set_at(interlace.bottom_corner(), Cell::no_color(Segment::Vertical));

    for p in interlace
        .left_corner()
        .iter(PositionDelta::to_top_right())
        .skip(1)
        .take(interlace.n1 as usize)
    {
        grid.set_at(p, Cell::no_color(Segment::DownAndRight));
    }

    for p in interlace
        .bottom_corner()
        .iter(PositionDelta::to_top_right())
        .skip(1)
        .take(interlace.n1 as usize)
    {
        grid.set_at(p, Cell::no_color(Segment::UpAndLeft));
    }

    for p in (interlace.left_corner() + PositionDelta::to_right())
        .iter(PositionDelta::to_top_right())
        .take((interlace.n1 + 1) as usize)
    {
        for p2 in p
            .iter(PositionDelta::to_bottom_right())
            .take((interlace.n2 + 1) as usize)
        {
            grid.set_at(p2, Cell::no_color(Segment::Horizontal));
        }
    }

    for p in (interlace.left_corner() + PositionDelta::to_right() * 2)
        .iter(PositionDelta::to_top_right())
        .take(interlace.n1 as usize)
    {
        for p2 in p
            .iter(PositionDelta::to_bottom_right())
            .take(interlace.n2 as usize)
        {
            grid.set_at(p2, Cell::no_color(Segment::Vertical));
        }
    }

    for p in interlace
        .left_corner()
        .iter(PositionDelta::to_bottom_right())
        .skip(1)
        .take(interlace.n2 as usize)
    {
        grid.set_at(p, Cell::no_color(Segment::UpAndRight));
    }

    for p in interlace
        .top_corner()
        .iter(PositionDelta::to_bottom_right())
        .skip(1)
        .take(interlace.n2 as usize)
    {
        grid.set_at(p, Cell::no_color(Segment::DownAndLeft));
    }
}

fn print_grid(grid: &Grid) {
    let print_border_row = |first_segment, last_segment| {
        println!(
            "{}{}{}",
            first_segment,
            Segment::Horizontal
                .to_string()
                .repeat((grid.size() * 2 + 1) as usize),
            last_segment
        );
    };

    print_border_row(Segment::DownAndRight, Segment::DownAndLeft);

    for row in grid.rows_iter() {
        print!("{} ", Segment::Vertical);
        for cell in row {
            let (char, color_number) = match cell {
                Some(cell) => (cell.segment.char(), cell.color_number),
                None => (' ', None),
            };

            let (color_code_begin, color_code_end) = match color_number {
                Some(color_number) => {
                    assert!(color_number < 7, "Only 7 colors are supported");
                    fn color_code(number: usize) -> String {
                        format!("\x1b[{}m", number)
                    }

                    let begin = color_code(31 + color_number);
                    let end = color_code(0);
                    (begin, end)
                }
                None => (String::new(), String::new()),
            };

            print!("{}{}{} ", color_code_begin, char, color_code_end);
        }
        println!("{}", Segment::Vertical);
    }

    print_border_row(Segment::UpAndRight, Segment::UpAndLeft);
}
