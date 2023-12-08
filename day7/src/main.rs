mod hand;

use hand::Hand;
use nom::{Parser,IResult};
use nom::bytes::complete::tag;
use nom::branch::alt;
use nom::combinator::value;
use nom::multi::count;
use nom::character::complete::{
    space1, 
    u32,
};
use std::fs;
use itertools::Itertools;

#[derive(Clone)]
struct Person {
    hand: Hand,
    bid: u32,
}

fn part_one_solution(people: Vec<Person>) -> u32 {
    let mut sorted: Vec<Person> = people.clone();
    sorted.sort_by(|lhs,rhs| lhs.hand.cmp(&rhs.hand));
    sorted.iter()
    .enumerate()
    .map(|(count, p)| {
        let rank: u32 = (count+1).try_into().unwrap();
        return p.bid*rank
    }).sum()
}

fn parse_card(input: &str) -> IResult<&str,u8> {
    alt((
        value(1, tag("1")),
        value(2, tag("2")),
        value(3, tag("3")),
        value(4, tag("4")),
        value(5, tag("5")),
        value(6, tag("6")),
        value(7, tag("7")),
        value(8, tag("8")),
        value(9, tag("9")),
        value(10, tag("T")),
        value(11, tag("J")),
        value(12, tag("Q")),
        value(13, tag("K")),
        value(14, tag("A")),
    ))(input)
}

fn parse_hand(input: &str) -> IResult<&str, Hand> {
    let (input, vec) = count(parse_card, 5)(input)?;
    let ar5: [u8;5] = vec.as_slice().try_into().unwrap();
    Ok((input, Hand::from(ar5)))
}

fn parse_bid(input: &str) -> IResult<&str, Person> {
    let (input, hand) = parse_hand(input)?;
    let (input, _) = space1(input)?;
    let (input, bid) = u32(input)?;
    Ok((input, Person {
        hand,
        bid,
    }))
}

fn main() {
    let results = 
      fs::read_to_string("input.txt")
      .expect("Input file needs to exist.");
    let people: Vec<Person> =
      results.lines()
      .map(|line| {
        let (input, result) = parse_bid(line).unwrap();
        result
      })
      .collect();
    let result = part_one_solution(people);
    println!("Result: {result}");
}

#[cfg(test)]
mod tests {
    use crate::*;
    use crate::hand::*;

    #[test]
    fn test_parse_hand() {
        let (input, hand) = parse_hand("KKQ13").unwrap();
        assert_eq!(input, "");
        assert_eq!(hand, Hand::from([13,13,12,1,3]));
    }
}