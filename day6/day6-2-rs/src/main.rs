use std::env;
use std::fs;
use std::str::FromStr;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    let input_string = fs::read_to_string(file_path).unwrap();

    let past_race: RaceRecord = input_string.parse().unwrap();

    dbg!(&past_race);

    let part1_solution: u64 = (1..past_race.race_time)
        .into_iter()
        .map(|i| {
            if i * (past_race.race_time - i) > past_race.distance {
                1
            } else {
                0
            }
        })
        .sum::<u64>();

    dbg!(part1_solution);
}

#[derive(Debug)]
struct RaceRecord {
    race_time: u64,
    distance: u64,
}

impl FromStr for RaceRecord {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let [time, distance_record]: &[u64] = &s
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| {
                line.chars()
                    .filter(|c| c.is_digit(10))
                    .collect::<String>()
                    .parse::<u64>()
                    .unwrap()
            })
            .collect::<Vec<u64>>()[0..2]
        else {
            dbg!(s);
            return Err(());
        };

        return Ok(RaceRecord {
            race_time: *time,
            distance: *distance_record,
        });
    }
}
