use std::env;
use std::fs;

#[derive(Debug, Clone)]
struct Games {
    games: Vec<Game>,
}

#[derive(Debug, Clone)]
struct Game {
    id: u32,
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

    for game in games.games.clone() {
        println!("{game:?}");
    }

    let puzzle_1_result: u32 = games
        .games
        .iter()
        .filter(|game| is_possible_game_12r_13g_14b(game))
        .map(|game| game.id)
        .sum();

    println!("{}", puzzle_1_result);
}

fn is_possible_game_12r_13g_14b(game: &Game) -> bool {
    game.rounds
        .clone()
        .into_iter()
        .all(|round| is_possible_round_12r_13g_14b(&round))
}

fn is_possible_round_12r_13g_14b(round: &Round) -> bool {
    round.red <= 12 && round.green <= 13 && round.blue <= 14
}

fn parse_games(s: &str) -> Games {
    let games = s
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| parse_game(line))
        .collect();

    Games { games: games }
}

fn parse_game(game_str: &str) -> Game {
    let splitted: Vec<&str> = game_str.split_terminator(":").collect();
    let game_id = splitted
        .get(0)
        .unwrap()
        .strip_prefix("Game")
        .unwrap()
        .trim()
        .parse()
        .unwrap();

    let rounds = splitted
        .get(1)
        .unwrap()
        .split_terminator(";")
        .map(|round_str| parse_round(round_str))
        .collect();

    Game {
        id: game_id,
        rounds: rounds,
    }
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
