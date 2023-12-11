use std::collections::HashMap;
use std::env;
use std::fs;
use std::ops::AddAssign;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let input_string = fs::read_to_string(file_path).unwrap();

    let [instruction_string, map_string] =
        input_string.split_terminator("\n\n").collect::<Vec<&str>>()[0..2]
    else {
        panic!()
    };

    let instructions: Vec<char> = instruction_string.chars().collect();

    let map: HashMap<Node, (Node, Node)> = map_string
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|word| {
                    word.chars()
                        .filter(|c| c.is_alphabetic())
                        .collect::<String>()
                })
                .filter(|word| !word.is_empty())
                .collect::<Vec<String>>()
        })
        .map(|nodes| (nodes[0].clone(), (nodes[1].clone(), nodes[2].clone())))
        .collect();

    let part1_result = walk_map(instructions, map);

    dbg!(part1_result);
}

type Node = String;

fn walk_map(instructions: Vec<char>, map: HashMap<String, (String, String)>) -> u32 {
    let mut count: u32 = 0;
    let mut instructions_index: usize = 0;
    let mut node: &str = "AAA";
    while node != "ZZZ" {
        let instruction = &instructions[instructions_index];
        node = map
            .get(node)
            .map(|(left, right)| match instruction {
                'L' => left,
                'R' => right,
                _ => panic!(),
            })
            .unwrap();
        count.add_assign(1);
        instructions_index = (instructions_index + 1) % instructions.len();
    }
    count
}
