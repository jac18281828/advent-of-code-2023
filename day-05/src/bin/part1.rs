use anyhow::Error;
use lyn::Scanner;
use std::{collections::HashMap, io::Read};
use tracing::Level;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(
        short = 'f',
        long = "input",
        help = "Input file to use, stdin if not present",
        default_value = "-"
    )]
    file: String,
}

#[derive(Debug, PartialEq)]
enum Token {
    Seeds,
    Map { name: String },
    Array { n: Vec<u32> },
    NewLine,
}

struct AlmanacParser {
    scanner: Scanner,
    seeds: Vec<u32>,
    map_list: Vec<String>,
    map_table: HashMap<String, Vec<day_05::RangeMap>>,
}

impl AlmanacParser {
    fn new(input: &str) -> Self {
        Self {
            scanner: Scanner::new(input),
            seeds: Vec::new(),
            map_list: Vec::new(),
            map_table: HashMap::new(),
        }
    }

    fn parse(&mut self) {
        while let Some(token) = self.parse_token() {
            match token {
                Token::Seeds => {
                    let array = self.parse_array();
                    if array.is_none() {
                        tracing::error!("seeds missing");
                        break;
                    }
                    let array_token = array.unwrap();
                    match array_token {
                        Token::Array { n } => {
                            self.seeds = n;
                        }
                        _ => {
                            tracing::error!("seeds missing");
                            break;
                        }
                    }
                }
                Token::Map { name } => {
                    self.map_list.push(name.clone());
                    while let Some(token) = self.parse_array() {
                        match token {
                            Token::Array { n } => {
                                let range_map = day_05::RangeMap::new(
                                    n[1] as usize,
                                    n[0] as usize,
                                    n[2] as usize,
                                );
                                if self.map_table.contains_key(&name) {
                                    self.map_table.get_mut(&name).unwrap().push(range_map);
                                } else {
                                    self.map_table.insert(name.clone(), vec![range_map]);
                                }
                            }
                            Token::NewLine => {
                                if self.map_table.contains_key(&name) {
                                    break;
                                } else {
                                    continue;
                                }
                            }
                            _ => {
                                tracing::error!("Input error");
                                break;
                            }
                        }
                    }
                }
                Token::NewLine => {
                    continue;
                }
                _ => {
                    tracing::error!("unexpected input");
                    break;
                }
            }
        }
    }

    fn parse_token(&mut self) -> Option<Token> {
        let mut name = String::new();
        let mut token = String::new();
        while !self.scanner.is_done() {
            let c = self.scanner.pop();
            if c.is_none() {
                break;
            }
            match c.unwrap() {
                '\n' => {
                    token.push('\n');
                    break;
                }
                ':' => {
                    if token.is_empty() {
                        token = name;
                        name = String::new();
                        continue;
                    } else {
                        break;
                    }
                }
                ' ' => {
                    if !token.is_empty() && name.is_empty() {
                        name = token;
                        token = String::new();
                    }
                    continue;
                }
                '-' => {
                    token.push('-');
                }
                other => {
                    if other.is_alphabetic() {
                        token.push(*other);
                    } else {
                        break;
                    }
                }
            }
        }
        if token == '\n'.to_string() {
            Some(Token::NewLine)
        } else if token == "seeds" {
            Some(Token::Seeds)
        } else if token == "map" {
            Some(Token::Map { name })
        } else {
            None
        }
    }

    fn parse_array(&mut self) -> Option<Token> {
        let mut n = Vec::new();
        let mut nstr = String::new();
        while !self.scanner.is_done() {
            let c = self.scanner.pop();
            if c.is_none() {
                break;
            }
            match c.unwrap() {
                '\r' | '\n' => {
                    if !nstr.is_empty() {
                        n.push(nstr.parse::<u32>().unwrap());
                        nstr = String::new();
                    }
                    break;
                }
                '\t' | ' ' => {
                    if !nstr.is_empty() {
                        n.push(nstr.parse::<u32>().unwrap());
                        nstr = String::new();
                    }
                    continue;
                }
                '[' => {
                    continue;
                }
                ']' => {
                    break;
                }
                other => {
                    if other.is_numeric() {
                        nstr.push(*other);
                    } else {
                        tracing::info!("unexpected character: {}", *other);
                        break;
                    }
                }
            }
        }
        if !nstr.is_empty() {
            n.push(nstr.parse::<u32>().unwrap());
        }
        if n.is_empty() {
            Some(Token::NewLine)
        } else {
            Some(Token::Array { n })
        }
    }
}

fn parse_data(input: &str) -> Result<u32, Error> {
    let mut parser = AlmanacParser::new(input);
    parser.parse();
    let mut least_location = u32::MAX;
    for seed in parser.seeds.iter() {
        let mut last_map = "-seed";
        let mut last_value = *seed;
        for map_name in parser.map_list.iter() {
            tracing::debug!("map_name: {}", map_name);
            let _map_from = map_from(map_name);
            tracing::debug!("{}: {}", _map_from, last_value);
            let _map_to = map_to(map_name);
            if _map_from != map_to(last_map) {
                tracing::error!("expected map for {} found {}", last_map, map_name);
                break;
            }
            let map = parser.map_table.get(map_name).unwrap();
            for range_map in map.iter() {
                if range_map.is_in_range(last_value as usize) {
                    last_value = range_map.map(last_value as usize) as u32;
                    break;
                } else {
                    continue;
                }
            }
            tracing::info!("map_to: {}, {}", _map_to, last_value);
            last_map = map_name;
        }
        if last_map.ends_with("location") && last_value < least_location {
            least_location = last_value;
        }
    }
    if least_location == u32::MAX {
        tracing::error!("no location found");
        return Err(anyhow::anyhow!("no location found"));
    } else {
        println!("least location: {}", least_location);
    }
    Ok(least_location)
}

