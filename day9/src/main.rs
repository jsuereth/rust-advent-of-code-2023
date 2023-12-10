
fn solve_next_in_sequence(seq: Vec<i64>) -> i64 {
    // We create a temporary vec of vec to store diff arrays while we work
    let mut cache: Vec<Vec<i64>> = vec!(seq);
    fn is_zeros(seq: &Vec<i64>) -> bool {
        seq.iter(). all(|v| *v == 0)
    }
    let mut cur_seq: &Vec<i64> = cache.get(0).unwrap();
    while !is_zeros(cur_seq) {
        // Calculate diff sequence
        let diff_seq: Vec<i64> =
          cur_seq.as_slice()
            .windows(2)
            .map(|slice| slice[1] - slice[0])
            .collect();
        println!("Calculate diff seq as: {diff_seq:?}");
        cache.push(diff_seq);
        cur_seq = cache.last().unwrap();
    }
    cache.iter().rfold(0, |prev_plus, seq| {
        *seq.last().unwrap() + prev_plus
    })
}

fn part1_solution(seq: Vec<Vec<i64>>) -> i64 {
    seq.iter()
    .cloned()
    .map(solve_next_in_sequence)
    .sum()
}

// TODO - parse input file.
fn parse_input(input: &str) -> Vec<Vec<i64>> {
    input.lines()
    .map(|line| line.split_whitespace().map(|s| s.parse().unwrap()).collect())
    .collect()
}


fn main() {
    let input =
      std::fs::read_to_string("input.txt")
      .expect("Must find solution input!");
    let solution = part1_solution(parse_input(&input)); 
    println!("Part 1 solution: {solution}")
}

mod tests {
    use crate::*;

    #[test]
    fn test_solve_next_in_sequence() {
        assert_eq!(solve_next_in_sequence(vec!(1, 2, 3, 4, 5, 6, 7, 8, 9)), 10);
        assert_eq!(solve_next_in_sequence(vec!(0, 3, 6, 9, 12, 15)), 18);
        assert_eq!(solve_next_in_sequence(vec!(1, 3, 6, 10, 15, 21)), 28);
        assert_eq!(solve_next_in_sequence(vec!(10, 13, 16, 21, 30, 45)), 68);
    }
}
