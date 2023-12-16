
use itertools::Itertools;

mod grid;

fn find_expanded_rows_cols(grid: &grid::Grid) -> (Vec<usize>, Vec<usize>) {
    let expanded_rows: Vec<usize> =
        (0..grid.rows())
        .filter(|row| grid.row_iter(*row).all(|c| c.value() == '.'))
        .collect();
    let expanded_cols: Vec<usize> =
      (0..grid.cols())
      .filter(|col| grid.col_iter(*col).all(|c| c.value() == '.'))
      .collect();
    (expanded_rows, expanded_cols)
}

fn find_galaxies(grid: &grid::Grid) -> Vec<(usize,usize)> {
    grid.iter()
    .filter(|(_, c)| c.value() == '#')
    .map(|(xy, _)| xy)
    .collect()
}

// Helps calculate distance with galaxy expansion
struct DistanceHelper {
    expanded_rows: Vec<usize>,
    expanded_cols: Vec<usize>
}

impl DistanceHelper {
    // TODO - we require a + b to be sorted top left to bottom right.
    fn distance(&self, a: (usize, usize), b: (usize, usize)) -> usize {
        // Part 1 - this is 2
        let expansion_amount = 1000000;
        let double_rows =
          self.expanded_rows.iter()
          .copied()
          .filter(|row| {
            if a.0 > b.0 {
                row > &b.0 && row < &a.0
            } else {
                row > &a.0 && row < &b.0
            }
          }).count();
        let double_cols =
          self.expanded_cols.iter()
          .copied()
          .filter(|col| {
            if a.1 > b.1 {
                col > &b.1 && col < &a.1
            } else {
                col > &a.1 && col < &b.1
            }
          }).count();
        let row_diff = a.0.abs_diff(b.0);
        let col_diff = a.1.abs_diff(b.1);
        row_diff + (double_rows*(expansion_amount-1)) + col_diff + (double_cols*(expansion_amount-1))
    }
}

fn solve_part1(input: &str) -> usize {
    let grid: grid::Grid = input.into();
    let (er, ec) = find_expanded_rows_cols(&grid);
    let helper = DistanceHelper {
        expanded_cols: ec,
        expanded_rows: er,
    };
    let galaxies = find_galaxies(&grid);

    galaxies.iter().tuple_combinations()
    .map(|(g1, g2)| helper.distance(*g1, *g2))
    .sum()
}

fn main() {
    let input =
      std::fs::read_to_string("input.txt")
      .expect("Must find solution input!");
    let result = solve_part1(&input);
    println!("Result: {result}");
}

mod tests {
    use crate::*;

    #[test]
    fn test_solution_1() {
        let input = "...#......\n\
        .......#..\n\
        #.........\n\
        ..........\n\
        ......#...\n\
        .#........\n\
        .........#\n\
        ..........\n\
        .......#..\n\
        #...#.....";
        assert_eq!(solve_part1(input), 374);
    }

    #[test]
    fn test_distance_helper() {
        let helper = DistanceHelper {
            expanded_rows: vec!(3, 7),
            expanded_cols: vec!(2, 5, 8),
        };
        assert_eq!(helper.distance((0,0), (1,1)), 2);
        assert_eq!(helper.distance((9,0), (9,4)), 5);
        assert_eq!(helper.distance((0,3), (8,7)), 15);
    }

    #[test]
    fn find_galaxies_works() {
        let grid: grid::Grid = 
          "....#........\n\
          .........#...\n\
          #............\n\
          .............\n\
          .............\n\
          ........#....\n\
          .#...........\n\
          ............#\n\
          .............\n\
          .............\n\
          .........#...\n\
          #....#.......".into();
        let galaxies = find_galaxies(&grid);
        assert_eq!(galaxies, 
            vec!((0, 4), 
                 (1, 9),
                 (2, 0),
                 (5, 8),
                 (6, 1),
                 (7, 12),
                 (10, 9),
                 (11, 0),
                 (11, 5)));

    }
    #[test]
    fn find_expansion_works() {
        let grid: grid::Grid = 
          "...#......\n\
          .......#..\n\
          #.........\n\
          ..........\n\
          ......#...\n\
          .#........\n\
          .........#\n\
          ..........\n\
          .......#..\n\
          #...#.....".into();
      let (er, ec) = find_expanded_rows_cols(&grid);
      assert_eq!(er, vec!(3, 7));
      assert_eq!(ec, vec!(2, 5, 8));
    }
}