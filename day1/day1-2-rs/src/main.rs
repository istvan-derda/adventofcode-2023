use std::env;
use std::fs;
use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let data = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let result: u32 = data
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            println!("{}", line);
            let mut result = line.to_string();

            let re = Regex::new("one|two|three|four|five|six|seven|eight|nine").unwrap();
            let matches = re.find_iter(&line[..]);
            
            for m in matches {
                result.replace_range(m.range().start..m.range().start+1, word_to_digit(m.as_str()));
            }
        
            let result_stage1 = result.clone();
            let matches = re.find_iter(&result_stage1[..]);
            
            for m in matches {
                result.replace_range(m.range().start..m.range().start+1, word_to_digit(m.as_str()));
            }
            
            println!("{}", result);
            result
        })
        .map(|line| {
            let mut digits = line
                .chars()
                .filter(|c| c.is_digit(10))
                .map(|c| c.to_digit(10).expect("These chars should all be digits"))
                .peekable();
            let first = digits.peek().expect("Line should have a first digit").clone();
            let last = digits.last().expect("Line should have a last digit").clone();
            println!("{}{}", first, last);
            (first * 10) + last})
        .sum();

    println!("sum {}", result);

}

fn word_to_digit(word: &str) -> &str {
    match word {
        "one" => "1",
        "two" => "2",
        "three" => "3",
        "four" => "4",
        "five" => "5",
        "six" => "6",
        "seven" => "7",
        "eight" => "8",
        "nine" => "9",
        _ => panic!(),
    }
}