
struct RaceRecord {
    time: i64,
    distance: i64,
}

// How a boat moves:
// Time pushed = millis/second.
// distance in millis
// time in second.
//
//  (max_time - time_pushed) * time_pushed > distance
//  max_time*time_pushed - time_pushed^2 > distance
// -time_pushed^2 + max_time*time_pushed - distance > 0

// max_time +/- sqrt(max_time^2 - 4*time_pushed*distance)/-2*time_pushed

fn number_of_winning_strategies(record: &RaceRecord) -> u32 {
    // Brute force method
    (0..record.time).filter(|time_pushed| (record.time-time_pushed)*time_pushed>record.distance).count().try_into().unwrap()
}

fn part_one(records: Vec<RaceRecord>) -> u32 {
    records.iter().map(|r| number_of_winning_strategies(r)).fold(1, |acc, cur| acc*cur)
}

fn main() {
    // Part 1
    // Time:        38     67     76     73
    // Distance:   234   1027   1157   1236

    let result1 = part_one(vec!(RaceRecord{
        time: 38,
        distance: 234,
    },RaceRecord{
        time: 67,
        distance: 1027,
    },RaceRecord{
        time: 76,
        distance: 1157,
    },RaceRecord{
        time: 73,
        distance: 1236,
    }));
    println!("Part one answer is: {result1}");
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_winning_strat() {
        assert_eq!(number_of_winning_strategies(&RaceRecord{
            time: 7,
            distance: 9,
        }), 4);
        assert_eq!(number_of_winning_strategies(&RaceRecord{
            time: 15,
            distance: 40,
        }), 8);
        assert_eq!(number_of_winning_strategies(&RaceRecord{
            time: 30,
            distance: 200,
        }), 9);
    }
}
