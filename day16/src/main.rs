
mod grid;
mod location;

use grid::Grid;
use location::Location;
use std::{collections::HashMap, hash::Hash};
use itertools::Itertools;

#[derive(Clone, Copy, Debug)]
enum Direction {
    North, South, East, West
}

// An instance of a ray-tracing beam.
#[derive(Clone, Copy, Debug)]
struct Beam {
    location: Location,
    direction: Direction,
}

use Direction::*;

impl Beam {
    // Beam is just moving straight forward.
    fn simple_move(&mut self) {
        let next_location =
            match self.direction {
                North => self.location.north(),
                South => self.location.south(),
                East => self.location.east(),
                West => self.location.west(),
            };
        self.location = next_location;
    }
    
    // Runs the beam to the next site based on what it sees at the
    // current site.  Possibly returns a new beam.
    pub fn next_location(&mut self, node: char) -> Option<Beam> {
        match node {
            '.' => {
                self.simple_move();
                None
            },
            '|' => {
                match self.direction {
                    North | South => { self.simple_move(); None },
                    East | West => {
                        // Split to North + South
                        let new_location = self.location.north();
                        self.location = self.location.south();
                        self.direction = South;
                        Some(Beam {
                            location: new_location,
                            direction: North,
                        })
                    },
                }
            },
            '-' => {
                match self.direction {
                    East | West => {self.simple_move(); None },
                    North | South => {
                        // Split to east and west.
                        let new_location = self.location.east();
                        self.location = self.location.west();
                        self.direction = West;
                        Some(Beam {
                            location: new_location,
                            direction: East,
                        })
                    },
                }
            },
            '\\' => match self.direction {
                North => {
                    self.direction = West;
                    self.simple_move();
                    None
                },
                South => {
                    self.direction = East;
                    self.simple_move();
                    None
                },
                East => {
                    self.direction = South;
                    self.simple_move();
                    None
                },
                West => {
                    self.direction = North;
                    self.simple_move();
                    None
                },
            },
            '/' => match self.direction {
                North => {
                    self.direction = East;
                    self.simple_move();
                    None
                },
                South => {
                    self.direction = West;
                    self.simple_move();
                    None
                },
                East => {
                    self.direction = North;
                    self.simple_move();
                    None
                },
                West => {
                    self.direction = South;
                    self.simple_move();
                    None
                },
            },
            _ => None,
        }
    }
}


// TODO - probably should have hidden masks behind a struct.
fn direction_mask(d: Direction) -> i32 {
    match d {
        North => 1,
        South => 2,
        East => 4,
        West => 8,
    }
}

fn next_mask(flags: i32, d: Direction) -> i32 {
    flags | direction_mask(d)
}

fn has_direction(flags: i32, d: Direction) -> bool {
    direction_mask(d) & flags > 0
}

// Walks the beams through the maze.
fn raytrace(grid: &Grid<char>) -> usize {
    let mut visited = HashMap::new();
    let mut beams: Vec<Beam> = vec!(Beam {
        location: Location::new(0,0),
        direction: East,
    });
    let mut next_beams: Vec<Beam> = Vec::new();
    let mut done = beams.is_empty();
    let mut index = 0;
    while !done {
        println!("-- Ray tracing loop {}, beams: {}", index, beams.len());
        index += 1;
        // First mark each beam as visiting its location
        for b in &beams {
            visited.entry(b.location).and_modify(|e| {
                *e = next_mask(*e, b.direction);
            }).or_insert(direction_mask(b.direction));
        }
        // Now we mutate the beams, collecting results for next_beams.
        for b in beams.iter_mut() {
            match grid.get(
                b.location.y().try_into().unwrap(),
                b.location.x().try_into().unwrap()) {
                Some(c) => {
                    match b.next_location(*c) {
                        Some(added) => {
                            next_beams.push(added)
                        },
                        None => (),
                    }
                },
                None => (),
            }
        }
        // All beams have been moved, now we add and filter beams.
        beams.extend(next_beams.into_iter());
        next_beams = vec!();
        beams.retain(|b| {
            let x = b.location.x();
            let y = b.location.y();
            (y >= 0 && y < grid.rows().try_into().unwrap()) &&
            (x >= 0 && x < grid.cols().try_into().unwrap())
        });
        // We're done when all beams have either reached locations THEY
        // visited before, or are off the grid.
        // We do this by tracking the direction we've traveled on each grid.
        done = beams.iter().all(|b| {
            visited.get(&b.location)
            .map(|flags| has_direction(*flags, b.direction))
            .unwrap_or(false)
        });
    }
    // Now we check how many nodes were visited.
    // for (l, v) in visited.iter().sorted_by(|a,b| {
    //     match Ord::cmp(&a.0.y(), &b.0.y()) {
    //         std::cmp::Ordering::Equal => Ord::cmp(&a.0.x(), &b.0.x()),
    //         other => other,
    //     }
    // }) {
    //     println!("{l:?} was visited {v:?}");
    // }

    visited.len()
}

fn main() {
    let input =
      std::fs::read_to_string("input.txt")
      .expect("Must find solution input file: input.txt");
    let grid: Grid<char> = input.as_str().into();
    let result = raytrace(&grid);
    println!("Result: {result}");
}


mod tests {
    use crate::*;

    #[test]
    fn test_raytrace() {
        let grid: Grid<char> =
          ".|...\\....\n\
           |.-.\\.....\n\
           .....|-...\n\
           ........|.\n\
           ..........\n\
           .........\\\n\
           ..../.\\\\..\n\
           .-.-/..|..\n\
           .|....-|.\\\n\
           ..//.|....".into();
        let result = raytrace(&grid);
        assert_eq!(result, 46);
    }

    #[test]
    fn test_flags() {
        assert!(!has_direction(next_mask(0, South), North));
        assert!(!has_direction(next_mask(0, South), East));
        assert!(!has_direction(next_mask(0, South), West));
        assert!(has_direction(next_mask(0, South), South));
        assert!(!has_direction(next_mask(0, North), South));
        assert!(!has_direction(next_mask(0, North), East));
        assert!(!has_direction(next_mask(0, North), West));
        assert!(has_direction(next_mask(0, North), North));
        assert!(!has_direction(next_mask(0, East), North));
        assert!(!has_direction(next_mask(0, East), South));
        assert!(!has_direction(next_mask(0, East), West));
        assert!(has_direction(next_mask(0, East), East));
        assert!(!has_direction(next_mask(0, West), North));
        assert!(!has_direction(next_mask(0, West), East));
        assert!(!has_direction(next_mask(0, West), South));
        assert!(has_direction(next_mask(0, West), West));

        assert!(has_direction(next_mask(next_mask(0, West), East), West));
        assert!(has_direction(next_mask(next_mask(0, West), East), East));
        assert!(!has_direction(next_mask(next_mask(0, West), East), North));
        assert!(!has_direction(next_mask(next_mask(0, West), East), South));
    }

    #[test]
    fn test_next_location() {
        let mut beam = Beam {
            location: Location::new(5,5),
            direction: North,
        };

        let result = beam.next_location('.');
        assert!(result.is_none());
        assert_eq!(beam.location, Location::new(5, 4));
        let result2 = beam.next_location('-');
        assert!(result2.is_some());
        assert_eq!(beam.location, Location::new(4,4));
        assert_eq!(result2.unwrap().location, Location::new(6,4));
    }
}