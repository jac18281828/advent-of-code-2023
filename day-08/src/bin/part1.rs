use anyhow::Error;
use std::collections::HashMap;
use std::io;

const MAXIMUM_CYCLE: usize = 1000;

struct Graph {
    value: String,
    left: Option<String>,
    right: Option<String>,
}

struct Instruction {
    cycle: Vec<char>,
    route: HashMap<String, Graph>,
}

fn parse_node(line: &str, graph: &mut Graph) -> Result<(), Error> {
    let mut split = line.split('=');
    let start = split.next().unwrap().trim();
    let dest = split.next().unwrap().trim();
    let mut lrsplit = dest.split(',');
    let left = lrsplit.next().unwrap().trim();
    let left = left[1..].to_string();
    let right = lrsplit.next().unwrap().trim();
    let right = right[..right.len() - 1].to_string();
    graph.value = start.to_string();
    graph.left = Some(left);
    graph.right = Some(right);
    Ok(())
}

fn parse_lines(lines: Vec<String>) -> Result<Instruction, Error> {
    let mut instruction = Instruction {
        cycle: vec![],
        route: HashMap::new(),
    };
    instruction.cycle = lines[0].trim().to_string().chars().collect();
    for line in lines[1..].iter() {
        if line.is_empty() {
            continue;
        }
        let mut graph = Graph {
            value: String::from(""),
            left: None,
            right: None,
        };
        if parse_node(line, &mut graph).is_ok() {
            instruction.route.insert(graph.value.clone(), graph);
        } else {
            return Err(anyhow::anyhow!("Failed to parse node:{}", line));
        }
    }
    Ok(instruction)
}

fn count_steps(instruction: Instruction, start: &str, term: &str) -> Result<usize, Error> {
    let mut step_taken = HashMap::new();
    let mut steps: usize = 0;
    let mut direction = instruction.cycle[0];
    let mut current = start.to_string();
    while let Some(graph) = instruction.route.get(&current) {
        steps += 1;
        if direction == 'L' {
            current = graph.left.clone().unwrap();
        } else if direction == 'R' {
            current = graph.right.clone().unwrap();
        } else {
            return Err(anyhow::anyhow!("Invalid direction in cycle"));
        }
        let path = (current.to_string(), direction);
        let entry = step_taken.entry(path.clone()).or_insert(1);
        if *entry > MAXIMUM_CYCLE {
            return Err(anyhow::anyhow!("maximum cycle detected"));
        }
        direction = instruction.cycle[steps % instruction.cycle.len()];
        if current == term {
            break;
        }
    }
    Ok(steps)
}

fn main() -> Result<(), Error> {
    let lines = io::stdin()
        .lines()
        .map(|line| line.unwrap().to_string())
        .collect::<Vec<String>>();
    if let Ok(instruction) = parse_lines(lines) {
        let steps = count_steps(instruction, "AAA", "ZZZ");
        match steps {
            Ok(steps) => println!("Steps: {}", steps),
            Err(e) => println!("Error: {}", e),
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_node() {
        let mut graph = Graph {
            value: String::from(""),
            left: None,
            right: None,
        };
        let line = "AAA = (BBB, CCC)";
        assert!(parse_node(line, &mut graph).is_ok());
        assert_eq!(graph.value, "AAA");
        assert_eq!(graph.left, Some("BBB".to_string()));
        assert_eq!(graph.right, Some("CCC".to_string()));
    }

    #[test]
    fn test_parse_lines() {
        let lines = vec![
            "RL",
            "",
            "AAA = (BBB, CCC)",
            "BBB = (DDD, EEE)",
            "CCC = (ZZZ, GGG)",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
        let instruction = parse_lines(lines).unwrap();
        assert_eq!(instruction.cycle, vec!['R', 'L']);
        assert_eq!(instruction.route.len(), 3);
        assert_eq!(instruction.route["AAA"].value, "AAA");
        assert_eq!(instruction.route["AAA"].left, Some("BBB".to_string()));
        assert_eq!(instruction.route["AAA"].right, Some("CCC".to_string()));
        assert_eq!(instruction.route["BBB"].value, "BBB");
        assert_eq!(instruction.route["BBB"].left, Some("DDD".to_string()));
        assert_eq!(instruction.route["BBB"].right, Some("EEE".to_string()));
        assert_eq!(instruction.route["CCC"].value, "CCC");
        assert_eq!(instruction.route["CCC"].left, Some("ZZZ".to_string()));
        assert_eq!(instruction.route["CCC"].right, Some("GGG".to_string()));
    }

    #[test]
    fn test_count_steps_example1() {
        let lines = vec![
            "RL",
            "",
            "AAA = (BBB, CCC)",
            "BBB = (DDD, EEE)",
            "CCC = (ZZZ, GGG)",
            "DDD = (DDD, DDD)",
            "EEE = (EEE, EEE)",
            "GGG = (GGG, GGG)",
            "ZZZ = (ZZZ, ZZZ)",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
        let instruction = parse_lines(lines).unwrap();
        assert_eq!(count_steps(instruction, "AAA", "ZZZ").unwrap(), 2);
    }

    #[test]
    fn test_count_steps_example2() {
        let lines = vec![
            "LLR",
            "",
            "AAA = (BBB, BBB)",
            "BBB = (AAA, ZZZ)",
            "ZZZ = (ZZZ, ZZZ)",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
        let instruction = parse_lines(lines).unwrap();
        assert_eq!(count_steps(instruction, "AAA", "ZZZ").unwrap(), 6);
    }
}
