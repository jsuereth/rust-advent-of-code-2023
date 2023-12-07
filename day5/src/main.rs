use nom::{Parser,IResult};
use nom::multi::{separated_list1, many1};
use nom::error::context;
use nom::sequence::{terminated, tuple};
use nom::bytes::complete::tag;
use nom::character::complete::{
    space0, 
    space1, 
    i64, 
    line_ending,
    none_of
};
use std::fs;


// Seed -> Soil
// Soil -> Fertilizer
// Fertilizer -> Water
// Water -> Light
// Light -> Temp
// Temp -> Humidity
// Humidity -> Location

#[derive(PartialEq, Debug)]
struct LookupRange {
    dest_range_start: i64,
    source_range_start: i64,
    length: i64,
}

#[derive(PartialEq, Debug)]
struct Almanac {
    seeds: Vec<i64>,
    mappings: Vec<Vec<LookupRange>>,
}

// Uses one range and either maps or doesn't.
fn lookup(source: i64, range: &LookupRange) -> Option<i64> {
    let idx = source - range.source_range_start;
    if idx >= 0 && idx < range.length {
        Some(range.dest_range_start+idx)
    } else {
        None
    }
}
// Uses ranges, in order, to map ids.  Fallback to same id.
fn lookup_all(source: i64, ranges: &Vec<LookupRange>) -> i64 {
    ranges.iter().fold(None, |result, next_range| result.or_else(|| lookup(source, next_range))).unwrap_or(source)
}

// Lookup data across all mappings.
fn lookup_across(source: i64, mappings: &Vec<Vec<LookupRange>>) -> i64 {
    mappings.iter().fold(source, |id, next_mappings| lookup_all(id, next_mappings))
}

// Parse a single range line.
fn parse_range(input: &str) -> IResult<&str, LookupRange> {
    let (input, _) = space0(input)?;
    let (input, dest_range_start) = i64(input)?;
    let (input, _) = space1(input)?;
    let (input, source_range_start) = i64(input)?;
    let (input, _) = space1(input)?;
    let (input, length) = i64(input)?;
    let (input, _) = space0(input)?;
    Ok((input, LookupRange {
        dest_range_start,
        source_range_start,
        length,
    }))
}


// Parse the mapping list, not including the header.
fn parse_mapping(input: &str) -> IResult<&str, Vec<LookupRange>> {
    separated_list1(line_ending, parse_range)(input)
}

// parse full mapping with name head
fn parse_mapping_section(input: &str) -> IResult<&str, Vec<LookupRange>> {
    let (input, _) = 
      context("section header", 
        tuple((many1(none_of(":")), tag(":"), line_ending))
      )(input)?;
    let (input, mapping) = context("mapping section", parse_mapping)(input)?;
    Ok((input, mapping))
}

// parse seed list
fn parse_seeds(input: &str) -> IResult<&str, Vec<i64>> {
    let (input, _) = tag("seeds:")(input)?;
    let (input, _) = space0(input)?;
    let (input, seeds) = separated_list1(space1, i64)(input)?;
    Ok((input, seeds))
}

// parse input file
fn parse_input(input: &str) -> IResult<&str, Almanac> {
    let (input, seeds) = parse_seeds(input)?;
    let (input, _) = line_ending(input)?;
    let (input, mappings) = 
      separated_list1(line_ending, parse_mapping_section)(input)?;
    Ok((input, Almanac {
        seeds,
        mappings,
    }))
}

fn part_one_answer(input: &str) -> i64 {
    let (_, almanac) = 
      parse_input(input)
      .expect("Unable to parse input");
    almanac.seeds.iter()
      .map(|seed| lookup_across(*seed, &almanac.mappings))
      .min()
      .unwrap()
}

// Super brute force, slow solution.
fn part_two_answer(input: &str) -> i64 {
    let (_, almanac) = 
      parse_input(input)
      .expect("Unable to parse input");
    almanac.seeds.chunks(2)
      .flat_map(|arr| {
        let start = arr[0];
        let length = arr[1];
        start..(start+length)
      })
      .map(|seed| lookup_across(seed, &almanac.mappings))
      .min()
      .unwrap()
}

