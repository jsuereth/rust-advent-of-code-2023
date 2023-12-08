use std::cmp::Ordering;
use std::cmp::max;
use std::fmt;
use std::convert::From;
use std::convert::TryFrom;
use enum_ordinalize::Ordinalize;
use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, Ordinalize, Clone)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard
}

#[derive(Clone)]
pub struct Hand {
    // Assume all contents are sorted upon creation.
    // We use numbers: 1=1, 2=2, etc. until 10=T, 11=J, 12=Q, 13=K, 14=A
    contents: [u8;5],
    // Classification of the Hand.
    htype: HandType,
}


impl fmt::Debug for Hand {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let hand_string: String = self.contents.iter().map(|c| match c {
      1 => '1',
      2 => '2',
      3 => '3',
      4 => '4',
      5 => '5',
      6 => '6',
      7 => '7',
      8 => '8',
      9 => '9',
      10 => 'T',
      11 => 'J',
      12 => 'Q',
      13 => 'K',
      14 => 'A',
      _ => ' ',
    }).collect();
    write!(f, "{}", hand_string)
  }
}

// Simplify creating a hand from the card array.
impl From<[u8;5]> for Hand {
    fn from(cards: [u8;5]) -> Hand {
        Hand {
            contents: cards,
            htype: classify_hand(cards),
        }
    }
}

fn classify_hand(cards: [u8;5]) -> HandType {
  // TODO - what else do we need to remember? 
  let mut max_dupes = 0;
  let mut num_pairs = 0;
  for (_, duplicates) in &cards.iter().sorted().group_by(|c| *c) {
    let count = duplicates.count();
    max_dupes = max(max_dupes, count);
    if count == 2 {
      num_pairs += 1;
    }
  }
  match max_dupes {
    5 => HandType::FiveOfAKind,
    4 => HandType::FourOfAKind,
    3 => if num_pairs > 0 {HandType::FullHouse} else {HandType::ThreeOfAKind},
    2 => if num_pairs > 1 {HandType::TwoPair} else {HandType::OnePair},
    _ => HandType::HighCard,
  }  
}

fn compare_hands_by_card(lhs: &Hand, rhs: &Hand) -> Ordering {
    for (lc, rc) in lhs.contents.iter().zip(rhs.contents.iter()) {
      if lc > rc {
        return Ordering::Greater;
      }
      if rc > lc {
        return Ordering::Less;
      }
    }
    return Ordering::Equal;
}

impl PartialEq for Hand {
    fn eq(&self, other: &Hand) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}
impl Eq for Hand {}
impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Hand) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        // TODO - Sort poker cards.
        if self.htype.ordinal() < other.htype.ordinal() {
            Ordering::Greater
        } else if self.htype.ordinal() > other.htype.ordinal() {
            Ordering::Less
        } else {
            compare_hands_by_card(self, other)
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::hand::*;

    #[test]
    fn test_hand() {
        let hand = Hand::from([1,2,1,1,5]);
        assert_eq!(hand.htype, HandType::ThreeOfAKind);
    }

    #[test]
    fn test_sort() {
      let start =
        vec!(
          Hand::from([3, 2, 10, 3, 13]),
          Hand::from([10, 5, 5, 11, 5]),
          Hand::from([13, 13, 11, 11, 10]),
          Hand::from([13, 10, 11, 11, 10]),
          Hand::from([12, 12, 12, 11, 14]),
        );
      let sorted: Vec<Hand> = start.iter().sorted().cloned().collect();
      assert_eq!(sorted,
        vec!(
          Hand::from([3, 2, 10, 3, 13]),
          Hand::from([13, 10, 11, 11, 10]),
          Hand::from([13, 13, 11, 11, 10]),
          Hand::from([10, 5, 5, 11, 5]),
          Hand::from([12, 12, 12, 11, 14]),
        ));
    }
}
