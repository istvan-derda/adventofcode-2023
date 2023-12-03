use core::ops::Range;
use regex::Regex;
use std::env;
use std::fs;

struct PartNumber {
    number: u32,
    part_pos: (usize, usize),
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let input_str = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let input_char_grid: Vec<Vec<char>> = input_str
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut parts_coords: Vec<(usize, usize)> = vec![];
    for (y, row) in input_char_grid.clone().into_iter().enumerate() {
        for (x, c) in row.into_iter().enumerate() {
            if c != '.' {
                parts_coords.push((x, y));
            }
        }
    }

    let mut part_numbers = vec![];
    for (y, row) in input_char_grid.clone().into_iter().enumerate() {
        let re = Regex::new(r"\d+").unwrap();
        let row_str = &row.into_iter().collect::<String>()[..];
        let matches = re.find_iter(row_str);
        for m in matches {
            if let Some((x, y)) = find_part_around(&input_char_grid, y, &m.range()) {
                part_numbers.push(PartNumber {
                    number: m.as_str().parse().unwrap(),
                    part_pos: (x, y),
                });
            }
        }
    }

    let puzzle_result: u32 = part_numbers
        .into_iter()
        .map(|part_number| part_number.number)
        .sum();

    println!("{puzzle_result:?}");
}

fn find_part_around(
    char_grid: &Vec<Vec<char>>,
    row: usize,
    range: &Range<usize>,
) -> Option<(usize, usize)> {
    for x in range.start.checked_sub(1).unwrap_or(range.start)..range.end + 1 {
        for y in row.checked_sub(1).unwrap_or(row)..row + 2 {
            if let Some(current_row) = char_grid.get(y) {
                if let Some(current_char) = current_row.get(x) {
                    if *current_char != '.' && !current_char.is_digit(10) {
                        return Some((x, y));
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
