use std::env;
use std::fs;
use regex::Regex

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let data = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let result: u32 = data
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            
            let re = Regex::new("one|two|three|four|five|six|seven|eight|nine").unwrap();
            let word = re.find(line).unwrap().as_str();
            let digit = match word {
                "one" => 1,
                "two" => 2,
                "three" => 3,
                "four" => 4,
                "five" => 5,
                "six" => 6,
                "seven" => 7,
                "eight" => 8,
                "nine" => 9,
                _ => panic!(),
            }

            let line2 = line.replace(word, digit);
            println("{}")
        })
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