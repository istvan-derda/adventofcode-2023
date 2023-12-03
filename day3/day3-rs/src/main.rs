use core::ops::Range;
use regex::Regex;
use std::collections::HashMap;
use std::env;
use std::fs;

#[derive(Clone)]
struct PartNumber {
    number: u32,
    part_pos: (usize, usize),
    part_char: char,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let input_str = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let input_char_grid: Vec<Vec<char>> = input_str
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let part_numbers: Vec<PartNumber> = input_char_grid
        .clone()
        .into_iter()
        .enumerate()
        .map(|(current_row, row_chars)| {
            let re = Regex::new(r"\d+").unwrap();
            let row_str = &row_chars.into_iter().collect::<String>()[..];
            let matches = re.find_iter(row_str);
            matches
                .filter_map(|m| {
                    if let Some((x, y, c)) =
                        find_part_around(&input_char_grid, current_row, &m.range())
                    {
                        Some(PartNumber {
                            number: m.as_str().parse().unwrap(),
                            part_pos: (x, y),
                            part_char: c,
                        })
                    } else {
                        None
                    }
                })
                .collect::<Vec<PartNumber>>()
        })
        .flatten()
        .collect();

    let puzzle1_result: u32 = part_numbers
        .clone()
        .into_iter()
        .map(|part_number| part_number.number)
        .sum();

    println!("{puzzle1_result:?}");

    let mut parts: HashMap<(usize, usize), (char, Vec<u32>)> = HashMap::new();
    for part_number in part_numbers {
        parts
            .entry(part_number.part_pos)
            .and_modify(|details| details.1.push(part_number.number.clone()))
            .or_insert((part_number.part_char, vec![part_number.number]));
    }

    let puzzle2_result: u32 = parts
        .iter()
        .map(|(_, details)| details)
        .filter(|(part_char, numbers)| *part_char == '*' && numbers.len() == 2)
        .map(|(_, numbers)| numbers[0] * numbers[1])
        .sum();

    println!("{puzzle2_result:?}");
}

fn find_part_around(
    char_grid: &Vec<Vec<char>>,
    row: usize,
    range: &Range<usize>,
) -> Option<(usize, usize, char)> {
    for x in range.start.checked_sub(1).unwrap_or(range.start)..range.end + 1 {
        for y in row.checked_sub(1).unwrap_or(row)..=row + 1 {
            if let Some(current_row) = char_grid.get(y) {
                if let Some(current_char) = current_row.get(x) {
                    if *current_char != '.' && !current_char.is_digit(10) {
                        return Some((x, y, *current_char));
                    }
                } else {
                    continue;
                }
            } else {
                continue;
            }
        }
    }
    None
}
