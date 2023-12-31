use anyhow::Error;
use std::io;
use tracing::Level;

#[derive(Debug, PartialEq)]
pub struct PartNumber {
    number: u32,
    width: u32,
    row: u32,
    column: u32,
}

fn parse_part_number(character_map: &[String]) -> Result<Vec<PartNumber>, Error> {
    let mut result = Vec::new();
    let mut parse_number = false;
    let mut start = 0;
    let mut end: usize;

    for (n, line) in character_map.iter().enumerate() {
        for (col, c) in line.chars().enumerate() {
            if c.is_ascii_digit() {
                if !parse_number {
                    start = col;
                    parse_number = true;
                }
            } else if parse_number {
                end = col;
                let partnum = line[start..end].parse::<u32>()?;
                result.push(PartNumber {
                    number: partnum,
                    width: end as u32 - start as u32,
                    row: n as u32,
                    column: start as u32,
                });
                parse_number = false;
            }
        }
        if parse_number {
            end = line.len();
            let partnum = line[start..end].parse::<u32>()?;
            result.push(PartNumber {
                number: partnum,
                width: end as u32 - start as u32,
                row: n as u32,
                column: start as u32,
            });
            parse_number = false;
        }
    }
    Ok(result)
}

fn is_symbol(c: char) -> bool {
    c != '.' && !c.is_ascii_digit()
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
    let endrow = if part_number.row + 2 > character_map.len() as u32 {
        character_map.len() as u32
    } else {
        part_number.row + 2
    };
    let endcol = if part_number.column + part_number.width + 1 >= character_map[0].len() as u32 {
        character_map[0].len() as u32
    } else {
        part_number.column + part_number.width + 1
    };
    while row < endrow {
        while col < endcol {
            if is_symbol(
                character_map[row as usize]
                    .chars()
                    .nth(col as usize)
                    .unwrap(),
            ) {
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
            tracing::debug!("part number: {:?}", n);
            if is_next_to_symbol(character_map, n) {
                tracing::debug!("part number: {}", n.number);
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
            column: 1,
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
            column: 1,
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
            column: 1,
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
            column: 1,
        };
        assert!(is_next_to_symbol(&s1, &p1));
    }

    #[test]
    fn test_is_next_to_symbol_loleft() {
        let s1 = vec![".467.114..".to_string(), "*.........".to_string()];
        let p1 = PartNumber {
            number: 467,
            width: 3,
            row: 0,
            column: 1,
        };
        assert!(is_next_to_symbol(&s1, &p1));
    }

    #[test]
    fn test_is_next_to_symbol_loright() {
        let s1 = vec![".467.114..".to_string(), "....*.....".to_string()];
        let p1 = PartNumber {
            number: 467,
            width: 3,
            row: 0,
            column: 1,
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
    fn test_is_next_to_symbol_not_included() {
        let s1 = vec!["......29..".to_string(), ".........*".to_string()];
        let p1 = PartNumber {
            number: 29,
            width: 2,
            row: 0,
            column: 6,
        };
        assert!(!is_next_to_symbol(&s1, &p1));
    }

    #[test]
    fn test_is_next_to_symbol_not_included2() {
        let s1 = vec![
            String::from("..........................*..889*....89............675..........%.......29..427...................508..&........&...641..................455"),
            String::from("..........897...960......403.....971...*......806.....@.363................*......9+..............*.....464...................586....282*..."),
        ];
        let p1 = PartNumber {
            number: 29,
            width: 2,
            row: 0,
            column: 72,
        };
        assert!(!is_next_to_symbol(&s1, &p1));
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
            "467..114..",
            "...*......",
            "..35..633.",
            "......#...",
            "617*......",
            ".....+.58.",
            "..592.....",
            "......755.",
            "...$.*....",
            ".664.598..",
        ];
        let exa = exa.iter().map(|s| s.to_string()).collect::<Vec<String>>();
        let expect = 4361;
        assert_eq!(sum_partnumber(&exa).unwrap(), expect);
    }

    #[test]
    fn test_sum_possible_last_char_num() {
        let exa = vec![
            "..........................*..889*....89............675..........%.......29..427...................508..&........&...641..................455",
            "..........897...960......403.....971...*......806.....@.363................*......9+..............*.....464...................586....282*...",
        ];
        let exa = exa.iter().map(|s| s.to_string()).collect::<Vec<String>>();
        let expect = 5172;
        assert_eq!(sum_partnumber(&exa).unwrap(), expect);
    }

    #[test]
    fn test_sum_possible_overcount() {
        let exa = vec![
            ".....984...+......&..618.39.493.289..21....*....379.600...........16.642..162....256........................................*....403........",
            "...............168........*........*...*....326...............*...............*...+..............413.*.....+293.769*620....674..............",
            "647.................949..........502...748..............692...208.......271..903..................=..132.........................506$..832..",
        ];
        let exa = exa.iter().map(|s| s.to_string()).collect::<Vec<String>>();
        let expect = 6867;
        assert_eq!(sum_partnumber(&exa).unwrap(), expect);
    }

    #[test]
    fn test_addl_ex1() {
        let exa = vec![
            "12.......*..",
            "+.........34",
            ".......-12..",
            "..78........",
            "..*....60...",
            "78..........",
            ".......23...",
            "....90*12...",
            "............",
            "2.2......12.",
            ".*.........*",
            "1.1.......56",
        ];
        let exa = exa.iter().map(|s| s.to_string()).collect::<Vec<String>>();
        let expect = 413;
        assert_eq!(sum_partnumber(&exa).unwrap(), expect);
    }

    #[test]
    fn test_addl_ex2() {
        let exa = vec![
            "12.......*..",
            "+.........34",
            ".......-12..",
            "..78........",
            "..*....60...",
            "78.........9",
            ".5.....23..$",
            "8...90*12...",
            "............",
            "2.2......12.",
            ".*.........*",
            "1.1..503+.56",
        ];
        let exa = exa.iter().map(|s| s.to_string()).collect::<Vec<String>>();
        let expect = 925;
        assert_eq!(sum_partnumber(&exa).unwrap(), expect);
    }

    #[test]
    fn test_add2() {
        let exa = vec![".*1", "1.."];
        let exa = exa.iter().map(|s| s.to_string()).collect::<Vec<String>>();
        let expect = 2;
        assert_eq!(sum_partnumber(&exa).unwrap(), expect);
    }
}
