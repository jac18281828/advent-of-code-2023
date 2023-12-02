use anyhow::Error;
use std::io;
use tracing::Level;

#[derive(Debug)]
struct CubeGame {
    red: u32,
    green: u32,
    blue: u32,
}

impl CubeGame {
    fn parse(input: &str) -> Result<CubeGame, Error> {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        let mut value = 0;
        for colorpart in input.split(',').map(|part| part.trim()) {
            for part in colorpart.split(char::is_whitespace) {
                tracing::debug!("{}", part);
                if value == 0 {
                    value = part.parse::<u32>()?;
                } else {
                    match part.trim() {
                        "red" => red = value,
                        "green" => green = value,
                        "blue" => blue = value,
                        _ => return Err(anyhow::anyhow!("Invalid color")),
                    }
                    value = 0;
                }
            }
        }
        Ok(CubeGame { red, green, blue })
    }

    fn is_possible(&self, cube: &CubeGame) -> bool {
        self.red <= cube.red && self.green <= cube.green && self.blue <= cube.blue
    }
}

fn sum_possible_games(line: Vec<String>, game: &CubeGame) -> Result<u32, Error> {
    let sum = line.iter()
        .map(|line| {
            let mut game_id = 0;
            let mut is_possible = true;

            for token in line.split(':') {
                tracing::debug!("{}", token);
                if token.contains("Game") {
                    game_id = token
                        .split(char::is_whitespace)
                        .nth(1)
                        .unwrap()
                        .parse::<u32>()
                        .unwrap();
                    tracing::debug!("Game {}", game_id);
                } else {
                    tracing::debug!("Set {}", token);
                    let set_list = token.split(';').map(|set| set.trim());
                    for set in set_list {
                        tracing::debug!("{}", set);
                        let cube = CubeGame::parse(set).unwrap();
                        tracing::debug!("{:?}", cube);
                        if !cube.is_possible(&game) {
                            tracing::info!("Game {} is not possible", game_id);
                            is_possible = false;
                            break;
                        }
                    }
                }
            }

            if is_possible {
                tracing::info!("Game {} is possible", game_id);
                game_id
            } else {
                0
            }
        })
        .sum::<u32>();
    Ok(sum as u32)
}

fn main() -> Result<(), Error> {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();
    let game = CubeGame {
        red: 12,
        green: 13,
        blue: 14,
    };

    let lines = io::stdin()
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();

    let sum = sum_possible_games(lines, &game)?;
    println!("{}", sum);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let game = CubeGame::parse("1 red, 2 green, 3 blue").unwrap();
        assert_eq!(game.red, 1);
        assert_eq!(game.green, 2);
        assert_eq!(game.blue, 3);
    }

    #[test]
    fn test_is_possible() {
        let game = CubeGame {
            red: 1,
            green: 2,
            blue: 3,
        };
        let cube = CubeGame {
            red: 2,
            green: 3,
            blue: 4,
        };
        assert!(game.is_possible(&cube));
    }

    #[test]
    fn test_is_not_possible() {
        let game = CubeGame {
            red: 1,
            green: 2,
            blue: 3,
        };
        let cube = CubeGame {
            red: 2,
            green: 3,
            blue: 2,
        };
        assert!(!game.is_possible(&cube));
    }

    #[test]
    fn test_sum_possible_games() {
        let game = CubeGame {
            red: 1,
            green: 2,
            blue: 3,
        };
        let lines = vec![
            String::from("Game 1: 1 red, 2 green, 3 blue; 2 red, 3 green, 4 blue"),
            String::from("Game 2: 2 red, 3 green, 2 blue; 1 red, 2 green, 3 blue"),
        ];
        let sum = sum_possible_games(lines, &game).unwrap();
        assert_eq!(sum, 3);
    }

    #[test]
    fn test_sum_possible_games_example() {
        let game = CubeGame {
            red: 1,
            green: 2,
            blue: 3,
        };
        let lines = vec![
            String::from("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"),
            String::from("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue"),
            String::from("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"),
            String::from("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red"),
            String::from("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"),
        ];
        let sum = sum_possible_games(lines, &game).unwrap();
        assert_eq!(sum, 8);
    }
}
