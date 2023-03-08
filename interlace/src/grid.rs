use std::iter;
use std::ops::{Index, IndexMut};

use crate::position::Position;

pub struct Grid<T> {
    pub size: u8,
    pub content: Vec<Vec<T>>,
}

impl<T> Grid<T> {
    pub fn new(value: T, size: u8) -> Self
    where
        T: Clone,
    {
        let row = vec_repeat(value, size);
        let content = vec_repeat(row, size);
        Self { size, content }
    }

    pub fn get_mut(&mut self, position: Position) -> Option<&mut T> {
        self.content
            .get_mut(position.row as usize)
            .and_then(|row| row.get_mut(position.col as usize))
    }
}

impl<T> Index<Position> for Grid<T> {
    type Output = T;

    fn index(&self, index: Position) -> &Self::Output {
        &self.content[index.row as usize][index.col as usize]
    }
}

impl<T> IndexMut<Position> for Grid<T> {
    fn index_mut(&mut self, index: Position) -> &mut Self::Output {
        &mut self.content[index.row as usize][index.col as usize]
    }
}

fn vec_repeat<T: Clone>(elem: T, n: u8) -> Vec<T> {
    Vec::from_iter(iter::repeat(elem).take(n as usize))
}
