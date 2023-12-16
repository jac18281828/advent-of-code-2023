use std::io::Read;

use anyhow::Error;
use clap::Parser;
use tracing::Level;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = "Advent of Code 2023, Day 5")]
struct Args {
    #[arg(
        short = 'f',
        long = "input",
        help = "Input file to use, stdin if not present",
        default_value = "-"
    )]
    file: String,
}

fn parse_data(input: &str) -> Result<u32, Error> {
    let mut parser = day_05::AlmanacParser::new(input);
    parser.parse();
    let mut least_location = u32::MAX;
    for seed in parser.seeds.iter() {
        let mut last_map = "-seed";
        let mut last_value = *seed;
        for map_name in parser.map_list.iter() {
            tracing::debug!("map_name: {}", map_name);
            let map_from = day_05::map_from(map_name);
            tracing::debug!("{}: {}", map_from, last_value);
            let map_to = day_05::map_to(map_name);
            if map_from != day_05::map_to(last_map) {
                tracing::error!("expected map for {} found {}", last_map, map_name);
                break;
            }
            let map = parser.map_table.get(map_name).unwrap();
            for range_map in map.iter() {
                if range_map.is_in_range(last_value as usize) {
                    last_value = range_map.map(last_value as usize) as u32;
                    break;
                } else {
                    continue;
                }
            }
            tracing::debug!("map_to: {}, {}", map_to, last_value);
            last_map = map_name;
        }
        if last_map.ends_with("location") && last_value < least_location {
            least_location = last_value;
        }
    }
    if least_location == u32::MAX {
        tracing::error!("no location found");
        return Err(anyhow::anyhow!("no location found"));
    } else {
        println!("least location: {}", least_location);
    }
    Ok(least_location)
}

fn main() -> Result<(), Error> {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();
    let matches = Args::parse();
    let mut buf = String::new();
    if matches.file == "-" {
        tracing::info!("reading from stdin");
        let data_read = std::io::stdin().read_to_string(&mut buf)?;
        if data_read > 0 {
            tracing::info!("read {} bytes from stdin", data_read);
        } else {
            tracing::error!("error reading from stdin");
            return Err(anyhow::anyhow!("error reading from stdin"));
        }
    } else {
        tracing::info!("reading from file: {}", matches.file);
        let alminac_data = std::fs::read_to_string(matches.file)?;
        buf.push_str(&alminac_data);
    }
    parse_data(&buf)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example() {
        let example = vec![
            "seeds: 79 14 55 13\n",
            "\n",
            "seed-to-soil map:\n",
            "50 98 2\n",
            "52 50 48\n",
            "\n",
            "soil-to-fertilizer map:\n",
            "0 15 37\n",
            "37 52 2\n",
            "39 0 15\n",
            "\n",
            "fertilizer-to-water map:\n",
            "49 53 8\n",
            "0 11 4\n",
            "42 0 7\n",
            "57 7 4\n",
            "\n",
        ];
        let inputstr = example.iter().map(|s| s.to_string()).collect::<String>();
        let mut parser = day_05::AlmanacParser::new(inputstr.as_str());
        parser.parse();
        assert_eq!(parser.seeds, vec![79, 14, 55, 13]);
        assert_eq!(
            parser.map_list,
            vec!["seed-to-soil", "soil-to-fertilizer", "fertilizer-to-water"]
        );
        assert_eq!(parser.map_table.len(), 3);
        assert_eq!(parser.map_table.get("seed-to-soil").unwrap().len(), 2);
        assert_eq!(parser.map_table.get("soil-to-fertilizer").unwrap().len(), 3);
        assert_eq!(
            parser.map_table.get("fertilizer-to-water").unwrap().len(),
            4
        );
    }
}
