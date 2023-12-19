use std::io;

use anyhow::Error;
use tracing::Level;

#[derive(Debug, PartialEq)]
struct RaceTable {
    max_time: usize,
    record_distance: usize,
}

fn parse_race_table(lines: &Vec<String>) -> Result<Vec<RaceTable>, Error> {
    let mut max_time = Vec::<usize>::new();
    let mut record_distance = Vec::<usize>::new();

    if lines[0].contains("Time") {
        let time_part = lines[0].split(':').last().unwrap();
        let time_token = time_part.split(' ');
        for time_str in time_token {
            let time_str = time_str.trim();
            if time_str.is_empty() {
                continue;
            }
            let time = time_str.parse::<usize>()?;
            max_time.push(time);
        }
    }

    if lines[1].contains("Distance") {
        let distance_part = lines[1].split(':').last().unwrap();
        let distance_token = distance_part.split(' ');
        for distance_str in distance_token {
            let distance_str = distance_str.trim();
            if distance_str.is_empty() {
                continue;
            }
            let distance = distance_str.parse::<usize>()?;
            record_distance.push(distance);
        }
    }

    if max_time.len() == record_distance.len() {
        let mut race_table = Vec::<RaceTable>::new();
        for i in 0..max_time.len() {
            race_table.push(RaceTable {
                max_time: max_time[i],
                record_distance: record_distance[i],
            });
        }
        Ok(race_table)
    } else {
        Err(anyhow::anyhow!("Input length not matching"))
    }
}

fn race_distance(charge_time: usize, max_time: usize) -> usize {
    assert!(charge_time <= max_time);
    let speed_mmpms = charge_time;
    let remaining_time = max_time - charge_time;
    speed_mmpms * remaining_time
}

fn number_of_wins(max_time: usize, required_distance: usize) -> usize {
    let mut win_count = 0;
    for charge_time in 1..max_time {
        if race_distance(charge_time, max_time) > required_distance {
            tracing::debug!(
                "charge_time: {} - {}",
                charge_time,
                race_distance(charge_time, max_time)
            );
            win_count += 1;
        }
    }
    win_count
}

fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();
    let lines = io::stdin()
        .lines()
        .map(|line| line.unwrap().trim().to_string())
        .collect::<Vec<String>>();
    if let Ok(race_table) = parse_race_table(&lines) {
        let mut n_way_tally = 1;
        for race in race_table.iter() {
            let n_win = number_of_wins(race.max_time, race.record_distance);
            n_way_tally *= n_win;
        }
        println!("{}", n_way_tally);
        Ok(())
    } else {
        Err(anyhow::anyhow!("Invalid input"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_race_distance() {
        assert_eq!(race_distance(0, 7), 0);
        assert_eq!(race_distance(1, 7), 6);
        assert_eq!(race_distance(2, 7), 10);
        assert_eq!(race_distance(3, 7), 12);
        assert_eq!(race_distance(4, 7), 12);
        assert_eq!(race_distance(5, 7), 10);
        assert_eq!(race_distance(6, 7), 6);
        assert_eq!(race_distance(7, 7), 0);
    }

    #[test]
    fn test_number_of_wins() {
        assert_eq!(number_of_wins(7, 9), 4);
        assert_eq!(number_of_wins(15, 40), 8);
        assert_eq!(number_of_wins(30, 200), 9);
    }

    #[test]
    fn test_parse_table() {
        let data = vec![
            "Time:        46     68     98     66\n",
            "Distance:   358   1054   1807   1080",
        ];
        let data = data.iter().map(|s| s.to_string()).collect::<Vec<String>>();
        let race_table = parse_race_table(&data).unwrap();
        assert_eq!(race_table.len(), 4);
        assert_eq!(
            race_table[0],
            RaceTable {
                max_time: 46,
                record_distance: 358
            }
        );
    }
}
