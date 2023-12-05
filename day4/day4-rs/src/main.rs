use std::collections::HashSet;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let in_file_path = &args[1];

    let input_string = fs::read_to_string(in_file_path).expect("File couldn't be read");

    let winning_number_counts: Vec<u32> = input_string
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let [ref winning_numbers, ref my_numbers] = line
                .chars()
                .skip_while(|c| *c != ':')
                .skip(1)
                .collect::<String>()
                .split_terminator('|')
                .map(|numbers_str| {
                    numbers_str
                        .split_whitespace()
                        .map(|number_str| {
                            number_str
                                .trim()
                                .parse::<u32>()
                                .expect(&format!("Failed to parse {number_str:?} to u32.")[..])
                        })
                        .collect::<HashSet<u32>>()
                })
                .collect::<Vec<HashSet<u32>>>()[..]
            else {
                panic!()
            };

            winning_numbers
                .intersection(&my_numbers)
                .collect::<HashSet<&u32>>()
                .len() as u32
        })
        .collect();

    let puzzle1_result: u32 = winning_number_counts
        .clone()
        .into_iter()
        .map(|winning_number_count| {
            if winning_number_count == 0 {
                0
            } else {
                u32::pow(2, winning_number_count - 1)
            }
        })
        .sum();

    println!("{puzzle1_result:?}");

    let puzzle2_result = (0..winning_number_counts.len())
        .map(|i| count_cards_won(&winning_number_counts[i..]))
        .sum::<usize>();

    println!("{puzzle2_result}");
}

fn count_cards_won(winning_number_counts: &[u32]) -> usize {
    let current = winning_number_counts[0] as usize;

    match current {
        0 => 1,
        _ => {
            1 + (1..=current)
                .map(|i| count_cards_won(&winning_number_counts[i..]))
                .sum::<usize>()
        }
    }
}
