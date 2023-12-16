use std::collections::HashMap;

use lyn::Scanner;

#[derive(Debug, PartialEq)]
enum Token {
    Seeds,
    Map { name: String },
    Array { n: Vec<u32> },
    NewLine,
}

pub struct AlmanacParser {
    scanner: Scanner,
    pub seeds: Vec<u32>,
    pub map_list: Vec<String>,
    pub map_table: HashMap<String, Vec<RangeMap>>,
}

impl AlmanacParser {
    pub fn new(input: &str) -> Self {
        Self {
            scanner: Scanner::new(input),
            seeds: Vec::new(),
            map_list: Vec::new(),
            map_table: HashMap::new(),
        }
    }

    pub fn parse(&mut self) {
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
                                let range_map =
                                    RangeMap::new(n[1] as usize, n[0] as usize, n[2] as usize);
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

pub struct RangeMap {
    source_start: usize,
    destination_start: usize,
    range_width: usize,
}

impl RangeMap {
    pub fn new(source_start: usize, destination_start: usize, range_width: usize) -> Self {
        Self {
            source_start,
            destination_start,
            range_width,
        }
    }

    pub fn map(&self, source: usize) -> usize {
        if source < self.source_start || source >= self.source_start + self.range_width {
            source
        } else {
            source - self.source_start + self.destination_start
        }
    }

    pub fn is_in_range(&self, source: usize) -> bool {
        source >= self.source_start && source < self.source_start + self.range_width
    }
}

//
// functions
//

pub fn map_from(map_name: &str) -> &str {
    map_name.split('-').next().unwrap()
}

pub fn map_to(map_name: &str) -> &str {
    map_name.split('-').last().unwrap()
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
    fn test_range_map() {
        let range_map = RangeMap::new(0, 0, 10);
        assert_eq!(range_map.map(0), 0);
        assert_eq!(range_map.map(1), 1);
        assert_eq!(range_map.map(9), 9);
        assert_eq!(range_map.map(10), 10);
        assert_eq!(range_map.map(11), 11);

        let range_map = RangeMap::new(0, 10, 10);
        assert_eq!(range_map.map(0), 10);
        assert_eq!(range_map.map(1), 11);
        assert_eq!(range_map.map(9), 19);
        assert_eq!(range_map.map(10), 10);
        assert_eq!(range_map.map(11), 11);

        let range_map = RangeMap::new(10, 0, 10);
        assert_eq!(range_map.map(0), 0);
        assert_eq!(range_map.map(1), 1);
        assert_eq!(range_map.map(9), 9);
        assert_eq!(range_map.map(10), 0);
        assert_eq!(range_map.map(11), 1);

        let range_map = RangeMap::new(10, 10, 10);
        assert_eq!(range_map.map(0), 0);
        assert_eq!(range_map.map(1), 1);
        assert_eq!(range_map.map(9), 9);
        assert_eq!(range_map.map(10), 10);
        assert_eq!(range_map.map(11), 11);
    }

    #[test]
    fn test_example1() {
        let range_map = RangeMap::new(98, 50, 2);
        assert_eq!(range_map.map(98), 50);
        assert_eq!(range_map.map(99), 51);
        for i in 0..98 {
            assert_eq!(range_map.map(i), i);
        }
        for i in 100..200 {
            assert_eq!(range_map.map(i), i);
        }
    }

    #[test]
    fn test_example2() {
        let range_map = RangeMap::new(50, 52, 48);
        assert_eq!(range_map.map(49), 49);
        assert_eq!(range_map.map(99), 99);
        for i in 50..98 {
            assert_eq!(range_map.map(i), i + 2);
        }
    }

    #[test]
    fn test_is_in_range() {
        let range_map = RangeMap::new(50, 52, 2);
        assert_eq!(range_map.is_in_range(49), false);
        assert_eq!(range_map.is_in_range(50), true);
        assert_eq!(range_map.is_in_range(51), true);
        assert_eq!(range_map.is_in_range(52), false);
        assert_eq!(range_map.is_in_range(53), false);
    }
}
