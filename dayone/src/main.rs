use std::fs;

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

fn decode_msg(msg: &str) -> u32 {
    msg.lines().map(|line| decode_line(line)).sum()
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
    use crate::decode_msg;

    #[test]
    fn it_works() {
        let result = 
        decode_msg("1abc2\n\
                    pqr3stu8vwx\n\
                    a1b2c3d4e5f\n\
                    treb7uchet");
        assert_eq!(result, 142);
    }
}