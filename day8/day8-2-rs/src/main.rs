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
                        .filter(|c| c.is_alphanumeric())
                        .collect::<String>()
                })
                .filter(|word| !word.is_empty())
                .collect::<Vec<String>>()
        })
        .map(|nodes| (nodes[0].clone(), (nodes[1].clone(), nodes[2].clone())))
        .collect();

    let mut step_count_iterators: Vec<FinalNodesIter> = map
        .keys()
        .filter(|node| node.ends_with("A"))
        .map(|node| compute_final_nodes(&node, &instructions, &map).iter())
        .collect();
    dbg!(&step_count_iterators);

    let mut current_step_counts: Vec<u64> = step_count_iterators
        .iter_mut()
        .map(|iterator: &mut FinalNodesIter| iterator.next().unwrap())
        .collect();
    while current_step_counts
        .iter()
        .any(|count| *count != current_step_counts[0])
    {
        let lowest_iterator_i = current_step_counts
            .iter()
            .enumerate()
            .min_by_key(|(_, count)| *count)
            .unwrap()
            .0;
        current_step_counts[lowest_iterator_i] =
            step_count_iterators[lowest_iterator_i].next().unwrap();
        dbg!(&current_step_counts);
    }

    let part2_result = current_step_counts.iter().min().unwrap();
    dbg!(&part2_result);
}

type Node = String;

#[derive(Debug, Clone)]
struct FinalNodes {
    loop_period: u64,
    loop_start_offset: u64,
    final_nodes_at: Vec<u64>,
}

#[derive(Debug)]
struct FinalNodesIter {
    map: FinalNodes,
    period_n: u64,
    inner_index: usize,
}

impl FinalNodes {
    fn iter(&self) -> FinalNodesIter {
        FinalNodesIter {
            map: self.clone(),
            period_n: 0,
            inner_index: 0,
        }
    }
}

impl Iterator for FinalNodesIter {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.map.loop_start_offset
            + self.map.final_nodes_at[self.inner_index]
            + (self.period_n * self.map.loop_period);
        self.inner_index = (self.inner_index + 1) % self.map.final_nodes_at.len();
        if self.inner_index == 0 {
            self.period_n.add_assign(1)
        };
        Some(result)
    }
}

fn compute_final_nodes(
    start: &str,
    instructions: &Vec<char>,
    map: &HashMap<String, (String, String)>,
) -> FinalNodes {
    let mut count: u64 = 0;
    let mut instructions_index: usize = 0;
    let mut node: &str = start;
    let mut final_nodes_at = vec![];
    let mut state_first_seen_at: HashMap<(&str, usize), u64> = HashMap::new();
    let loop_start_offset;
    loop {
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
        if node.ends_with("Z") {
            final_nodes_at.push(count)
        };
        instructions_index = (instructions_index + 1) % instructions.len();
        if let Some(loop_start) = state_first_seen_at.get(&(node, instructions_index)) {
            loop_start_offset = *loop_start;
            break;
        }
        state_first_seen_at.insert((node, instructions_index), count);
    }
    final_nodes_at = final_nodes_at
        .into_iter()
        .filter_map(|steps| steps.checked_sub(loop_start_offset))
        .collect();
    FinalNodes {
        loop_period: count - loop_start_offset,
        loop_start_offset,
        final_nodes_at,
    }
}
