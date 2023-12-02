use std::io::{self, Error};
use tracing;

fn decode(input: &String) -> Result<i32, Error> {
    let mut number_tokens = vec![];
    let mut i = 0;
    tracing::debug!("{}", input);
    while i < input.len() {
        if char::is_numeric(input.chars().nth(i).unwrap()) {
            let n = input[i..i + 1].parse::<i32>().unwrap();
            number_tokens.push(n);
        } else if input[i..].starts_with("zero") {
            number_tokens.push(0);
        } else if input[i..].starts_with("one") {
            number_tokens.push(1);
        } else if input[i..].starts_with("two") {
            number_tokens.push(2);
        } else if input[i..].starts_with("three") {
            number_tokens.push(3);
        } else if input[i..].starts_with("four") {
            number_tokens.push(4);
        } else if input[i..].starts_with("five") {
            number_tokens.push(5);
        } else if input[i..].starts_with("six") {
            number_tokens.push(6);
        } else if input[i..].starts_with("seven") {
            number_tokens.push(7);
        } else if input[i..].starts_with("eight") {
            number_tokens.push(8);
        } else if input[i..].starts_with("nine") {
            number_tokens.push(9);
        }
        i += 1;
    }

    if number_tokens.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("No numbers found: {}", input),
        ));
    }
    tracing::debug!("{:?}", number_tokens);
    let n1 = number_tokens[0];
    let n2 = number_tokens[number_tokens.len() - 1];
    Ok(n1 * 10 + n2)
}

fn main() -> Result<(), Error> {
    let sum = io::stdin()
        .lines()
        .map(|line| decode(&line.unwrap()).unwrap())
        .sum::<i32>();
    println!("{}", sum);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode() {
        let s1 = "11";
        assert!(decode(&s1.to_string()).unwrap() == 11);
    }

    #[test]
    fn test_decode2() {
        let s1 = "111";
        assert!(decode(&s1.to_string()).unwrap() == 11);
    }

    #[test]
    fn test_decode3() {
        let s1 = "pqr3stu8vwx";
        assert!(decode(&s1.to_string()).unwrap() == 38);
    }

    #[test]
    fn test_decode4() {
        let s1 = "treb7uchet";
        assert!(decode(&s1.to_string()).unwrap() == 77);
    }

    #[test]
    fn test_decode5() {
        let s1 = "trebuchet";
        assert!(decode(&s1.to_string()).is_err());
    }

    #[test]
    fn test_decode6() {
        let s1 = "zero";
        assert!(decode(&s1.to_string()).unwrap() == 0);
    }

    #[test]
    fn test_decode7() {
        let s1 = "one";
        assert!(decode(&s1.to_string()).unwrap() == 11);
    }

    #[test]
    fn test_decode8() {
        let s1 = "two";
        assert!(decode(&s1.to_string()).unwrap() == 22);
    }

    #[test]
    fn test_decode9() {
        let s1 = "three";
        assert!(decode(&s1.to_string()).unwrap() == 33);
    }

    #[test]
    fn test_decode10() {
        let s1 = "four";
        assert!(decode(&s1.to_string()).unwrap() == 44);
    }

    #[test]
    fn test_decode11() {
        let s1 = "five";
        assert!(decode(&s1.to_string()).unwrap() == 55);
    }

    #[test]
    fn test_decode12() {
        let s1 = "six";
        assert!(decode(&s1.to_string()).unwrap() == 66);
    }

    #[test]
    fn test_decode13() {
        let s1 = "seven";
        assert!(decode(&s1.to_string()).unwrap() == 77);
    }

    #[test]
    fn test_decode14() {
        let s1 = "eight";
        assert!(decode(&s1.to_string()).unwrap() == 88);
    }

    #[test]
    fn test_decode15() {
        let s1 = "nine";
        assert!(decode(&s1.to_string()).unwrap() == 99);
    }

    #[test]
    fn test_decode16() {
        let s1 = "zerozero";
        assert!(decode(&s1.to_string()).unwrap() == 0);
    }

    #[test]
    fn test_decode17() {
        let s1 = "onetwo";
        assert!(decode(&s1.to_string()).unwrap() == 12);
    }

    #[test]
    fn test_decode18() {
        let s1 = "twoaone";
        assert!(decode(&s1.to_string()).unwrap() == 21);
    }

    #[test]
    fn test_decode19() {
        let s1 = "athreebfourc";
        assert!(decode(&s1.to_string()).unwrap() == 34);
    }

    #[test]
    fn test_decode20() {
        let s1 = "athreebfourc1";
        assert!(decode(&s1.to_string()).unwrap() == 31);
    }

    #[test]
    fn test_decode21() {
        let words = vec![
            "two1nine",
            "eightwothree",
            "abcone2threexyz",
            "xtwone3four",
            "4nineeightseven2",
            "zoneight234",
            "7pqrstsixteen",
        ];
        let expect = 281;
        let result = words
            .iter()
            .map(|line| line.to_string())
            .map(|line| decode(&line).unwrap())
            .sum::<i32>();
        assert_eq!(result, expect);
    }

    #[test]
    fn test_allowoverlapinwords() {
        let s1 = "ninesevensrzxkzpmgz8kcjxsbdftwoner";
        assert_eq!(decode(&s1.to_string()).unwrap(), 91);
    }
}
