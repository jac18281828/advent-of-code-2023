use std::collections::HashMap;
use std::io;

use anyhow::Error;
use tracing::Level;

#[derive(Debug, PartialEq, Eq, Clone)]
struct Game {
    hand: String,
    bid: usize,
}

fn cmp_suit(a: char, b: char) -> std::cmp::Ordering {
    let a = match a {
        'T' => 10,
        'J' => 11,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => a.to_digit(10).unwrap(),
    };
    let b = match b {
        'T' => 10,
        'J' => 11,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => b.to_digit(10).unwrap(),
    };
    a.cmp(&b)
}

impl Game {
    fn cmp_suit(&self, other: &Self) -> std::cmp::Ordering {
        for (a, b) in self.hand.chars().zip(other.hand.chars()) {
            let cmp = cmp_suit(a, b);
            if cmp != std::cmp::Ordering::Equal {
                return cmp;
            }
        }
        std::cmp::Ordering::Equal
    }
}

impl Ord for Game {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if is_five_of_kind(&self.hand) && !is_five_of_kind(&other.hand) {
            std::cmp::Ordering::Greater
        } else if !is_five_of_kind(&self.hand) && is_five_of_kind(&other.hand) {
            std::cmp::Ordering::Less
        } else if is_five_of_kind(&self.hand) && is_five_of_kind(&other.hand) {
            self.cmp_suit(other)
        } else if is_four_of_kind(&self.hand) && !is_four_of_kind(&other.hand) {
            std::cmp::Ordering::Greater
        } else if !is_four_of_kind(&self.hand) && is_four_of_kind(&other.hand) {
            std::cmp::Ordering::Less
        } else if is_four_of_kind(&self.hand) && is_four_of_kind(&other.hand) {
            self.cmp_suit(other)
        } else if is_full_house(&self.hand) && !is_full_house(&other.hand) {
            std::cmp::Ordering::Greater
        } else if !is_full_house(&self.hand) && is_full_house(&other.hand) {
            std::cmp::Ordering::Less
        } else if is_full_house(&self.hand) && is_full_house(&other.hand) {
            self.cmp_suit(other)
        } else if is_three_of_kind(&self.hand) && !is_three_of_kind(&other.hand) {
            std::cmp::Ordering::Greater
        } else if !is_three_of_kind(&self.hand) && is_three_of_kind(&other.hand) {
            std::cmp::Ordering::Less
        } else if is_three_of_kind(&self.hand) && is_three_of_kind(&other.hand) {
            self.cmp_suit(other)
        } else if is_two_pair(&self.hand) && !is_two_pair(&other.hand) {
            std::cmp::Ordering::Greater
        } else if !is_two_pair(&self.hand) && is_two_pair(&other.hand) {
            std::cmp::Ordering::Less
        } else if is_two_pair(&self.hand) && is_two_pair(&other.hand) {
            self.cmp_suit(other)
        } else if is_pair(&self.hand) && !is_pair(&other.hand) {
            std::cmp::Ordering::Greater
        } else if !is_pair(&self.hand) && is_pair(&other.hand) {
            std::cmp::Ordering::Less
        } else if is_pair(&self.hand) && is_pair(&other.hand) {
            self.cmp_suit(other)
        } else if is_high_card(&self.hand) && !is_high_card(&other.hand) {
            std::cmp::Ordering::Greater
        } else if !is_high_card(&self.hand) && is_high_card(&other.hand) {
            std::cmp::Ordering::Less
        } else if is_high_card(&self.hand) && is_high_card(&other.hand) {
            self.cmp_suit(other)
        } else {
            std::cmp::Ordering::Equal
        }
    }
}

