use std::env;

use grid::Grid;
use interlace::Interlace;
use position::PositionDelta;
use segment::Segment;

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

    let mut grid = Grid::new(None, interlace.required_grid_size());

    make_paths(&interlace, &mut grid);

    print_grid(&grid);
}

fn make_paths(interlace: &Interlace, grid: &mut Grid<Option<Segment>>) {
    grid[interlace.left_corner()] = Some(Segment::Horizontal);
    grid[interlace.top_corner()] = Some(Segment::Vertical);
    grid[interlace.right_corner()] = Some(Segment::Horizontal);
    grid[interlace.bottom_corner()] = Some(Segment::Vertical);

    for p in interlace
        .left_corner()
        .iter(PositionDelta::to_top_right())
        .skip(1)
        .take(interlace.n1 as usize)
    {
        grid[p] = Some(Segment::DownAndRight);
    }

    for p in interlace
        .bottom_corner()
        .iter(PositionDelta::to_top_right())
        .skip(1)
        .take(interlace.n1 as usize)
    {
        grid[p] = Some(Segment::UpAndLeft);
    }

    for p in (interlace.left_corner() + PositionDelta::to_right())
        .iter(PositionDelta::to_top_right())
        .take((interlace.n1 + 1) as usize)
    {
        for p2 in p
            .iter(PositionDelta::to_bottom_right())
            .take((interlace.n2 + 1) as usize)
        {
            grid[p2] = Some(Segment::Horizontal);
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
            grid[p2] = Some(Segment::Vertical);
        }
    }

    for p in interlace
        .left_corner()
        .iter(PositionDelta::to_bottom_right())
        .skip(1)
        .take(interlace.n2 as usize)
    {
        grid[p] = Some(Segment::UpAndRight);
    }

    for p in interlace
        .top_corner()
        .iter(PositionDelta::to_bottom_right())
        .skip(1)
        .take(interlace.n2 as usize)
    {
        grid[p] = Some(Segment::DownAndLeft);
    }
}

fn print_grid(grid: &Grid<Option<Segment>>) {
    let print_border_row = |first_segment, last_segment| {
        println!(
            "{}{}{}",
            first_segment,
            Segment::Horizontal
                .to_string()
                .repeat((grid.size * 2 + 1) as usize),
            last_segment
        );
    };

    print_border_row(Segment::DownAndRight, Segment::DownAndLeft);

    for row in &grid.content {
        print!("{} ", Segment::Vertical);
        for segment in row {
            let char = segment.map_or(' ', |segm| segm.char());
            print!("{} ", char);
        }
        println!("{}", Segment::Vertical);
    }

    print_border_row(Segment::UpAndRight, Segment::UpAndLeft);
}
