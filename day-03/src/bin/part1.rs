use anyhow::Error;
use std::io;
use tracing::Level;

#[derive(Debug, PartialEq)]
struct PartNumber {
    number: u32,
    width: u32,
    row: u32,
    column: u32,
}

fn parse_part_number(character_map: &Vec<String>) -> Result<Vec<PartNumber>, Error> {
    let mut result = Vec::new();
    let mut parse_number = false;
    let mut start = 0;
    let mut end: usize;

    for (n, line) in character_map.iter().enumerate() {
        for (col, c) in line.chars().enumerate() {
            if c.is_digit(10) {
                if !parse_number {
                    start = col;
                    parse_number = true;
                }
            } else {
                if parse_number {
                    end = col;
                    let number = line[start..end].parse::<u32>()?;
                    result.push(PartNumber {
                        number: number,
                        width: end as u32 - start as u32,
                        row: n as u32,
                        column: start as u32,
                    });
                    parse_number = false;
                }
            }
        }
    }
    Ok(result)
}

fn is_symbol(c: char) -> bool {
    c == '*' || c == '#' || c == '$' || c == '+'
}

fn is_next_to_symbol(character_map: &Vec<String>, part_number: &PartNumber) -> bool {
    let mut result = false;
    let mut row = if part_number.row == 0 {
        0
    } else {
        part_number.row - 1
    };
    let mut col = if part_number.column == 0 {
        0
    } else {
        part_number.column - 1
    };
    let endrow = if part_number.row + 3 > character_map.len() as u32 {
        character_map.len() as u32
    } else {
        part_number.row + 3
    };
    let endcol = if part_number.column + part_number.width + 1 >= character_map[0].len() as u32 - 1
    {
        character_map[0].len() as u32 - 1
    } else {
        part_number.column + part_number.width + 1
    };
    while row < endrow {
        while col <= endcol {
            if is_symbol(character_map[row as usize]
                .chars()
                .nth(col as usize)
                .unwrap())
            {
                result = true;
                break;
            }
            col += 1;
        }
        if result {
            break;
        }
        col = if part_number.column == 0 {
            0
        } else {
            part_number.column - 1
        };
        row += 1;
    }
    result
}

fn sum_partnumber(character_map: &Vec<String>) -> Result<u32, Error> {
    let sum = parse_part_number(character_map)?
        .iter()
        .map(|n| {
            if is_next_to_symbol(character_map, n) {
                n.number
            } else {
                0
            }
        })
        .sum::<u32>();
    Ok(sum)
}

fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();
    let character_map: Vec<String> = io::stdin()
        .lines()
        .map(|line| line.unwrap().trim().to_string())
        .collect::<Vec<String>>();
    let sum = sum_partnumber(&character_map)?;
    println!("{}", sum);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_next_to_symbol() {
        let s1 = vec![
            "467..114..".to_string(),
            "...*......".to_string(),
            "..35..633.".to_string(),
        ];
        let p1 = PartNumber {
            number: 467,
            width: 3,
            row: 0,
            column: 0,
        };
        let p2 = PartNumber {
            number: 114,
            width: 3,
            row: 0,
            column: 5,
        };
        let p3 = PartNumber {
            number: 35,
            width: 2,
            row: 2,
            column: 2,
        };
        let p4 = PartNumber {
            number: 633,
            width: 3,
            row: 2,
            column: 6,
        };
        assert!(is_next_to_symbol(&s1, &p1));
        assert!(!is_next_to_symbol(&s1, &p2));
        assert!(is_next_to_symbol(&s1, &p3));
        assert!(!is_next_to_symbol(&s1, &p4));
    }

    #[test]
    fn test_is_next_to_symbol_upleft() {
        let s1 = vec!["*.........".to_string(), ".467.114..".to_string()];
        let p1 = PartNumber {
            number: 467,
            width: 3,
            row: 1,
            column: 0,
        };
        assert!(is_next_to_symbol(&s1, &p1));
    }

    #[test]
    fn test_is_next_to_symbol_upright() {
        let s1 = vec!["....*.....".to_string(), ".467.114..".to_string()];
        let p1 = PartNumber {
            number: 467,
            width: 3,
            row: 1,
            column: 0,
        };
        assert!(is_next_to_symbol(&s1, &p1));
    }

    #[test]
    fn test_is_next_to_symbol_left() {
        let s1 = vec!["..........".to_string(), "*467.114..".to_string()];
        let p1 = PartNumber {
            number: 467,
            width: 3,
            row: 1,
            column: 0,
        };
        assert!(is_next_to_symbol(&s1, &p1));
    }

    #[test]
    fn test_is_next_to_symbol_right() {
        let s1 = vec!["..........".to_string(), ".467*114..".to_string()];
        let p1 = PartNumber {
            number: 467,
            width: 3,
            row: 1,
            column: 0,
        };
        assert!(is_next_to_symbol(&s1, &p1));
    }

    #[test]
    fn test_is_next_to_symbol_loleft() {
        let s1 = vec![".467.114..".to_string(), "*.........".to_string()];
        let p1 = PartNumber {
            number: 467,
            width: 3,
            row: 1,
            column: 0,
        };
        assert!(is_next_to_symbol(&s1, &p1));
    }

    #[test]
    fn test_is_next_to_symbol_loright() {
        let s1 = vec![".467.114..".to_string(), "....*.....".to_string()];
        let p1 = PartNumber {
            number: 467,
            width: 3,
            row: 1,
            column: 0,
        };
        assert!(is_next_to_symbol(&s1, &p1));
    }

    #[test]
    fn test_is_next_bottom_right_corner() {
        let s1 = vec![
            "467..114..".to_string(),
            "...*......".to_string(),
            "..35..633.".to_string(),
            ".........*".to_string(),
        ];
        let pn = PartNumber {
            number: 633,
            width: 3,
            row: 2,
            column: 6,
        };
        assert!(is_next_to_symbol(&s1, &pn));
    }

    #[test]
    fn test_parse_partnumber() {
        let s1 = vec![
            "467..114..".to_string(),
            "...*......".to_string(),
            "..35..633.".to_string(),
        ];
        let expect = vec![
            PartNumber {
                number: 467,
                width: 3,
                row: 0,
                column: 0,
            },
            PartNumber {
                number: 114,
                width: 3,
                row: 0,
                column: 5,
            },
            PartNumber {
                number: 35,
                width: 2,
                row: 2,
                column: 2,
            },
            PartNumber {
                number: 633,
                width: 3,
                row: 2,
                column: 6,
            },
        ];
        assert_eq!(parse_part_number(&s1).unwrap(), expect);
    }

    #[test]
    fn test_example() {
        let exa = vec![
            "467..114..".to_string(),
            "...*......".to_string(),
            "..35..633.".to_string(),
            "......#...".to_string(),
            "617*......".to_string(),
            ".....+.58.".to_string(),
            "..592.....".to_string(),
            "......755.".to_string(),
            "...$.*....".to_string(),
            ".664.598..".to_string(),
        ];
        let expect = 4361;
        assert_eq!(sum_partnumber(&exa).unwrap(), expect);
    }
}
