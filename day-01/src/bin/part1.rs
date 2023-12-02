use std::io::{self, Error};

fn decode(input: &String) -> Result<i32, Error> {
    let firstnumber = input.find(char::is_numeric);
    let lastnumber = input.rfind(char::is_numeric);
    if firstnumber.is_none() || lastnumber.is_none() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, format!("No numbers found: {}", input)));
    }
    let firstnumber = firstnumber.unwrap();
    let lastnumber = lastnumber.unwrap();
    let n1 = input[firstnumber..firstnumber+1].parse::<i32>().unwrap();
    let n2 = input[lastnumber..lastnumber+1].parse::<i32>().unwrap();
    Ok(n1 * 10 + n2)
}

fn main() -> Result<(), Error>{
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
}
