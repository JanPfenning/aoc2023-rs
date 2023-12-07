use std::{fs, cmp::Ordering};

fn read_puzzle_input() -> String {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let file_path = root.join("src/d07/puzzleinput.txt");
    //println!("{}", file_path.to_string_lossy());
    let contents = fs::read_to_string(file_path)
        .expect(format!("Something went wrong reading the file").as_str());
    contents
}

type Hand = [u8; 5];

#[derive(Debug)]
struct Player {
    hand: Hand,
    hand_type_value: u8,
    bid: usize
}

pub fn p1() {
    let mut players = read_puzzle_input().split("\n")
        .map(|val| {
            let parts = val.split(" ").into_iter().collect::<Vec<&str>>();
            let hand =  parse_hand(parts.get(0).unwrap());
            Player {
                hand: hand,
                hand_type_value: get_highest_type_value(&hand),
                bid: parts.get(1).unwrap().parse().unwrap()
            }
        })
        .collect::<Vec<Player>>();
    players.sort_by(|a, b| a.hand_type_value.cmp(&b.hand_type_value).then_with(|| compare_hands(&a.hand, &b.hand)));
    players.iter().for_each(|player| println!("{player:?}"));
    let total_winnings = players.iter().enumerate().fold(0, |acc, (idx, p)| acc + (idx+1) * p.bid);
    println!("total_winnings: {}", total_winnings)
}

fn compare_hands(hand1: &Hand, hand2: &Hand) -> Ordering {

    for (&card1, &card2) in hand1.iter().zip(hand2) {
        match card1.cmp(&card2) {
            Ordering::Equal => continue,
            non_eq => return non_eq,
        }
    }

    Ordering::Equal
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    use super::*;

    #[test]
    fn test_compare_hands() {
        assert_eq!(compare_hands(&[0, 1, 2, 3, 4], &[0, 1, 3, 4, 5]), Ordering::Less);
        assert_eq!(compare_hands(&[0, 1, 3, 4, 5], &[0, 1, 2, 3, 4]), Ordering::Greater);
        assert_eq!(compare_hands(&[0, 1, 2, 3, 4], &[0, 1, 2, 3, 4]), Ordering::Equal);
    }
}

fn count_cards(hand: &Hand) -> [usize; 13] {
    let mut counts = [0; 13];
    for &card in hand.iter() {
        counts[card as usize] += 1;
    }
    counts
}

fn is_five_of_a_kind(hand: &Hand) -> bool {
    count_cards(hand).iter().any(|&count| count == 5)
}

fn is_four_of_a_kind(hand: &Hand) -> bool {
    count_cards(hand).iter().any(|&count| count == 4)
}

fn is_full_house(hand: &Hand) -> bool {
    let counts = count_cards(hand);
    counts.contains(&3) && counts.contains(&2)
}

fn is_three_of_a_kind(hand: &Hand) -> bool {
    count_cards(hand).iter().any(|&count| count == 3)
}

fn is_two_pair(hand: &Hand) -> bool {
    count_cards(hand).iter().filter(|&&count| count == 2).count() == 2
}

fn is_one_pair(hand: &Hand) -> bool {
    count_cards(hand).iter().any(|&count| count == 2)
}

#[cfg(test)]
mod type_tests {
    use super::*;

    #[test]
    fn test_is_five_of_a_kind() {
        assert!(is_five_of_a_kind(&[0, 0, 0, 0, 0]));
        assert!(!is_five_of_a_kind(&[0, 0, 0, 0, 1]));
    }

    #[test]
    fn test_is_four_of_a_kind() {
        assert!(is_four_of_a_kind(&[0, 0, 0, 0, 1]));
        assert!(!is_four_of_a_kind(&[0, 0, 0, 1, 1]));
    }

    #[test]
    fn test_is_full_house() {
        assert!(is_full_house(&[0, 0, 0, 1, 1]));
        assert!(!is_full_house(&[0, 0, 0, 0, 1]));
    }
    
    #[test]
    fn test_is_three_of_a_kind() {
        assert!(is_three_of_a_kind(&[0, 0, 0, 1, 2]));
        assert!(!is_three_of_a_kind(&[0, 0, 1, 1, 2]));
    }

    #[test]
    fn test_is_two_pair() {
        assert!(is_two_pair(&[0, 0, 1, 1, 2]));
        assert!(!is_two_pair(&[0, 0, 0, 1, 2]));
    }

    #[test]
    fn test_is_one_pair() {
        assert!(is_one_pair(&[0, 0, 1, 2, 3]));
        assert!(!is_one_pair(&[0, 1, 2, 3, 4]));
    }
}

fn get_highest_type_value(hand: &Hand) -> u8 {
    if is_five_of_a_kind(&hand) {
        return 6;
    } else if is_four_of_a_kind(&hand) {
        return 5;
    } else if is_full_house(&hand) {
        return 4;
    } else if is_three_of_a_kind(&hand) {
        return 3;
    } else if is_two_pair(&hand) {
        return 2;
    } else if is_one_pair(&hand) {
        return 1;
    }  
    0
}

fn parse_hand(hand: &str) -> [u8; 5] {
    assert_eq!(hand.chars().count(), 5);
    println!("{:?}", hand.chars());
    let result: [u8; 5] = hand.chars().map(|c| match c {
        '2'..='9' => c as u8 - '2' as u8,
        'T' => 8,
        'J' => 9,
        'Q' => 10,
        'K' => 11,
        'A' => 12,
        _ => panic!("Invalid character"),
    }).collect::<Vec<_>>().try_into().expect("Wrong number of elements");
    
    result
}

#[cfg(test)]
mod parse_hand_tests {
    use super::*;

    #[test]
    fn test_parse_hand() {
        let test_cases = vec![
            ("23456", [0, 1, 2, 3, 4]),
            ("789JK", [5, 6, 7, 9, 11]),
            ("QKAT3", [10, 11, 12, 8, 1]),
            ("2349A", [0, 1, 2, 7, 12]),
        ];

        for (hand, expected) in test_cases {
            let result = parse_hand(hand);

            assert_eq!(result, expected);
        }
    }

    #[test]
    #[should_panic(expected = "Invalid character")]
    fn test_parse_hand_invalid_char() {
        parse_hand("23X56"); // X is an invalid character. 
    }

    #[test]
    #[should_panic(expected = "assertion failed")]
    fn test_parse_hand_invalid_length() {
        parse_hand("234567"); // Length is 6, not 5.
    }
}