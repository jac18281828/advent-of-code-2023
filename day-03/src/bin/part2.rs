use anyhow::{anyhow, Error};

fn char_at(line: &[char], x: usize, y: usize, width: usize, height: usize) -> char {
    let x = x % width;
    let y = y % height;
    line[y * width + x]
}

fn sum_gearratio(_input: &[char], _width: usize, _height: usize) -> Result<usize, anyhow::Error> {
    for y in 0.._height {
        for x in 0.._width {
            let c = char_at(_input, x, y, _width, _height);
            if c == '*' {
                return Ok(x * y);
            }
        }
    }
    Err(anyhow!("No Gear found"))
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
        let expect = 467835;
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
        assert_eq!(char_at(&input_vec, 9, 6, width, height), '8');
    }
}
