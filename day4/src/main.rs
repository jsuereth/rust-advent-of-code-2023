
use nom::{Parser,IResult};
use nom::sequence::tuple;
use nom::bytes::complete::tag;
use nom::multi::separated_list1;
use nom::error::context;
use nom::character::complete::{space0, space1, u32};
use std::collections::HashSet;
use std::fs;

struct CardResult {}

fn line_parser(input: &str) -> IResult<&str, u32> {
    // Instead of combining parsers into larger ones,
    // we can just immediately use them, and leverage `?` for
    // monadic composition.
    let (input, (_, _, card_num, _)) =
      context("card number", tuple((tag("Card"), space1, u32, tag(":"))))(input)?;
    let (input, _) = space0(input)?;
    let (input, winners) = context("winning numbers", separated_list1(space1, u32))(input)?;
    let (input, _) = space0(input)?;
    let (input, _) = tag("|")(input)?;
    let (input, _) = space0(input)?;
    let (input, hand) = separated_list1(space1, u32)(input)?;
    // Here we can just calculate the score directly then.
    let winner_set: HashSet<u32> =  HashSet::from_iter(winners.iter().cloned());
    let matches = hand.iter().filter(|x| winner_set.contains(x)).count();
    if matches < 1 {
        return Ok((input, 0));
    }
    let (result, _overflow) = u32::overflowing_pow(2, (matches-1).try_into().unwrap());
    // TODO - check overflow
    Ok((input, result))
}

fn main() {
    let results = 
    fs::read_to_string("results.txt")
    .expect("Calibration file needs to exist.");

    let score: u32 = results.lines().map(|line| {
        let (_, result) = line_parser(line).unwrap();
        return result;
    }).sum();
    println!("Total = {score}");
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_line_parser() {
        assert_eq!(line_parser("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"), Ok(("", 8)));
        assert_eq!(line_parser("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19"), Ok(("", 2)));
        assert_eq!(line_parser("Card 3: 13 32 20 16 61 | 0"), Ok(("", 0)));
    }
}