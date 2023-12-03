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

    fn max(&mut self, cube: &CubeGame) {
        self.red = self.red.max(cube.red);
        self.green = self.green.max(cube.green);
        self.blue = self.blue.max(cube.blue);
    }
}

fn sum_minimum_power_games(line: Vec<String>) -> Result<u32, Error> {
    let sum = line
        .iter()
        .map(|line| {
            let mut minimum_cube = CubeGame {
                red: u32::MIN,
                green: u32::MIN,
                blue: u32::MIN,
            };

            for token in line.split(':') {
                tracing::debug!("{}", token);
                if !token.contains("Game") {
                    tracing::debug!("Set {}", token);
                    let set_list = token.split(';').map(|set| set.trim());
                    for set in set_list {
                        tracing::debug!("{}", set);
                        let cube = CubeGame::parse(set).unwrap();
                        minimum_cube.max(&cube);
                        tracing::debug!("{:?}", minimum_cube);
                    }
                }
            }
            minimum_cube.red * minimum_cube.green * minimum_cube.blue
        })
        .sum::<u32>();
    Ok(sum)
}

fn main() -> Result<(), Error> {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();
    let lines = io::stdin()
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();

    let sum = sum_minimum_power_games(lines)?;
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
    fn test_sum_possible_games() {
        let lines = vec![
            String::from("Game 1: 1 red, 2 green, 3 blue; 2 red, 3 green, 4 blue"),
            String::from("Game 2: 2 red, 3 green, 2 blue; 1 red, 2 green, 3 blue"),
        ];
        let sum = sum_minimum_power_games(lines).unwrap();
        assert_eq!(sum, 42);
    }

    #[test]
    fn test_sum_possible_games_example() {
        let lines = vec![
            String::from("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"),
            String::from("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue"),
            String::from(
                "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
            ),
            String::from(
                "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
            ),
            String::from("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"),
        ];
        let sum = sum_minimum_power_games(lines).unwrap();
        assert_eq!(sum, 2286);
    }
}
