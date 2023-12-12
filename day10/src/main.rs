use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;
use itertools::Itertools;

// A location in the grid.
//
// x - 0 is left, N is right
// y - 0 is top, N is bottom
#[derive(Clone, Copy, Debug, PartialEq, Hash, Eq)]
struct Location {
    x: i64,
    y: i64,
}

impl Location {
    fn north(&self) -> Location {
        Location {
            x: self.x,
            y: self.y-1,
        }
    }
    fn south(&self) -> Location {
        Location {
            x: self.x,
            y: self.y+1,
        }
    }
    fn east(&self) -> Location {
        Location {
            x: self.x+1,
            y: self.y,
        }
    }
    fn west(&self) -> Location {
        Location {
            x: self.x-1,
            y: self.y,
        }
    }
    fn new(x: i64, y: i64) -> Location {
        Location { x, y }
    }
}
impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

#[derive(Debug)]
struct Node {
    id: Location,
    name: char,
}


impl Node {
    fn new(id: Location, name: char) -> Node {
        Node { id, name }
    }
    fn connections(&self) -> Vec<Location> {
        match self.name {
            '|' => vec!(self.id.north(), self.id.south()),
            '-' => vec!(self.id.east(), self.id.west()),
            'L' => vec!(self.id.north(), self.id.east()),
            'J' => vec!(self.id.north(), self.id.west()),
            '7' => vec!(self.id.south(), self.id.west()),
            'F' => vec!(self.id.south(), self.id.east()),
            _ => vec!(),
        }
    }
}


fn solve(nodes: Vec<Node>) -> i64 {
    // Put all edges into a data structure (Location -> Set<Location>)
    let mut edges = HashMap::new();
    for n in &nodes {
        for c in n.connections() {
            // Write edges from n->n2 and n2->n
            edges.entry(n.id).or_insert(HashSet::new()).insert(c);
            edges.entry(c).or_insert(HashSet::new()).insert(n.id);
        }
    }
    let Some(start) = nodes.iter().find(|n| n.name =='S') else { panic!("Could not find starting position S!") };
    // For every location we write an integer, if not visited, denoting distance from start.
    let mut colors = HashMap::new();
    let mut cur_nodes = vec!(start.id);
    let mut cur_iteration = 0;
    let mut done = false;
    while !done {
        let len = cur_nodes.len();
        println!("{cur_iteration} STEP, nodes: {len}");

        for n in cur_nodes.iter().cloned() {
            println!(" - Visiting node: {n}");
            colors.insert(n, cur_iteration);
        }
        // Figure out next nodes to use:
        let mut next_nodes: Vec<Location> =
          cur_nodes.iter()
          .flat_map(|n| edges.get(n).unwrap().iter())
        //   .inspect(|edge| {
        //     let color = colors.get(edge);
        //     println!("Edge: {edge} has color: {color:?}");
        //   })
          // Only visit nodes we haven't seen yet.
          .filter(|n| !colors.contains_key(n))
          .copied()
          .unique()
          .collect();

        // Update the coloring.
        done = next_nodes.len() == 0;
        cur_iteration += 1;
        cur_nodes.clear();
        cur_nodes.append(&mut next_nodes);
    }

    cur_iteration
}


fn parse_input(input: &str) -> Vec<Node> {
    let mut result = Vec::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            result.push(Node::new(Location::new(x.try_into().unwrap(),y.try_into().unwrap()), c));
        }
    }
    result
}

fn main() {
    let input =
      std::fs::read_to_string("input.txt")
      .expect("Must find solution input!");
    let result =solve(parse_input(&input));
    println!("Result: {result}!");
}


mod tests {
    use crate::*;

    #[test]
    fn test_location() {
        let x = Location { x: 10, y: 10 };
        assert_eq!(x.north(), Location::new(10,9));
        assert_eq!(x.south(), Location::new(10,11));
        assert_eq!(x.east(), Location::new(11, 10));
        assert_eq!(x.west(), Location::new(9, 10));
    }
    #[test]
    fn test_node() {
        let initial = Location { x: 10, y: 10 };
        assert_eq!(Node::new(initial, '.').connections(), vec!());
        // Not sure the value of these tests.
        assert_eq!(Node::new(initial, '|').connections(), vec!(initial.north(), initial.south()));
        assert_eq!(Node::new(initial, 'F').connections(), vec!(initial.south(), initial.east()));
    }
}