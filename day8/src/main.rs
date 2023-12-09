
use nom::IResult;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::value;
use nom::multi::*;
use nom::character::complete::*;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug, PartialEq)]
struct Node {
    name: String,
    left: String,
    right: String,
}

fn parse_direction(input: &str) -> IResult<&str, Direction> {
    alt((
        value(Direction::Left, tag("L")),
        value(Direction::Right, tag("R"))
    ))(input)
}

fn parse_directions(input: &str) -> IResult<&str, Vec<Direction>> {
    many1(parse_direction)(input)
}

fn parse_id(input: &str) -> IResult<&str, String> {
    let (input, result) = count(anychar, 3)(input)?;
    Ok((input, String::from_iter(result)))
}

// XYZ = (ABC, DEF)
fn parse_node(input: &str) -> IResult<&str, Node> {
    let (input, name) = parse_id(input)?;
    let (input, _) = space1(input)?;
    let (input, _) = tag("=")(input)?;
    let (input, _) = space1(input)?;
    let (input, _) = tag("(")(input)?;
    let (input, left) = parse_id(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, _) = space0(input)?;
    let (input, right) = parse_id(input)?;
    let (input, _) = tag(")")(input)?;
    Ok((input, Node {
        name,
        left,
        right,
    }))
}

fn parse_input(input: &str) -> IResult<&str, (Vec<Direction>, Vec<Node>)> {
    let (input, directions) = parse_directions(input)?;
    let (input, _) = line_ending(input)?;
    // Empty line:
    let (input, _) = line_ending(input)?;
    // Now nodes one per line
    let (input, nodes) = separated_list1(line_ending, parse_node)(input)?;
    Ok((input, (directions, nodes)))
}

fn solve_part_1(directions: Vec<Direction>, nodes: Vec<Node>) -> u32 {
    let mut node_lookup = HashMap::new();
    for node in nodes.iter() {
        node_lookup.insert(node.name.as_str(), node);
    }
    let mut direction_iterator = directions.iter().cloned().cycle();
    let mut current_node = "AAA";
    let mut count: u32 = 0;
    while current_node != "ZZZ" {
      count += 1;
      let Some(direction) = direction_iterator.next() else { panic!("Directions should be infinite iterator")};
      let Some(node) = node_lookup.get(current_node) else { panic!("Badly formed puzzle, can't find {current_node}")};
      match direction {
        Direction::Left => current_node = node.left.as_str(),
        Direction::Right => current_node = node.right.as_str(),
      }
    }
    count
}

fn main() {
    let input = std::fs::read_to_string("input.txt").expect("Input file needs to exist.");
    let (input, (directions, nodes)) = parse_input(&input).unwrap();
    let result = solve_part_1(directions, nodes);
    println!("Part 1 result is {result}!");
}



mod tests {
    use crate::*;

    #[test]
    fn test_parse_directions() {
        assert_eq!(parse_directions("LR"), Ok(("", vec!(Direction::Left, Direction::Right))));
        assert_eq!(parse_directions("RRL"), Ok(("", vec!(Direction::Right, Direction::Right, Direction::Left))));
    }

    #[test]
    fn test_parse_id() {
        let (input, result) = parse_id("ABC").unwrap();
        assert_eq!(input, "");
        assert_eq!(result, "ABC");
    }

    #[test]
    fn test_parse_node() {
        let (input, result) = parse_node("AAA = (BBB,CCC)").unwrap();
        assert_eq!(input, "");
        assert_eq!(result, Node {
            name: String::from("AAA"),
            left: String::from("BBB"),
            right: String::from("CCC"),
        });
    }

    #[test]
    fn test_input() {
        let (input, (directions, nodes)) = parse_input("RL\n\
            \n\
            AAA = (BBB, CCC)\n\
            BBB = (DDD, EEE)\n\
            CCC = (ZZZ, GGG)\n\
            DDD = (DDD, DDD)\n\
            EEE = (EEE, EEE)\n\
            GGG = (GGG, GGG)\n\
            ZZZ = (ZZZ, ZZZ)").unwrap();
        assert_eq!(input, "");
        assert_eq!(directions.len(), 2);
        assert_eq!(nodes.len(), 7);
    }
}