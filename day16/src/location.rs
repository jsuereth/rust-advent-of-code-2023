use std::fmt;
use std::convert;


// A location in the grid.
//
// x - 0 is left, N is right
// y - 0 is top, N is bottom
#[derive(Clone, Copy, Debug, PartialEq, Hash, Eq)]
pub struct Location {
    x: i64,
    y: i64,
}

impl Location {
  pub fn x(&self) -> i64 {
    self.x
  }
  pub fn y(&self) -> i64 {
    self.y
  }
    pub fn north(&self) -> Location {
        Location {
            x: self.x,
            y: self.y-1,
        }
    }
    pub fn south(&self) -> Location {
        Location {
            x: self.x,
            y: self.y+1,
        }
    }
    pub fn east(&self) -> Location {
        Location {
            x: self.x+1,
            y: self.y,
        }
    }
    pub fn west(&self) -> Location {
        Location {
            x: self.x-1,
            y: self.y,
        }
    }
    pub fn new(x: i64, y: i64) -> Location {
        Location { x, y }
    }
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

impl From<(usize, usize)> for Location {
    fn from(value: (usize, usize)) -> Self {
        Location {
          // Note: this is unsafe only for very large sizes,
          // which we do NOT have to worry about in this AoC problem.
          x: value.0.try_into().unwrap(), 
          y: value.1.try_into().unwrap(),
        }
    }
}

mod tests {
  use crate::{grid::*, location::Location};

  #[test]
  fn test_tuple_conversion() {
    let input: (usize, usize) = (1,2);
    let result: Location = input.into();
    assert_eq!(result, Location::new(1,2));
  }

  #[test]
  fn test_location() {
      let x = Location { x: 10, y: 10 };
      assert_eq!(x.north(), Location::new(10,9));
      assert_eq!(x.south(), Location::new(10,11));
      assert_eq!(x.east(), Location::new(11, 10));
      assert_eq!(x.west(), Location::new(9, 10));
  }
}