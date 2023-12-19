use std::convert::From;
use std::fmt;

// An implementation of a 2D matrix/grid.
// Provides helper iterators, and simple ingestion from strings.
pub struct Grid<T> {
    cells: Vec<T>,
    rows: usize,
    cols: usize,
}

impl <T> fmt::Display for Grid<T> where T: fmt::Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      for row in 0..self.rows() {
        write!(f, "|")?;
        for col in 0..self.cols() {
          if col > 0 {
            write!(f, " ")?;
          }
          write!(f, "{}", self.get(row, col).unwrap())?
        }
        write!(f, "|\n")?;
      }
      // TODO - write
      write!(f, "\n")
    }
}

impl <T> From<&str> for Grid<T> where T: From<char> {
    fn from(input: &str) -> Self {
        let mut rows = 0;
        let cells: Vec<T> =
          input.lines()
          .inspect(|_| rows += 1)
          .flat_map(|line| line.chars())
          .map(|c| c.into())
          .collect();
        let cols = cells.len() / rows;
        Grid { cells, rows, cols }
    }
}

impl <T> Grid<T> {
  pub fn rows(&self) -> usize {
    self.rows
  }
  pub fn cols(&self) -> usize {
    self.rows
  }
  pub fn get(&self, row: usize, col: usize) -> Option<&T> {
        if row < self.rows && col < self.cols {
            self.cells.get(row*self.cols + col)
        } else {
            None
        }
    }
    pub fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut T> {
      if row < self.rows && col < self.cols {
        self.cells.get_mut(row*self.cols + col)
      } else {
          None
      }
    }
    // Iterators lifetimes are tied to ours.
    pub fn row_iter<'a>(&'a self, row: usize) -> GridRowIterator<'a, T> {
        GridRowIterator { grid: self, row, index: 0 }
    }
    pub fn col_iter<'a>(&'a self, col: usize) -> GridColIterator<'a, T> {
        GridColIterator { grid: self, col: col, index: 0 }
    }
    pub fn iter<'a>(&'a self) -> GridIterator<'a, T> {
      GridIterator { grid: self, row: 0, col: 0 }
    }
}

// We tie the iterators lifetime to the grid.
pub struct GridRowIterator<'a, T> {
    grid: &'a Grid<T>,
    row: usize,
    index: usize,
}
impl <'a, T> Iterator for GridRowIterator<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        let result = self.grid.get(self.row, self.index);
        self.index += 1;
        result
    }
}

pub struct GridColIterator<'a, T> {
    grid: &'a Grid<T>,
    col: usize,
    index: usize,
}
impl <'a, T> Iterator for GridColIterator<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        let result = self.grid.get(self.index, self.col);
        self.index += 1;
        result
    }
}

// Iterates row->column across every item, returning the index.
pub struct GridIterator<'a, T> {
  grid: &'a Grid<T>,
  row: usize,
  col: usize,
}

impl <'a, T> Iterator for GridIterator<'a, T> {
  type Item = ((usize, usize),&'a T);
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
      let grid: Grid<char> =
        ".a.\n\
         .b.\n\
         .c.".into();

      let row1: Vec<char> =
        grid.row_iter(1)
        .map(|cell| *cell)
        .collect();
      assert_eq!(row1, vec!('.', 'b', '.'));
  }
  #[test]
  fn test_col_iteration() {
      let grid: Grid<char> =
        ".a.\n\
         .b.\n\
         .c.".into();

      let row1: Vec<char> =
        grid.col_iter(1)
        .map(|cell| *cell)
        .collect();
      assert_eq!(row1, vec!('a', 'b', 'c'));
  }
  #[test]
  fn test_grid_iteration() {
    let grid: Grid<char> = "ab\ncd".into();
    assert_eq!(grid.cols(), 2);
    assert_eq!(grid.rows(), 2);
    let xy_points: Vec<((usize, usize), char)> = 
      grid.iter().map(|(xy, c)| (xy, *c)).collect();
    assert_eq!(xy_points, vec!(((0,0), 'a'), ((0,1), 'b'), ((1,0), 'c'), ((1,1), 'd')));
  }

  #[test]
  fn test_display() {
    let grid: Grid<char> = "ab\ncd".into();
    assert_eq!(format!("{}", grid), "|a b|\n|c d|\n\n");
  }
}