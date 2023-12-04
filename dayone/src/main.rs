use std::fs;
use nom::IResult;
use nom::branch::alt;
use nom::bytes::complete::tag_no_case;
use nom::character::complete::anychar;
use nom::error::context;
use nom::combinator::{map, value};
use nom::multi::many1;

// Part 1 - Solution using simple iteration
fn char_to_int(first: &char, last: &char) -> u32 {
    first.to_digit(10).unwrap()*10 + last.to_digit(10).unwrap()
}


fn decode_line(line: &str) -> u32 {
    let result: Vec<char> =
      line.chars()
      .filter(|x| x.is_numeric())
      .collect();
    match result.as_slice() {
        [one] => char_to_int(one, one),
        [first, .., last] => char_to_int(first, last),
        _ => 0,
    }
}

// Part 2 - Compose a parser/tokenizer using nom.

fn numeric_numbers(input: & str) -> IResult<&str, u32> {
    context("written numbers",
       alt(
        (value(1,tag_no_case("1")), 
         value(2, tag_no_case("2")),
         value(3, tag_no_case("3")),
         value(4, tag_no_case("4")),
         value(5, tag_no_case("5")),
         value(6, tag_no_case("6")),
         value(7, tag_no_case("7")),
         value(8, tag_no_case("8")),
         value(9, tag_no_case("9")),
         // value(0, tag_no_case("0")),
        ))
    )(input)
}

fn english_numbers(input: &str) -> IResult<&str, u32> {
    context("written numbers",
       alt(
        (value(1, tag_no_case("one")), 
         value(2, tag_no_case("two")),
         value(3, tag_no_case("three")),
         value(4, tag_no_case("four")),
         value(5, tag_no_case("five")),
         value(6, tag_no_case("six")),
         value(7, tag_no_case("seven")),
         value(8, tag_no_case("eight")),
         value(9, tag_no_case("nine")),
        // value(0, tag_no_case("zero")),
        ))
    )(input)
}

// Pulls the next token of input.
// This will return either: Some(number) or None
// via the following in-order patterns:
// 1. numeric characters 0-9
// 2. numeric words zero -> nine
// 3. Any other remaining character
fn token_parser(input: &str) -> IResult<&str, Option<u32>> {
    alt((
        map(numeric_numbers, |r| Some(r)), 
        map(english_numbers, |r| Some(r)),
        map(anychar, |_| None)
    ))(input)
}

// Part two's tokenizer + parser.
// We get a Vec<Option<u32>> where the option is filled out when numbers exist.
fn decode_line2(line: &str) -> u32 {
    let (rest, result) = many1(token_parser)(line).unwrap();
    assert_eq!(rest, "");
    let collapsed: Vec<u32> = result.iter().filter_map(|x| *x).collect();
    match collapsed.as_slice() {
        [one] => one*10 + one,
        [first, .., last] => first*10 + last,
        _ => 0,
    }
}

fn decode_msg(msg: &str) -> u32 {
    msg.lines().map(|line| {
      let result = decode_line2(line);
      println!("{line} results in {result}");
      return result
    }).sum()
}

fn main() {
    let calibration = 
    fs::read_to_string("calibration.txt")
    .expect("Calibration file needs to exist.");
    let result = decode_msg(&calibration);
    println!("The answer is {result}!");
}


#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn it_works() {
        let result = 
        decode_msg("1abc2\n\
                    pqr3stu8vwx\n\
                    a1b2c3d4e5f\n\
                    treb7uchet");
        assert_eq!(result, 142);
    }

    #[test]
    fn part_two_works() {
        let result =
        decode_msg("two1nine\n\
            eightwothree\n\
            abcone2threexyz\n\
            xtwone3four\n\
            4nineeightseven2\n\
            zoneight234\n\
            7pqrstsixteen");
        assert_eq!(result, 281)
    }

    #[test]
    fn part_two_helpers() {
        let result =
          decode_msg("1fooo2\n\
                      twoandthree\n\
                      a123459");
        assert_eq!(result, 12+23+19);
    }
    #[test]
    fn part_two_english_parser() {
        assert_eq!(english_numbers("one"), Ok(("", 1)));
        assert_eq!(english_numbers("twoalpha"), Ok(("alpha", 2)));
        assert_eq!(english_numbers("three"), Ok(("", 3)));
        assert_eq!(english_numbers("seveneightnine"), Ok(("eightnine", 7)));
    }
    #[test]
    fn part_two_number_parser() {
        assert_eq!(numeric_numbers("1"), Ok(("", 1)));
        assert_eq!(numeric_numbers("2alpha"), Ok(("alpha", 2)));
        assert_eq!(numeric_numbers("3"), Ok(("", 3)));
        assert_eq!(numeric_numbers("789"), Ok(("89", 7)));
    }

    #[test]
    fn part_two_token_parser() {
        assert_eq!(token_parser("1twothreefour"), Ok(("twothreefour", Some(1))));
        assert_eq!(token_parser("twoalpha"), Ok(("alpha", Some(2))));
        assert_eq!(token_parser("ab13"), Ok(("b13", None)));
    }
}