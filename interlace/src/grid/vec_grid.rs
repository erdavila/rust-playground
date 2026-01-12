use std::iter;
use std::ops::{Index, IndexMut};

pub struct VecGrid<T> {
    content: Vec<Vec<T>>,
}

impl<T> VecGrid<T> {
    pub fn new(rows: usize, cols: usize, initial_value: T) -> Self
    where
        T: Clone,
    {
        let initial_row = iter::repeat_n(initial_value, cols).collect::<Vec<_>>();
        let content = iter::repeat_n(initial_row, rows).collect::<Vec<_>>();
        VecGrid { content }
    }

    pub fn get(&self, index: (usize, usize)) -> Option<&T> {
        let (row_index, col_index) = index;
        self.content
            .get(row_index)
            .and_then(|vec| vec.get(col_index))
    }

    pub fn get_mut(&mut self, index: (usize, usize)) -> Option<&mut T> {
        let (row_index, col_index) = index;
        self.content
            .get_mut(row_index)
            .and_then(|vec| vec.get_mut(col_index))
    }
}

impl<T> Index<(usize, usize)> for VecGrid<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (row_index, col_index) = index;
        &self.content[row_index][col_index]
    }
}

impl<T> IndexMut<(usize, usize)> for VecGrid<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let (row_index, col_index) = index;
        &mut self.content[row_index][col_index]
    }
}
