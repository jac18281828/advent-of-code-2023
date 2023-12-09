use anyhow::{anyhow, Error};
use std::cmp::min;

fn char_at(line: &[char], x: usize, y: usize, width: usize, height: usize) -> char {
    let x = x % width;
    let y = y % height;
    line[y * width + x]
}

fn decode_gear_ratio(
    input: &[char],
    x: usize,
    y: usize,
    width: usize,
    height: usize,
) -> Result<Vec<usize>, anyhow::Error> {
    let mut ratio: Vec<usize> = vec![];
    let mut num = 0;
    let mut power = 1;
    let startx = if x >= 4 { x - 4 } else { 0 };
    let endx = min(x + 4, width);
    if y > 0 {
        let starty = y - 1;
        for x in (startx..endx).rev() {
            let c = char_at(input, x, starty, width, height);
            if c.is_ascii_digit() {
                num += (c as usize - '0' as usize) * power;
                power *= 10;
            } else {
                if power > 1 {
                    ratio.push(num);
                    num = 0;
                    power = 1;
                }
                break;
            }
        }
        if power > 1 {
            ratio.push(num);
            num = 0;
            power = 1;
        }
    }
    for x in (startx..endx).rev() {
        let c = char_at(input, x, y, width, height);
        if c.is_ascii_digit() {
            num += (c as usize - '0' as usize) * power;
            power *= 10;
        } else {
            if power > 1 {
                ratio.push(num);
                num = 0;
                power = 1;
            }
            break;
        }
    }
    if power > 1 {
        ratio.push(num);
        num = 0;
        power = 1;
    }
    if y < height - 1 {
        let starty = y + 1;
        for x in (startx..endx).rev() {
            let c = char_at(input, x, starty, width, height);
            if c.is_ascii_digit() {
                num += (c as usize - '0' as usize) * power;
                power *= 10;
            } else {
                if power > 1 {
                    ratio.push(num);
                    num = 0;
                    power = 1;
                }
                break;
            }
        }
        if power > 1 {
            ratio.push(num);
        }
    }
    Ok(ratio)
}

fn sum_gearratio(input: &[char], width: usize, height: usize) -> Result<usize, anyhow::Error> {
    let mut ratio: Vec<usize> = vec![];
    for y in 0..height {
        for x in 0..width {
            let c = char_at(input, x, y, width, height);
            if c == '*' {
                let gear_ratio = decode_gear_ratio(input, x, y, width, height)?;
                if ratio.len() == 2 {
                    let x = gear_ratio[0];
                    let y = gear_ratio[1];
                    ratio.push(x * y);
                } else if ratio.len() != 1 {
                    return Err(anyhow!("Invalid Gear found"));
                }
            }
        }
    }
    if ratio.is_empty() {
        return Err(anyhow!("No Gear found"));
    }
    Ok(ratio.iter().sum())
}

fn lines_to_vec(lines: &[String]) -> (Vec<char>, usize, usize) {
    let input_vec = lines.iter().flat_map(|s| s.chars()).collect::<Vec<char>>();
    let height = lines.len();
    let width = lines[0].len();
    (input_vec, width, height)
}

fn main() -> Result<(), Error> {
    let input = std::fs::read_to_string("data/input.txt")?;
    let input = input
        .lines()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    let (input_vec, width, height) = lines_to_vec(&input);
    let sum = sum_gearratio(&input_vec, width, height)?;
    println!("{}", sum);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
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
        let (input_vec, width, height) = lines_to_vec(&exa);
        //         let expect = 467835;
        let expect = 3;
        assert_eq!(sum_gearratio(&input_vec, width, height).unwrap(), expect);
    }

    #[test]
    fn test_char_at() {
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
        let (input_vec, width, height) = lines_to_vec(&exa);
        assert_eq!(char_at(&input_vec, 0, 0, width, height), '4');
        assert_eq!(char_at(&input_vec, 9, 0, width, height), '.');
        assert_eq!(char_at(&input_vec, 3, 1, width, height), '*');
        assert_eq!(char_at(&input_vec, 1, 9, width, height), '6');
        assert_eq!(char_at(&input_vec, 9, 9, width, height), '.');
        assert_eq!(char_at(&input_vec, 7, 9, width, height), '8');
    }

    #[test]
    #[ignore]
    fn test_decode_gear_ratio() {
        let exa = vec!["467..114..", "...*......", "..35..633."];
        let exa = exa.iter().map(|s| s.to_string()).collect::<Vec<String>>();
        let (input_vec, width, height) = lines_to_vec(&exa);
        let ratio = decode_gear_ratio(&input_vec, 3, 1, width, height).unwrap();
        assert_eq!(ratio.len(), 2);
        assert_eq!(ratio[0], 467);
        assert_eq!(ratio[1], 35);
    }
}
