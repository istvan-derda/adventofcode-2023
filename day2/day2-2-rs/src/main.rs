use std::cmp;
use std::env;
use std::fs;

#[derive(Debug, Clone)]
struct Games {
    games: Vec<Game>,
}

#[derive(Debug, Clone)]
struct Game {
    rounds: Vec<Round>,
}

#[derive(Debug, Clone)]
struct Round {
    red: u32,
    green: u32,
    blue: u32,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let input_string = fs::read_to_string(file_path).unwrap();

    let games = parse_games(&input_string);

    let result: u32 = games
        .games
        .into_iter()
        .map(|game| {
            let m = game
                .rounds
                .into_iter()
                .reduce(|a, b| Round {
                    red: cmp::max(a.red, b.red),
                    green: cmp::max(a.green, b.green),
                    blue: cmp::max(a.blue, b.blue),
                })
                .unwrap();
            m.red * m.green * m.blue
        })
        .sum();

    println!("{result:?}");
}

fn parse_games(s: &str) -> Games {
    let games = s
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| parse_game(line))
        .collect();

    Games { games }
}

fn parse_game(game_str: &str) -> Game {
    let splitted: Vec<&str> = game_str.split_terminator(":").collect();

    let rounds = splitted
        .get(1)
        .unwrap()
        .split_terminator(";")
        .map(|round_str| parse_round(round_str))
        .collect();

    Game { rounds }
}

fn parse_round(round_str: &str) -> Round {
    let color_strs = round_str.split_terminator(",");
    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;
    for color_str in color_strs {
        if let Some(head) = color_str.strip_suffix("red") {
            red = head.trim().parse().unwrap();
        }
        if let Some(head) = color_str.strip_suffix("green") {
            green = head.trim().parse().unwrap();
        }

        if let Some(head) = color_str.strip_suffix("blue") {
            blue = head.trim().parse().unwrap();
        }
    }
    Round {
        red: red,
        green: green,
        blue: blue,
    }
}
