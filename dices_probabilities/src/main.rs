use std::ops::{Index, IndexMut};

use term_table::{row::Row, table_cell::TableCell, Table, TableStyle};

const DEFAULT_FACES: u8 = 6;

fn main() {
    let mut args = std::env::args();
    args.next();

    let dices: u8 = args
        .next()
        .expect("How many dices?")
        .parse()
        .expect("Invalid number");
    if dices == 0 {
        panic!("Number of dices cannot be zero");
    }

    let faces = args
        .next()
        .map(|str| str.parse::<u8>().expect("Invalid number"))
        .unwrap_or(DEFAULT_FACES);

    let probabilities = Probabilities::generate(dices, faces);
    let possibilities = (faces as u32).pow(dices as u32);

    let mut table = Table::new();
    table.style = TableStyle::extended();
    table.add_row(Row::new(vec![
        TableCell::new("N"),
        TableCell::new("P(X=N)"),
        TableCell::new("P(X≤N)"),
        TableCell::new("P(X≥N)"),
    ]));

    let mut first = true;
    let mut accumulated_le = 0;
    let mut accumulated_ge = possibilities;
    for i in probabilities.min_value()..=probabilities.max_value() {
        let n = probabilities.values[i as usize];
        accumulated_le += n;

        let format_prob = |x| {
            format!(
                "{}:{possibilities} = {:.2}%",
                x,
                100.0 * x as f64 / possibilities as f64
            )
        };

        let mut row = Row::new(vec![
            TableCell::new(i),
            TableCell::new(format_prob(n)),
            TableCell::new(format_prob(accumulated_le)),
            TableCell::new(format_prob(accumulated_ge)),
        ]);
        row.has_separator = first;
        table.add_row(row);

        first = false;

        accumulated_ge -= n;
    }

    println!("{}", table.render());
}

struct OffsetVect<T> {
    vec: Vec<T>,
    offset: usize,
}
impl<T> OffsetVect<T> {
    pub fn new(vec: Vec<T>, offset: usize) -> Self {
        OffsetVect { vec, offset }
    }
}
impl<T> Index<usize> for OffsetVect<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.vec[index - self.offset]
    }
}
impl<T> IndexMut<usize> for OffsetVect<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.vec[index - self.offset]
    }
}

struct Probabilities {
    pub values: OffsetVect<u32>,
}
impl Probabilities {
    pub fn generate(dices: u8, faces: u8) -> Probabilities {
        let min_value = dices as u32;
        let max_value = dices as u32 * faces as u32;
        let values_count = max_value - min_value + 1;

        let values = vec![0; values_count as usize];
        let mut probabilities = Probabilities {
            values: OffsetVect::new(values, min_value as usize),
        };

        probabilities.generate_impl(dices, faces, 0);

        probabilities
    }

    fn generate_impl(&mut self, dices: u8, faces: u8, total_value: u32) {
        if dices == 0 {
            self.values[total_value as usize] += 1;
        } else {
            for face in 1..=faces {
                self.generate_impl(dices - 1, faces, total_value + face as u32);
            }
        }
    }

    pub fn min_value(&self) -> u32 {
        self.values.offset as u32
    }

    pub fn max_value(&self) -> u32 {
        (self.values.offset + self.values.vec.len() - 1) as u32
    }
}
