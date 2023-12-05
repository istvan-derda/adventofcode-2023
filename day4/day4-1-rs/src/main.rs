use std::collections::HashSet;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let in_file_path = &args[1];

    let input_string = fs::read_to_string(in_file_path).expect("File couldn't be read");

    let puzzle_result = input_string
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

            println!("{}", line);
            println!(
                "{:?} ::: {:?}",
                winning_numbers.iter().collect::<Vec<_>>(),
                my_numbers.iter().collect::<Vec<_>>()
            );

            let my_winning_numbers_count = winning_numbers
                .intersection(&my_numbers)
                .collect::<HashSet<&u32>>()
                .len();

            println!("{my_winning_numbers_count:?}");

            if my_winning_numbers_count > 0 {
                return u32::pow(2, (my_winning_numbers_count - 1).try_into().unwrap());
            } else {
                return 0;
            }
        })
        .sum::<u32>();

    println!("{puzzle_result:?}");
}
