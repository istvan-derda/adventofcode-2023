use std::cmp;
use std::env;
use std::fs;
use std::ops::Range;
use std::str::FromStr;
use std::vec;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file_path = &args[1];

    let input_string = fs::read_to_string(input_file_path).expect("Couldn't read input file");

    let almanac: Almanac = input_string.parse().unwrap();

    let flat_almanac_map = almanac.get_flat_almanac_map();

    let min_location = flat_almanac_map
        .range_rules
        .iter()
        .map(|range_rule| range_rule.destination_range.start)
        .min()
        .unwrap();

    println!("{min_location:?}");
}

#[derive(Debug)]
struct Almanac {
    seed_ranges: Vec<Range<u64>>,
    maps: Vec<AlmanacMap>,
}

#[derive(Debug)]
struct AlmanacMap {
    range_rules: Vec<RangeRule>,
}

#[derive(Debug, Clone)]
struct RangeRule {
    source_range: Range<u64>,
    destination_range: Range<u64>,
}

impl Almanac {
    fn get_flat_almanac_map(&self) -> AlmanacMap {
        let seeds_as_noop_rules: Vec<RangeRule> = self
            .seed_ranges
            .iter()
            .map(|range| RangeRule {
                source_range: range.clone(),
                destination_range: range.clone(),
            })
            .collect();
        let seeds_as_noop_map = AlmanacMap {
            range_rules: seeds_as_noop_rules,
        };
        let result = self
            .maps
            .iter()
            .fold(seeds_as_noop_map, |l_map, r_map| r_map.left_reduce(&l_map));
        result
    }
}

impl AlmanacMap {
    fn left_reduce(&self, left_map: &AlmanacMap) -> AlmanacMap {
        let mut l_rules = left_map.range_rules.clone();
        let mut result_rules = vec![];
        while let Some(l_rule) = l_rules.pop() {
            let mut l_rule_matched = false;
            for r_rule in &self.range_rules {
                if let Some((new_rule, leftover_rules)) = r_rule.left_reduce(&l_rule) {
                    result_rules.push(new_rule);
                    l_rules.extend(leftover_rules);
                    l_rule_matched = true;
                    break;
                }
            }
            if !l_rule_matched {
                result_rules.push(l_rule);
            }
        }
        AlmanacMap {
            range_rules: result_rules,
        }
    }
}

impl RangeRule {
    fn forward_lookup(&self, number: u64) -> Option<u64> {
        let s_start = self.source_range.start;
        let d_start = self.destination_range.start;
        let s_to_d: i128 = i128::from(d_start) - i128::from(s_start);
        if self.source_range.contains(&number) {
            Some(u64::try_from(i128::from(number) + s_to_d).unwrap())
        } else {
            None
        }
    }

    fn backward_lookup(&self, number: u64) -> Option<u64> {
        let s_start = self.source_range.start;
        let d_start = self.destination_range.start;
        let d_to_s: i128 = i128::from(s_start) - i128::from(d_start);
        if self.destination_range.contains(&number) {
            Some(u64::try_from(i128::from(number) + d_to_s).unwrap())
        } else {
            None
        }
    }

    fn left_reduce(&self, l_rule: &RangeRule) -> Option<(RangeRule, Vec<RangeRule>)> {
        let inner = cmp::max(l_rule.destination_range.start, self.source_range.start)
            ..cmp::min(l_rule.destination_range.end, self.source_range.end);
        if inner.is_empty() {
            return None;
        }
        let outer_top = l_rule.destination_range.start
            ..cmp::min(l_rule.destination_range.end, self.source_range.start);
        let outer_bottom = cmp::max(l_rule.destination_range.start, self.source_range.end)
            ..l_rule.destination_range.end;

        let combined = RangeRule {
            source_range: l_rule.backward_lookup(inner.start).unwrap()
                ..l_rule
                    .backward_lookup(inner.end - 1)
                    .expect(&format!("left: {l_rule:?}\nright: {self:?}\noverlap: {inner:?}")[..])
                    + 1,
            destination_range: self.forward_lookup(inner.start).unwrap()
                ..self.forward_lookup(inner.end - 1).unwrap() + 1,
        };

        let mut leftover_left_rules = vec![];
        if !outer_top.is_empty() {
            let outer_top_rule = RangeRule {
                source_range: l_rule.backward_lookup(outer_top.start).unwrap()
                    ..l_rule.backward_lookup(outer_top.end - 1).unwrap() + 1,
                destination_range: outer_top,
            };
            leftover_left_rules.push(outer_top_rule);
        }

        if !outer_bottom.is_empty() {
            let outer_bottom_rule = RangeRule {
                source_range: l_rule.backward_lookup(outer_bottom.start).unwrap()
                    ..l_rule.backward_lookup(outer_bottom.end - 1).unwrap() + 1,
                destination_range: outer_bottom,
            };
            leftover_left_rules.push(outer_bottom_rule);
        }

        Some((combined, leftover_left_rules))
    }
}

impl FromStr for Almanac {
    type Err = String;

    fn from_str(input_string: &str) -> Result<Self, Self::Err> {
        let seed_ranges: Vec<Range<u64>> = input_string
            .lines()
            .nth(0)
            .unwrap()
            .strip_prefix("seeds: ")
            .unwrap()
            .split_whitespace()
            .map(|number_str| number_str.parse::<u64>().unwrap())
            .collect::<Vec<u64>>()
            .windows(2)
            .step_by(2)
            .map(|range_def| range_def[0]..range_def[0] + range_def[1])
            .collect();

        let maps = input_string
            .split_terminator("\n\n")
            .skip(1)
            .filter(|block| !block.is_empty())
            .map(|block| block.parse::<AlmanacMap>().unwrap())
            .collect();

        //println!("{maps:?}");
        Ok(Almanac { seed_ranges, maps })
    }
}

impl FromStr for AlmanacMap {
    type Err = String;

    fn from_str(block_string: &str) -> Result<Self, Self::Err> {
        let range_rules: Vec<RangeRule> = block_string
            .lines()
            .skip(1)
            .map(|line| line.parse::<RangeRule>().unwrap())
            .collect();

        Ok(AlmanacMap { range_rules })
    }
}

impl FromStr for RangeRule {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let [destination_range_start, source_range_start, range_length] = line
            .split_whitespace()
            .map(|number_str| number_str.parse::<u64>().unwrap())
            .collect::<Vec<u64>>()[..]
        else {
            panic!()
        };

        Ok(RangeRule {
            destination_range: destination_range_start..destination_range_start + range_length,
            source_range: source_range_start..source_range_start + range_length,
        })
    }
}