fn map_from(map_name: &str) -> &str {
    map_name.split('-').next().unwrap()
}

fn map_to(map_name: &str) -> &str {
    map_name.split('-').last().unwrap()
}

fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();
    let matches = Args::parse();
    let mut buf = String::new();
    if matches.file == "-" {
        tracing::info!("reading from stdin");
        let data_read = std::io::stdin().read_to_string(&mut buf)?;
        if data_read > 0 {
            tracing::info!("read {} bytes from stdin", data_read);
        } else {
            tracing::error!("error reading from stdin");
            return Err(anyhow::anyhow!("error reading from stdin"));
        }
    } else {
        tracing::info!("reading from file: {}", matches.file);
        let alminac_data = std::fs::read_to_string(matches.file)?;
        buf.push_str(&alminac_data);
    }
    parse_data(&buf)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_seeds() {
        let mut parser = AlmanacParser::new("seeds :");
        let expect = Some(Token::Seeds);
        let actual = parser.parse_token();
        assert_eq!(actual, expect);
    }

    #[test]
    fn test_parse_soil_to_fertilizer() {
        let mut parser = AlmanacParser::new("soil-to-fertilizer map:");
        let expect = Some(Token::Map {
            name: "soil-to-fertilizer".to_string(),
        });
        let actual = parser.parse_token();
        assert_eq!(actual, expect);
    }

    #[test]
    fn test_parse_seed_to_soil() {
        let mut parser = AlmanacParser::new(" seed-to-soil map :");
        let expect = Some(Token::Map {
            name: "seed-to-soil".to_string(),
        });
        let actual = parser.parse_token();
        assert_eq!(actual, expect);
    }

    #[test]
    fn test_parse_array() {
        let mut parser = AlmanacParser::new("0 1 2 3 4 5 6 7 8 9");
        let expect = Some(Token::Array {
            n: vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
        });
        let actual = parser.parse_array();
        assert_eq!(actual, expect);
    }

    #[test]
    fn test_parse_seeds_array() {
        let mut parser = AlmanacParser::new(" seeds: 79 14 55 13 ");
        let expect1 = Some(Token::Seeds);
        let actual1 = parser.parse_token();
        assert_eq!(actual1, expect1);
        let expect = Some(Token::Array {
            n: vec![79, 14, 55, 13],
        });
        let actual = parser.parse_array();
        assert_eq!(actual, expect);
    }

    #[test]
    fn test_parse_seed_to_soil_array() {
        let mut parser = AlmanacParser::new(" seed-to-soil map :\n 0 1 2 3 4 5 6 7 8 9 ");
        let expect1 = Some(Token::Map {
            name: "seed-to-soil".to_string(),
        });
        let actual1 = parser.parse_token();
        assert_eq!(actual1, expect1);
        let expect2 = Some(Token::NewLine);
        let actual2 = parser.parse_token();
        assert_eq!(actual2, expect2);
        let expect = Some(Token::Array {
            n: vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
        });
        let actual = parser.parse_array();
        assert_eq!(actual, expect);
    }

    #[test]
    fn test_map_to() {
        let map_name = "seed-to-soil".to_string();
        let expect = "soil";
        let actual = map_to(&map_name);
        assert_eq!(actual, expect);
    }

    #[test]
    fn test_map_to_1() {
        let map_name = "-soil".to_string();
        let expect = "soil";
        let actual = map_to(&map_name);
        assert_eq!(actual, expect);
    }

    #[test]
    fn test_map_from() {
        let map_name = "seed-to-soil".to_string();
        let expect = "seed";
        let actual = map_from(&map_name);
        assert_eq!(actual, expect);
    }

    #[test]
    fn test_map_from_1() {
        let map_name = "seed".to_string();
        let expect = "seed";
        let actual = map_from(&map_name);
        assert_eq!(actual, expect);
    }

    #[test]
    fn test_example() {
        let example = vec![
            "seeds: 79 14 55 13\n",
            "\n",
            "seed-to-soil map:\n",
            "50 98 2\n",
            "52 50 48\n",
            "\n",
            "soil-to-fertilizer map:\n",
            "0 15 37\n",
            "37 52 2\n",
            "39 0 15\n",
            "\n",
            "fertilizer-to-water map:\n",
            "49 53 8\n",
            "0 11 4\n",
            "42 0 7\n",
            "57 7 4\n",
            "\n",
        ];
        let inputstr = example.iter().map(|s| s.to_string()).collect::<String>();
        let mut parser = AlmanacParser::new(inputstr.as_str());
        parser.parse();
        assert_eq!(parser.seeds, vec![79, 14, 55, 13]);
        assert_eq!(
            parser.map_list,
            vec!["seed-to-soil", "soil-to-fertilizer", "fertilizer-to-water"]
        );
        assert_eq!(parser.map_table.len(), 3);
        assert_eq!(parser.map_table.get("seed-to-soil").unwrap().len(), 2);
        assert_eq!(parser.map_table.get("soil-to-fertilizer").unwrap().len(), 3);
        assert_eq!(
            parser.map_table.get("fertilizer-to-water").unwrap().len(),
            4
        );
    }
}
