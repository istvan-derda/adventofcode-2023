use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let data = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let result: u32 = data
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut digits = line
                .chars()
                .filter(|c| c.is_digit(10))
                .map(|c| c.to_digit(10).expect("These chars should all be digits"))
                .peekable();
            let first = digits.peek().expect("Line should have a first digit").clone();
            let last = digits.last().expect("Line should have a last digit").clone();
            (first * 10) + last})
        .sum();

    println!("sum {}", result);

}