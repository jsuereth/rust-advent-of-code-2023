use std::convert::From;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell {
    value: char,
}

impl Cell {
  pub fn value(&self) -> char { self.value }
}


impl From<char> for Cell {
    fn from(input: char) -> Self {
        Cell { value: input }
    }
}

pub struct Grid {
    cells: Vec<Cell>,
    rows: usize,
    cols: usize,
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      // TODO - write
        write!(f, "{}", "")
    }
}

impl From<&str> for Grid  {
    fn from(input: &str) -> Self {
        let mut rows = 0;
        let cells: Vec<Cell> =
          input.lines()
          .inspect(|_| rows += 1)
          .flat_map(|line| line.chars())
          .map(|c| c.into())
          .collect();
        let cols = cells.len() / rows;
        Grid { cells, rows, cols }
    }
}

impl Grid {
  pub fn rows(&self) -> usize {
    self.rows
  }
  pub fn cols(&self) -> usize {
    self.rows
  }
  pub fn get(&self, row: usize, col: usize) -> Option<&Cell> {
        if row < self.rows && col < self.cols {
            self.cells.get(row*self.cols + col)
        } else {
            None
        }
    }
    pub fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut Cell> {
      if row < self.rows && col < self.cols {
        self.cells.get_mut(row*self.cols + col)
      } else {
          None
      }
    }
    // Iterators lifetimes are tied to ours.
    pub fn row_iter<'a>(&'a self, row: usize) -> GridRowIterator<'a> {
        GridRowIterator { grid: self, row, index: 0 }
    }
    pub fn col_iter<'a>(&'a self, col: usize) -> GridColIterator<'a> {
        GridColIterator { grid: self, col: col, index: 0 }
    }
    pub fn iter<'a>(&'a self) -> GridIterator<'a> {
      GridIterator { grid: self, row: 0, col: 0 }
    }
}

// We tie the iterators lifetime to the grid.
pub struct GridRowIterator<'a> {
    grid: &'a Grid,
    row: usize,
    index: usize,
}
impl <'a> Iterator for GridRowIterator<'a> {
    type Item = &'a Cell;
    fn next(&mut self) -> Option<Self::Item> {
        let result = self.grid.get(self.row, self.index);
        self.index += 1;
        result
    }
}

pub struct GridColIterator<'a> {
    grid: &'a Grid,
    col: usize,
    index: usize,
}
impl <'a> Iterator for GridColIterator<'a> {
    type Item = &'a Cell;
    fn next(&mut self) -> Option<Self::Item> {
        let result = self.grid.get(self.index, self.col);
        self.index += 1;
        result
    }
}

// Iterates row->column across every item, returning the index.
pub struct GridIterator<'a> {
  grid: &'a Grid,
  row: usize,
  col: usize,
}

impl <'a> Iterator for GridIterator<'a> {
  type Item = ((usize, usize),&'a Cell);
  fn next(&mut self) -> Option<Self::Item> {
    if self.col >= self.grid.cols {
      self.row += 1;
      self.col = 0;
    }
    if self.row < self.grid.rows {
      let row = self.row;
      let col = self.col;
      let result = 
        self.grid.get(row, col)
        .map(|c| ((row,col), c));
      self.col +=1;
      result
    } else {
      None
    }
  }
}

mod tests {
  use crate::grid::*;

  #[test]
  fn test_row_iteration() {
      let grid: Grid =
        ".a.\n\
         .b.\n\
         .c.".into();

      let row1: Vec<char> =
        grid.row_iter(1)
        .map(|cell| cell.value)
        .collect();
      assert_eq!(row1, vec!('.', 'b', '.'));
  }
  #[test]
  fn test_col_iteration() {
      let grid: Grid =
        ".a.\n\
         .b.\n\
         .c.".into();

      let row1: Vec<char> =
        grid.col_iter(1)
        .map(|cell| cell.value)
        .collect();
      assert_eq!(row1, vec!('a', 'b', 'c'));
  }
  #[test]
  fn test_grid_iteration() {
    let grid: Grid = "ab\ncd".into();
    assert_eq!(grid.cols(), 2);
    assert_eq!(grid.rows(), 2);
    let xy_points: Vec<((usize, usize), char)> = 
      grid.iter().map(|(xy, c)| (xy, c.value())).collect();
    assert_eq!(xy_points, vec!(((0,0), 'a'), ((0,1), 'b'), ((1,0), 'c'), ((1,1), 'd')));
  }
}