use std::io::{self, Error};

fn starts_with_number_word(s: &String) -> bool {
    if s.starts_with("zero")
        || s.starts_with("one")
        || s.starts_with("two")
        || s.starts_with("three")
        || s.starts_with("four")
        || s.starts_with("five")
        || s.starts_with("six")
        || s.starts_with("seven")
        || s.starts_with("eight")
        || s.starts_with("nine")
    {
        true
    } else {
        false
    }
}

fn findfirstnumberorword(s: &String) -> Option<usize> {
    for (i, c) in s.char_indices() {
        if char::is_numeric(c) {
            return Some(i);
        }
        let word = s[i..].to_string();
        if starts_with_number_word(&word.to_string()) {
            return Some(i);
        }
    }
    None
}

fn findlastnumberorword(s: &String) -> Option<usize> {
    for (i, c) in s.char_indices().rev() {
        if char::is_numeric(c) {
            return Some(i);
        }
        let word = s[i..].to_string();
        if starts_with_number_word(&word.to_string()) {
            return Some(i);
        }
    }
    None
}

fn decode(input: &String) -> Result<i32, Error> {
    let firstnumber = input.find(char::is_numeric);
    let lastnumber = input.rfind(char::is_numeric);
    if firstnumber.is_none() || lastnumber.is_none() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("No numbers found: {}", input),
        ));
    }
    let firstnumber = firstnumber.unwrap();
    let lastnumber = lastnumber.unwrap();
    let n1 = input[firstnumber..firstnumber + 1].parse::<i32>().unwrap();
    let n2 = input[lastnumber..lastnumber + 1].parse::<i32>().unwrap();
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
    fn test_find_first_number_word() {
        let s1 = "two1nine".to_string();
        assert!(findfirstnumberorword(&s1).unwrap() == 0);
    }

    #[test]
    fn test_find_first_number_word2() {
        let s1 = "to1nine".to_string();
        assert!(findfirstnumberorword(&s1).unwrap() == 2);
    }

    #[test]
    fn test_find_last_number_word() {
        let s1 = "two1nine".to_string();
        assert!(findlastnumberorword(&s1).unwrap() == 4);
    }

    #[test]
    fn test_find_last_number_word2() {
        let s1 = "two1nne".to_string();
        assert!(findlastnumberorword(&s1).unwrap() == 3);
    }
}
