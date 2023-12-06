
use nom::{Parser,IResult};
use nom::sequence::tuple;
use nom::bytes::complete::tag;
use nom::multi::separated_list1;
use nom::error::context;
use nom::character::complete::{space0, space1, u32};
use std::collections::HashSet;
use std::fs;

fn line_parser(input: &str) -> IResult<&str, usize> {
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
    let matches = 
      hand.iter()
      .filter(|x| winner_set.contains(x))
      .count();
    Ok((input, matches))
}

// Part two result
fn part_two(results: &str) -> u32 {
    let copies: Vec<usize> = results.lines().map(|line| {
        let (_, result) = line_parser(line).unwrap();
        return result;
    }).collect();
    // Number of cards
    let size = copies.len();
    // Create a new vector to remember the amount of cards we've experienced.
    let mut card_counts = vec![1; size];
    for (idx, matches) in copies.iter().enumerate() {
        // For next N cards...
        for j in idx+1 ..(idx+1+matches) {
            // Copy further cards by the amount of our card
            // we found.
            card_counts[j] += card_counts[idx];
        }
    }
    return card_counts.iter().sum();
}

fn main() {
    let results = 
    fs::read_to_string("results.txt")
    .expect("Calibration file needs to exist.");

    let score: u32 = part_two(&results);
    println!("Total = {score}");
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_line_parser() {
        assert_eq!(line_parser("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"), Ok(("", 4)));
        assert_eq!(line_parser("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19"), Ok(("", 2)));
        assert_eq!(line_parser("Card 3: 13 32 20 16 61 | 0"), Ok(("", 0)));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\n\
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\n\
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\n\
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\n\
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\n\
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"), 30)
    }
}