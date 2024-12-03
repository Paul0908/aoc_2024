use std::ops::Mul;

use crate::{solutions::solution::Solution, utils};
use regex::Regex;

pub struct MullItOver;

impl MullItOver {
    pub fn caluclate_elf_computer_mul(mut_expression: &str) -> i32 {
        if let Some((first, second)) = mut_expression.split_once(',') {
            let first_value: i32 = first
                .split_once('(')
                .and_then(|(_mul, num)| num.parse().ok())
                .unwrap();
            let second_value: i32 = second.get(..second.len() - 1).unwrap().parse().unwrap();
            first_value.mul(&second_value)
        } else {
            0
        }
    }
}

impl Solution for MullItOver {
    fn solve_first_task(&self) {
        let mul_regex = Regex::new(r"mul\((\d{1,3})\,(\d{1,3})\)").unwrap();
        let file_content =
            utils::file_reader::read_file_in_lines("./src/solutions/day3/corrupted_memory.txt")
                .unwrap()
                .filter_map(|line| line.ok())
                .collect::<String>();
        let accumulated_computer_operations: i32 = mul_regex
            .find_iter(&file_content.as_str())
            .fold(0, |acc, mul_match| {
                acc + MullItOver::caluclate_elf_computer_mul(&mul_match.as_str())
            });
        println!(
            "The computer tried to accumulate: {}",
            accumulated_computer_operations
        );
    }

    fn solve_second_task(&self) {
        let advanced_reg_ex =
            Regex::new(r"mul\((\d{1,3})\,(\d{1,3})\)|do\(\)|don\'t\(\)")
                .unwrap();
        let one_line =
            utils::file_reader::read_file_in_lines("./src/solutions/day3/corrupted_memory.txt")
                .unwrap()
                .filter_map(|line| line.ok())
                .collect::<String>();

        let mut allow_calculations = true;
        let accumulated_computer_operations: i32 = advanced_reg_ex
            .find_iter(&one_line.as_str())
            .fold(0, |acc, mul_match| {
                if mul_match.as_str().starts_with('d') {
                    allow_calculations = !mul_match.as_str().contains('\'')
                }
                if allow_calculations {
                    acc + MullItOver::caluclate_elf_computer_mul(
                        &one_line[mul_match.start()..mul_match.end()],
                    )
                } else {
                    acc
                }
            });
        println!(
            "Computer tried to accumulate with do(n't): {}",
            accumulated_computer_operations
        );
    }
}
