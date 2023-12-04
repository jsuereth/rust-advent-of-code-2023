
use nom::{Parser,IResult};
use nom::error::context;
use nom::bytes::complete::{tag,tag_no_case};
use nom::character::complete::{space0, space1, u32};
use nom::sequence::tuple;
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::branch::alt;
use std::fs;

#[derive(Debug, Default, PartialEq)]
struct ColorResult {
    red: u32,
    green: u32,
    blue: u32,
}

// parses game number
fn game(input: &str) -> IResult<&str, u32> {
    context("game number",
      tuple((tag_no_case("game"), space1, u32, tag_no_case(":"))).map(|(_, _, id, _)| id)
    )(input)
}

fn color_result(input: &str) -> IResult<&str, ColorResult> {
  let blue_result = tuple((space0, u32, space1, tag_no_case("blue"))).map(|(_,num,_,_)| ColorResult { blue: num, ..Default::default() });
  let red_result = tuple((space0, u32, space1, tag_no_case("red"))).map(|(_,num,_,_)| ColorResult { red: num, ..Default::default() });
  let green_result = tuple((space0, u32, space1, tag_no_case("green"))).map(|(_,num,_,_)| ColorResult { green: num, ..Default::default() });
  context("color result", alt((blue_result, red_result, green_result)))(input)
}

fn draw_results(input: &str) -> IResult<&str, ColorResult> {
  map(separated_list1(tag(","), color_result), |v| v.iter().fold(Default::default(), |acc: ColorResult, next| ColorResult {
    red: acc.red + next.red,
    blue: acc.blue + next.blue,
    green: acc.green + next.green,
  }))(input)
}

fn game_results(input: &str) -> IResult<&str, (u32, Vec<ColorResult>)> {
    let all_draws = separated_list1(tag(";"), draw_results);
    tuple((game, all_draws))(input)
}


fn possible_game_sums(input: &str) -> u32 {
    input.lines().map(|line| {
        let (rest, result) = game_results(line).unwrap();
        return result;
    })
    .filter(|(id, results)| results.iter().all(|result| result.red <= 12 && result.green <= 13 && result.blue <= 14))
    .map(|(id, _)| id)
    .sum()
}


fn main() {
    let results = 
      fs::read_to_string("results.txt")
      .expect("Calibration file needs to exist.");
    let sum = possible_game_sums(&results);
    println!("Game sum = {sum}")
}



#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn game_parser() {
        assert_eq!(game("Game 2:"), Ok(("", 2)));
        assert_eq!(game("Game 10: 3 blue, 2 red"), Ok((" 3 blue, 2 red", 10)));
        assert_eq!(game("Game  11: 3 blue, 2 red"), Ok((" 3 blue, 2 red", 11)));
    }
    #[test]
    fn color_result_parser() {
        assert_eq!(color_result("1 red"), Ok(("", ColorResult { red: 1, ..Default::default() })));
        assert_eq!(color_result("3 blue, 2 red"), Ok((", 2 red", ColorResult { blue: 3, ..Default::default() })));
        assert_eq!(color_result("4 green, 2 red"), Ok((", 2 red", ColorResult { green: 4, ..Default::default() })));
    }
    #[test]
    fn draw_results_parser() {
        assert_eq!(draw_results("1 red"), Ok(("", ColorResult { red: 1, ..Default::default() })));
        assert_eq!(draw_results(" 3 blue, 2 red"), Ok(("", ColorResult { blue: 3, red: 2, ..Default::default() })));
        assert_eq!(draw_results("4 green, 2 red"), Ok(("", ColorResult { green: 4, red: 2, ..Default::default() })));
    }

    #[test]
    fn game_results_parser() {
        assert_eq!(game_results("Game 10: 3 blue, 2 red; 1 green"), Ok(("", (10, vec!(ColorResult { red:2,blue:3,green:0}, ColorResult{green:1,red:0,blue:0})))));
    }

    #[test]
    fn test_possible_game_sums() {
        assert_eq!(possible_game_sums("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\n\
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\n\
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\n\
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\n\
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"), 8);
    }
}