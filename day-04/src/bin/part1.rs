use anyhow::Error;
use std::collections::HashSet;
use tracing::Level;

fn decode_winner(input: &[String]) -> Result<usize, Error> {
    let mut points = 0;
    for line in input {
        let mut parts = line.split(":").last().unwrap().split("|");
        let winners = parts.next().unwrap().trim().split(" ");
        let mut winner_set = HashSet::new();
        for winner in winners {
            let winner = winner.trim();
            if winner.is_empty() {
                continue;
            }
            let winner = winner.parse::<usize>()?;
            winner_set.insert(winner);
        }
        let cards = parts.next().unwrap().trim().split(" ");
        let mut tally = 0;
        for card_num in cards {
            let card_num = card_num.trim();
            if card_num.is_empty() {
                continue;
            }
            let card_num = card_num.parse::<usize>()?;
            if winner_set.contains(&card_num) {
                if tally == 0 {
                    tally = 1;
                } else {
                    tally *= 2;
                }
            }
        }
        points += tally;
    }

    Ok(points)
}

fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();
    let lines = std::io::stdin().lines();
    let lines = lines
        .map(|l| l.unwrap().trim().to_string())
        .collect::<Vec<String>>();
    let sum = decode_winner(&lines).unwrap();
    println!("{}", sum);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let exa = vec![
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53",
            "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
            "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
            "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
            "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36",
            "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        ];
        let exa = exa.iter().map(|s| s.to_string()).collect::<Vec<String>>();
        let expect_points = 13;
        let result = decode_winner(&exa).unwrap();
        assert_eq!(result, expect_points);
    }
}