fn main() {
    let results = 
      fs::read_to_string("input.txt")
      .expect("Calibration file needs to exist.");
    let result = part_two_answer(&results);
    println!("The smallest location is {result}")
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_lookup() {
        let test_range = LookupRange {
            dest_range_start: 50,
            source_range_start: 98,
            length: 2,
        };
        assert_eq!(lookup(97, &test_range), None);
        assert_eq!(lookup(98, &test_range), Some(50));
        assert_eq!(lookup(99, &test_range), Some(51));        
        assert_eq!(lookup(100, &test_range), None);
    }

    #[test]
    fn test_lookup_all() {
        let test_ranges = vec!(LookupRange {
            dest_range_start: 50,
            source_range_start: 98,
            length: 2,
        }, LookupRange {
            dest_range_start: 52,
            source_range_start: 50,
            length: 48,
        });
        assert_eq!(lookup_all(1, &test_ranges), 1);
        assert_eq!(lookup_all(50, &test_ranges), 52);
        assert_eq!(lookup_all(51, &test_ranges), 53);
        assert_eq!(lookup_all(97, &test_ranges), 99);
        assert_eq!(lookup_all(98, &test_ranges), 50);
        assert_eq!(lookup_all(99, &test_ranges), 51);        
        assert_eq!(lookup_all(100, &test_ranges), 100);
    }
    #[test]
    fn test_lookup_across() {
        let mappings = vec!(
            // Seed to SOil
            vec!(LookupRange {
                dest_range_start: 50,
                source_range_start: 98,
                length: 2,
            }, LookupRange {
                dest_range_start: 52,
                source_range_start: 50,
                length: 48,
            }),
            // Soil to Fertilizer
            vec!(LookupRange {
                dest_range_start: 0,
                source_range_start: 15,
                length: 37,
            }, LookupRange {
                dest_range_start: 37,
                source_range_start: 52,
                length: 2,
            }, LookupRange {
                dest_range_start: 39,
                source_range_start: 0,
                length: 15,
            }),
            // Fertilizer to Water
            vec!(LookupRange {
                dest_range_start: 49,
                source_range_start: 53,
                length: 8,
            }, LookupRange {
                dest_range_start: 0,
                source_range_start: 11,
                length: 42,
            }, LookupRange {
                dest_range_start: 42,
                source_range_start: 0,
                length: 7,
            }, LookupRange {
                dest_range_start: 57,
                source_range_start: 7,
                length: 4,
            }),
            // Water to Light
            vec!(LookupRange {
                dest_range_start: 88,
                source_range_start: 18,
                length: 7,
            }, LookupRange {
                dest_range_start: 18,
                source_range_start: 25,
                length: 70,
            }),
            // Light to Temperature
            vec!(LookupRange {
                dest_range_start: 45,
                source_range_start: 77,
                length: 23,
            }, LookupRange {
                dest_range_start: 81,
                source_range_start: 45,
                length: 19,
            }, LookupRange {
                dest_range_start: 68,
                source_range_start: 64,
                length: 13,
            }),
            // Temperature to Humidity
            vec!(LookupRange {
                dest_range_start: 0,
                source_range_start: 69,
                length: 1,
            }, LookupRange {
                dest_range_start: 1,
                source_range_start: 0,
                length: 69,
            }),
            // Humidity to Location
            vec!(LookupRange {
                dest_range_start: 60,
                source_range_start: 56,
                length: 37,
            }, LookupRange {
                dest_range_start: 56,
                source_range_start: 93,
                length: 4,
            }),
        );
        assert_eq!(lookup_across(79, &mappings), 82);
        assert_eq!(lookup_across(14, &mappings), 43);
        assert_eq!(lookup_across(55, &mappings), 86);
        assert_eq!(lookup_across(13, &mappings), 35);
    }

    #[test]
    fn test_parse_section() {
        let (rest, result) =
          parse_mapping("0 15 37\n\
                         37 52 2\n\
                         39 0 15").unwrap();
        assert_eq!(rest, "");
        assert_eq!(result,  vec!(LookupRange {
            dest_range_start: 0,
            source_range_start: 15,
            length: 37,
        }, LookupRange {
            dest_range_start: 37,
            source_range_start: 52,
            length: 2,
        }, LookupRange {
            dest_range_start: 39,
            source_range_start: 0,
            length: 15,
        }));
    }

    #[test]
    fn test_parse_mapping_section() {
        let (rest, result) =
          parse_mapping_section("seed-to-soil map:\n\
                                 50 98 2\n\
                                 52 50 48").unwrap();
        assert_eq!(rest, "");
        assert_eq!(result, vec!(LookupRange {
            dest_range_start: 50,
            source_range_start: 98,
            length: 2,
        }, LookupRange {
            dest_range_start: 52,
            source_range_start: 50,
            length: 48,
        }));
    }
    #[test]
    fn test_parse_seeds() {
        let (rest, result) = 
          parse_seeds("seeds: 79 14 55 13").unwrap();
        assert_eq!(rest, "");
        assert_eq!(result, vec!(79, 14, 55, 13));
    }
    #[test]
    fn test_parse_input_file() {
        let (rest, almanac) =
          parse_input("seeds: 79 14 55 13\n\
          \n\
          seed-to-soil map:\n\
          50 98 2\n\
          52 50 48\n\
          \n\
          soil-to-fertilizer map:\n\
          0 15 37\n\
          37 52 2\n\
          39 0 15\n\
          \n\
          fertilizer-to-water map:\n\
          49 53 8\n\
          0 11 42\n\
          42 0 7\n\
          57 7 4\n\
          \n\
          water-to-light map:\n\
          88 18 7\n\
          18 25 70\n\
          \n\
          light-to-temperature map:\n\
          45 77 23\n\
          81 45 19\n\
          68 64 13\n\
          \n\
          temperature-to-humidity map:\n\
          0 69 1\n\
          1 0 69\n\
          \n\
          humidity-to-location map:\n\
          60 56 37\n\
          56 93 4").unwrap();
        assert_eq!(rest, "");
        assert_eq!(almanac.seeds, vec!(79, 14, 55, 13));
        assert_eq!(almanac.mappings.len(), 7);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part_one_answer("seeds: 79 14 55 13\n\
        \n\
        seed-to-soil map:\r\n\
        50 98 2\r\n\
        52 50 48\n\
        \n\
        soil-to-fertilizer map:\n\
        0 15 37\n\
        37 52 2\n\
        39 0 15\n\
        \n\
        fertilizer-to-water map:\n\
        49 53 8\n\
        0 11 42\n\
        42 0 7\n\
        57 7 4\n\
        \n\
        water-to-light map:\n\
        88 18 7\n\
        18 25 70\n\
        \n\
        light-to-temperature map:\n\
        45 77 23\n\
        81 45 19\n\
        68 64 13\n\
        \n\
        temperature-to-humidity map:\n\
        0 69 1\n\
        1 0 69\n\
        \n\
        humidity-to-location map:\n\
        60 56 37\n\
        56 93 4"), 35);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part_two_answer("seeds: 79 14 55 13\n\
        \n\
        seed-to-soil map:\r\n\
        50 98 2\r\n\
        52 50 48\n\
        \n\
        soil-to-fertilizer map:\n\
        0 15 37\n\
        37 52 2\n\
        39 0 15\n\
        \n\
        fertilizer-to-water map:\n\
        49 53 8\n\
        0 11 42\n\
        42 0 7\n\
        57 7 4\n\
        \n\
        water-to-light map:\n\
        88 18 7\n\
        18 25 70\n\
        \n\
        light-to-temperature map:\n\
        45 77 23\n\
        81 45 19\n\
        68 64 13\n\
        \n\
        temperature-to-humidity map:\n\
        0 69 1\n\
        1 0 69\n\
        \n\
        humidity-to-location map:\n\
        60 56 37\n\
        56 93 4"), 46);
    }
}