impl PartialOrd for Game {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn is_card(c: char) -> bool {
    matches!(
        c,
        '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | 'T' | 'J' | 'Q' | 'K' | 'A'
    )
}

fn is_hand(hand: &str) -> bool {
    hand.len() == 5 && hand.chars().all(is_card)
}

fn is_full_house(hand: &str) -> bool {
    is_three_of_kind(hand) && is_pair(hand)
}

fn is_five_of_kind(hand: &str) -> bool {
    let mut chars = hand.chars();
    let c = chars.next().unwrap();
    chars.all(|x| x == c)
}

fn is_n_of_kind(hand: &str, n: usize) -> bool {
    let chars = hand.chars();
    let mut c_map = HashMap::new();
    for c in chars {
        let count = c_map.entry(c).or_insert(0);
        *count += 1;
    }
    c_map.values().any(|x| *x == n)
}

fn is_two_pair(hand: &str) -> bool {
    let chars = hand.chars();
    let mut c_map = HashMap::new();
    for c in chars {
        let count = c_map.entry(c).or_insert(0);
        *count += 1;
    }
    c_map.values().filter(|x| **x == 2).count() == 2
}

fn is_high_card(hand: &str) -> bool {
    let chars = hand.chars();
    let mut c_map = HashMap::new();
    for c in chars {
        let count = c_map.entry(c).or_insert(0);
        *count += 1;
    }
    c_map.values().all(|x| *x == 1)
}

fn is_four_of_kind(hand: &str) -> bool {
    is_n_of_kind(hand, 4)
}

fn is_three_of_kind(hand: &str) -> bool {
    is_n_of_kind(hand, 3)
}

fn is_pair(hand: &str) -> bool {
    is_n_of_kind(hand, 2)
}

fn parse_hand(line: &str) -> Result<Game, Error> {
    let mut parts = line.split_whitespace();
    let hand = parts.next().unwrap().to_string();
    let bid = parts.next().unwrap().parse::<usize>();
    if bid.is_err() {
        return Err(anyhow::anyhow!("Invalid bid {}", bid.unwrap_err()));
    }
    let bid = bid.unwrap();
    if !is_hand(&hand) {
        Err(anyhow::anyhow!("Invalid hand: {}", hand))
    } else {
        Ok(Game { hand, bid })
    }
}

fn parse_game(lines: &[String]) -> Vec<Game> {
    lines
        .iter()
        .map(|l| parse_hand(l).unwrap())
        .collect::<Vec<Game>>()
}

fn rank_game(hand: &[Game]) -> Vec<Game> {
    let mut ranked = hand.to_owned();
    ranked.sort();
    ranked
}

fn total_winnings(hand: &[Game]) -> usize {
    let mut total = 0;
    for (i, game) in hand.iter().enumerate() {
        total += game.bid * (i + 1);
    }
    total
}

fn main() -> Result<(), Error> {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();
    let lines = io::stdin()
        .lines()
        .map(|l| l.unwrap().trim().to_string())
        .collect::<Vec<String>>();
    let game = parse_game(&lines);
    let game = rank_game(&game);
    let total_winnings = total_winnings(&game);
    println!("{}", total_winnings);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_hand() {
        let game = parse_hand("55533 24").unwrap();
        assert_eq!(game.hand, "55533");
        assert_eq!(game.bid, 24);
    }

    #[test]
    fn test_parse_hand_invalid() {
        let game = parse_hand("55533 24");
        assert!(game.is_ok());
        let game = parse_hand("5553 24");
        assert!(game.is_err());
        let game = parse_hand("5553P 2");
        assert!(game.is_err());
        let game = parse_hand("55533 2X");
        assert!(game.is_err());
    }

    #[test]
    fn test_parse_game() {
        let lines = vec!["5432Q 1", "A2345 2"];
        let lines = lines.iter().map(|l| l.to_string()).collect::<Vec<String>>();
        let game = parse_game(&lines);
        assert_eq!(game.len(), 2);
        assert_eq!(game[0].hand, "5432Q");
        assert_eq!(game[0].bid, 1);
        assert_eq!(game[1].hand, "A2345");
        assert_eq!(game[1].bid, 2);
    }

    #[test]
    fn test_rank_game1() {
        let lines = vec!["5432Q 1", "A2345 2"];
        let lines = lines.iter().map(|l| l.to_string()).collect::<Vec<String>>();
        let game = parse_game(&lines);
        let ranked = rank_game(&game);
        assert_eq!(ranked.len(), 2);
        assert_eq!(ranked[1].hand, "A2345");
        assert_eq!(ranked[1].bid, 2);
        assert_eq!(ranked[0].hand, "5432Q");
        assert_eq!(ranked[0].bid, 1);
    }

    #[test]
    fn test_rank_game2() {
        let lines = vec![
            "22222 1", "AAAA3 1", "33322 6", "QQQKT 7", "KKQJT 6", "AKQJT 6",
        ];
        let lines = lines.iter().map(|l| l.to_string()).collect::<Vec<String>>();
        let game = parse_game(&lines);
        let ranked = rank_game(&game);
        assert_eq!(ranked.len(), 6);
        assert_eq!(ranked[0].hand, "AKQJT");
        assert_eq!(ranked[0].bid, 6);
        assert_eq!(ranked[1].hand, "KKQJT");
        assert_eq!(ranked[1].bid, 6);
        assert_eq!(ranked[2].hand, "QQQKT");
        assert_eq!(ranked[2].bid, 7);
        assert_eq!(ranked[3].hand, "33322");
        assert_eq!(ranked[3].bid, 6);
        assert_eq!(ranked[4].hand, "AAAA3");
        assert_eq!(ranked[4].bid, 1);
        assert_eq!(ranked[5].hand, "22222");
        assert_eq!(ranked[5].bid, 1);
    }

    #[test]
    fn test_rank_game3() {
        let lines = vec!["22222 1", "AAAA3 2"];
        let lines = lines.iter().map(|l| l.to_string()).collect::<Vec<String>>();
        let game = parse_game(&lines);
        let ranked = rank_game(&game);
        assert_eq!(ranked.len(), 2);
        assert_eq!(ranked[0].hand, "AAAA3");
        assert_eq!(ranked[0].bid, 2);
        assert_eq!(ranked[1].hand, "22222");
        assert_eq!(ranked[1].bid, 1);
    }

    #[test]
    fn test_is_five_of_kind2() {
        assert!(is_five_of_kind("22222"));
        assert!(!is_five_of_kind("AAAA3"));
    }

    #[test]
    fn test_game_cmp() {
        let game1 = Game {
            hand: "55555".to_string(),
            bid: 1,
        };
        let game2 = Game {
            hand: "55554".to_string(),
            bid: 1,
        };
        let game3 = Game {
            hand: "22222".to_string(),
            bid: 2,
        };
        let game4 = Game {
            hand: "AAAA3".to_string(),
            bid: 1,
        };
        assert_eq!(game1.cmp(&game2), std::cmp::Ordering::Greater);
        assert_eq!(game2.cmp(&game1), std::cmp::Ordering::Less);
        assert_eq!(game1.cmp(&game1), std::cmp::Ordering::Equal);
        assert_eq!(game1.cmp(&game3), std::cmp::Ordering::Greater);
        assert_eq!(game3.cmp(&game4), std::cmp::Ordering::Greater);
    }

    #[test]
    fn test_rank_cmp_equal() {
        let lines = vec!["77772 1", "QAAAA 2"];
        let lines = lines.iter().map(|l| l.to_string()).collect::<Vec<String>>();
        let game = parse_game(&lines);
        let ranked = rank_game(&game);
        assert_eq!(ranked.len(), 2);
        assert_eq!(ranked[0].hand, "77772");
        assert_eq!(ranked[0].bid, 1);
        assert_eq!(ranked[1].hand, "QAAAA");
        assert_eq!(ranked[1].bid, 2);
    }

    #[test]
    fn test_is_card() {
        assert!(is_card('2'));
        assert!(is_card('3'));
        assert!(is_card('4'));
        assert!(is_card('5'));
        assert!(is_card('6'));
        assert!(is_card('7'));
        assert!(is_card('8'));
        assert!(is_card('9'));
        assert!(is_card('T'));
        assert!(is_card('J'));
        assert!(is_card('Q'));
        assert!(is_card('K'));
        assert!(is_card('A'));
        assert!(!is_card('X'));
    }

    #[test]
    fn test_is_five_of_kind() {
        assert!(is_five_of_kind("55555"));
        assert!(!is_five_of_kind("55554"));
        assert!(!is_five_of_kind("5555T"));
        assert!(is_five_of_kind("AAAAA"));
        assert!(!is_five_of_kind("AAAAK"));
    }

    #[test]
    fn test_is_hand() {
        assert!(is_hand("55555"));
        assert!(is_hand("55554"));
        assert!(!is_hand("5555"));
        assert!(!is_hand("5555X"));
        assert!(is_hand("AAAAA"));
        assert!(!is_hand("AAAA"));
        assert!(!is_hand("XXXXX"));
    }

    #[test]
    fn test_is_four_of_kind() {
        assert!(is_four_of_kind("55554"));
        assert!(!is_four_of_kind("55523"));
        assert!(!is_four_of_kind("555AQ"));
        assert!(!is_four_of_kind("55511"));
        assert!(!is_four_of_kind("AAAAA"));
    }

    #[test]
    fn test_is_three_of_kind() {
        assert!(is_three_of_kind("555T4"));
        assert!(is_three_of_kind("55523"));
        assert!(!is_three_of_kind("55AAQ"));
        assert!(!is_three_of_kind("55K11"));
        assert!(!is_three_of_kind("AAAAT"));
        assert!(!is_three_of_kind("AAAAA"));
    }

    #[test]
    fn test_is_pair() {
        assert!(is_pair("555TT"));
        assert!(is_pair("55T33"));
        assert!(is_pair("55AAQ"));
        assert!(is_pair("55K11"));
        assert!(!is_pair("AAAAT"));
        assert!(!is_pair("AAAAA"));
    }

    #[test]
    fn test_is_n_of_kind() {
        assert!(is_n_of_kind("55554", 4));
        assert!(is_n_of_kind("55523", 3));
        assert!(is_n_of_kind("55TAQ", 2));
        assert!(is_n_of_kind("AAAAA", 5));
    }

    #[test]
    fn test_is_full_house() {
        assert!(is_full_house("555TT"));
        assert!(is_full_house("22333"));
        assert!(!is_full_house("55T33"));
        assert!(is_full_house("55AAA"));
        assert!(!is_full_house("55K11"));
        assert!(!is_full_house("AAAAT"));
        assert!(!is_full_house("AAAAA"));
    }

    #[test]
    fn test_is_two_pair() {
        assert!(!is_two_pair("555TT"));
        assert!(is_two_pair("K2233"));
        assert!(is_two_pair("55T33"));
        assert!(is_two_pair("55AAQ"));
        assert!(is_two_pair("55K11"));
        assert!(!is_two_pair("AAAAT"));
        assert!(!is_two_pair("AAAAA"));
    }

    #[test]
    fn test_is_high_card() {
        assert!(is_high_card("A2345"));
        assert!(is_high_card("6789T"));
        assert!(is_high_card("TJQKA"));
        assert!(!is_high_card("55AAQ"));
        assert!(!is_high_card("55555"));
        assert!(!is_high_card("AAAAT"));
        assert!(!is_high_card("AAAAA"));
    }

    #[test]
    fn test_example1() {
        let lines = vec![
            "32T3K 765",
            "T55J5 684",
            "KK677 28",
            "KTJJT 220",
            "QQQJA 483",
        ];
        let lines = lines.iter().map(|l| l.to_string()).collect::<Vec<String>>();
        let game = parse_game(&lines);
        let ranked = rank_game(&game);
        let total_winnings = total_winnings(&ranked);
        assert_eq!(total_winnings, 6440);
    }
}
