use std::collections::HashMap;

use crate::{solutions::solution::Solution, utils::file_reader::read_file_in_lines};

pub struct PrintQueue;

impl PrintQueue {
    fn create_rule(&self, rule: String) -> (i32, i32) {
        let rules: Vec<i32> = rule
            .split("|")
            .map(|s| s.trim().parse::<i32>().unwrap())
            .collect();
        (rules[0], rules[1])
    }

    fn parse_instruction(&self, instruction: String) -> Vec<i32> {
        instruction
            .split(',')
            .map(|s| s.parse::<i32>().unwrap())
            .collect()
    }
}

impl Solution for PrintQueue {
    fn solve_first_task(&self) {
        let lines = read_file_in_lines("./src/solutions/day5/sleigh_launch_manual.txt")
            .expect("should be abled to read file");

        let mut reversed_rules: HashMap<i32, Vec<i32>> = HashMap::new();
        let mut line_iter = lines.into_iter();

        for rule_result in line_iter.by_ref() {
            let unparsed_rule = rule_result.unwrap();
            if unparsed_rule.trim().is_empty() {
                break;
            }
            let (value, key) = self.create_rule(unparsed_rule);
            if let Some(values) = reversed_rules.get_mut(&key) {
                values.push(value);
            } else {
                reversed_rules.insert(key, vec![value]);
            }
        }
        let instructions =
            line_iter.map(|instruction_res| self.parse_instruction(instruction_res.unwrap()));
        let mut middle_page_sum = 0;
        'instructions: for instruction in instructions {
            let mut not_allowed_values: Vec<i32> = vec![];
            let mut middle_value = 0;
            let half_length = (instruction.len() + 1) / 2 - 1;
            for (i, page) in instruction.into_iter().enumerate() {
                if not_allowed_values.contains(&page) {
                    continue 'instructions;
                }
                if let Some(rules) = reversed_rules.get(&page) {
                    not_allowed_values.extend(rules.iter().cloned());
                }
                if i == half_length {
                    middle_value = page;
                }
            }
            middle_page_sum += middle_value;
        }
        println!("Sum of middle pages: {middle_page_sum}");
    }

    fn solve_second_task(&self) {
        let lines = read_file_in_lines("./src/solutions/day5/sleigh_launch_manual.txt")
            .expect("should be abled to read file");

        let mut reversed_rules: HashMap<i32, Vec<i32>> = HashMap::new();
        let mut line_iter = lines.into_iter();

        for rule_result in line_iter.by_ref() {
            let unparsed_rule = rule_result.unwrap();
            if unparsed_rule.trim().is_empty() {
                break;
            }
            let (value, key) = self.create_rule(unparsed_rule);
            if let Some(values) = reversed_rules.get_mut(&key) {
                values.push(value);
            } else {
                reversed_rules.insert(key, vec![value]);
            }
        }

        let instructions =
            line_iter.map(|instruction_res| self.parse_instruction(instruction_res.unwrap()));
        let mut middle_page_sum = 0;
        for mut instruction in instructions {
            let mut was_falsy = false;
            let half_length = (instruction.len() + 1) / 2 - 1;
            let mut i = 0;
            'outer: while i < instruction.len() {
                let current_value = instruction[i];
                let mut mini_i = i;
                if let Some(applying_rules) = reversed_rules.get(&current_value) {
                    while mini_i < instruction.len() {
                        if applying_rules.contains(&instruction[mini_i]) {
                            instruction.swap(i, mini_i);
                            was_falsy = true;
                            continue 'outer;
                        }
                        mini_i += 1;
                    }
                    i += 1;
                }
            }
            if was_falsy {
                middle_page_sum += instruction[half_length];
            }
        }
        println!("Sum of reordered middle pages: {middle_page_sum}");
    }
}
