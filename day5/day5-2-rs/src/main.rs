use std::collections::HashMap;
use std::env;
use std::fs;
use std::ops::Range;
use std::str::FromStr;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file_path = &args[1];

    let input_string = fs::read_to_string(input_file_path).expect("Couldn't read input file");

    let almanac: Almanac = input_string.parse().unwrap();
    //println!("{almanac:?}");

    let seeds: Vec<u64> = almanac
        .seed_ranges
        .iter()
        .flat_map(|range| range.clone())
        .collect();
    println!("{seeds:?}");

    let seed_to_location_map: HashMap<u64, u64> = seeds
        .iter()
        .map(|seed| (*seed, almanac.forward_lookup(*seed)))
        .collect();
    println!("{seed_to_location_map:?}");

    let min_location = seed_to_location_map.values().min().unwrap();

    println!("{min_location:?}");
}

#[derive(Debug)]
struct Almanac {
    seed_ranges: Vec<Range<u64>>,
    maps: Vec<AlmanacMap>,
}

#[derive(Debug)]
struct AlmanacMap {
    name: String,
    map: Vec<RangeRule>,
}

#[derive(Debug)]
struct RangeRule {
    destination_range_start: u64,
    source_range_start: u64,
    range_length: u64,
}

impl Almanac {
    fn forward_lookup(&self, number: u64) -> u64 {
        self.maps
            .iter()
            .fold(number, |previous_result: u64, map: &AlmanacMap| {
                map.forward_lookup(previous_result)
            })
    }
}

impl AlmanacMap {
    fn forward_lookup(&self, number: u64) -> u64 {
        self.map
            .iter()
            .fold(
                None,
                |previous_result: Option<u64>, range_rule: &RangeRule| match previous_result {
                    Some(_) => {
                        //println!("{previous_result:?}");
                        previous_result
                    }
                    None => {
                        //println!("{range_rule:?}, {number:?}");
                        let s_start = range_rule.source_range_start;
                        let d_start = range_rule.destination_range_start;
                        let s_to_d: i128 = i128::from(d_start) - i128::from(s_start);
                        if s_start <= number && number < s_start + range_rule.range_length {
                            //println!("match!");
                            Some(u64::try_from(i128::from(number) + s_to_d).unwrap())
                        } else {
                            //println!("no match!");
                            None
                        }
                    }
                },
            )
            .unwrap_or(number)
    }
}

impl FromStr for Almanac {
    type Err = String;

    fn from_str(input_string: &str) -> Result<Self, Self::Err> {
        let seed_ranges: Vec<Range<u64>> = input_string
            .lines()
            .nth(0)
            .unwrap()
            .strip_prefix("seeds: ")
            .unwrap()
            .split_whitespace()
            .map(|number_str| number_str.parse::<u64>().unwrap())
            .collect::<Vec<u64>>()
            .windows(2)
            .step_by(2)
            .map(|range_def| range_def[0]..range_def[0] + range_def[1])
            .collect();

        let maps = input_string
            .split_terminator("\n\n")
            .skip(1)
            .filter(|block| !block.is_empty())
            .map(|block| block.parse::<AlmanacMap>().unwrap())
            .collect();

        //println!("{maps:?}");
        Ok(Almanac { seed_ranges, maps })
    }
}

impl FromStr for AlmanacMap {
    type Err = String;

    fn from_str(block_string: &str) -> Result<Self, Self::Err> {
        //println!("parsing block '{block_string:?}'");
        let name = block_string
            .lines()
            .nth(0)
            .ok_or("no firstline")?
            .to_string();

        let map: Vec<RangeRule> = block_string
            .lines()
            .skip(1)
            .map(|line| line.parse::<RangeRule>().unwrap())
            .collect();

        Ok(AlmanacMap { name, map })
    }
}

impl FromStr for RangeRule {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let [destination_range_start, source_range_start, range_length] = line
            .split_whitespace()
            .map(|number_str| number_str.parse::<u64>().unwrap())
            .collect::<Vec<u64>>()[..]
        else {
            panic!()
        };

        Ok(RangeRule {
            destination_range_start,
            source_range_start,
            range_length,
        })
    }
}
