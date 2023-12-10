use std::env;
use std::fs;
use std::str::FromStr;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    let input_string = fs::read_to_string(file_path).unwrap();

    let past_races: RaceRecords = input_string.parse().unwrap();

    dbg!(&past_races);

    let part1_solution: u32 = past_races
        .records
        .iter()
        .map(|record| {
            (1..record.race_time)
                .into_iter()
                .map(|i| {
                    if i * (record.race_time - i) > record.distance {
                        1
                    } else {
                        0
                    }
                })
                .sum::<u32>()
        })
        .product();

    dbg!(part1_solution);
}

#[derive(Debug)]
struct RaceRecords {
    records: Vec<RaceRecord>,
}

#[derive(Debug)]
struct RaceRecord {
    race_time: u32,
    distance: u32,
}

impl FromStr for RaceRecords {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let [times, distance_records]: &[Vec<u32>] = &s
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| {
                line.split_whitespace()
                    .filter_map(|word| word.parse::<u32>().ok())
                    .collect::<Vec<u32>>()
            })
            .collect::<Vec<Vec<u32>>>()[0..2]
        else {
            dbg!(s);
            return Err(());
        };

        let records = std::iter::zip(times, distance_records)
            .map(|(race_time, distance)| RaceRecord {
                race_time: *race_time,
                distance: *distance,
            })
            .collect();

        return Ok(RaceRecords { records });
    }
}
