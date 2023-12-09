use tracing::Level;
use lyn::Scanner;

struct AlminacParser {
    scanner: Scanner,
}

impl AlminacParser {
    fn new(input: &str) -> Self {
        Self {
            scanner: Scanner::new(input),
        }
    }

    fn parse(&mut self) -> String {
        let mut result = String::new();
        while let Some(token) = self.scanner.next() {
            result.push_str(&token);
        }
        result
    }

    fn parse_keyword(&mut self) -> String {
        let mut result = String::new();
        while let Some(token) = self.scanner.next() {
            if token == " " {
                break;
            }
            result.push_str(&token);
        }
        result
    }
}


fn main() {
    tracing_subscriber::fmt()
    .with_max_level(Level::INFO)
    .init();
    let alminac: String = std::io::stdin().lines().map(|l| l.unwrap()).collect();
    println!("{}", alminac);
}
