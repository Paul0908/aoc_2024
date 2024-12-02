use std::{ops::Mul, process::exit};

use crate::{solutions::solution::Solution, utils::file_reader::read_file_in_lines};

pub struct HystorianHysteria;

impl HystorianHysteria {
    pub fn split_input_into_teams() -> (Vec<u32>, Vec<u32>) {
        let mut first_team = Vec::<u32>::new();
        let mut second_team = Vec::<u32>::new();
        match read_file_in_lines("./src/solutions/day1/input.txt") {
            Ok(lines) => lines.for_each(|line| match line {
                Ok(line) => {
                    if let Some((a, b)) = line.split_once(' ') {
                        let first_value: u32 = a
                            .trim()
                            .parse()
                            .expect("string should be a parsable number");
                        let second_value: u32 = b
                            .trim()
                            .parse()
                            .expect("string should be a parsable number");
                        first_team.push(first_value);
                        second_team.push(second_value);
                    } else {
                        println!("Error occured reading line");
                        exit(1)
                    }
                }
                Err(err) => {
                    println!("Error occured reading line: {}", err);
                    exit(1)
                }
            }),
            Err(err) => {
                println!("Error occured reading file: {}", err);
                exit(0);
            }
        };
        (first_team, second_team)
    }
}

impl Solution for HystorianHysteria {
    fn solve_first_task(&self) {
        let (mut first_team, mut second_team) = HystorianHysteria::split_input_into_teams();
        first_team.sort();
        second_team.sort();

        let mut total_distance = 0;
        first_team
            .iter()
            .zip(second_team.iter())
            .enumerate()
            .for_each(|(index, (first, second))| {
                let distance = if first < second {
                    second - first
                } else {
                    first - second
                };
                print!(
                    "current distance {} for step {} - value 1: {} - value 2: {} \n",
                    distance, index, first, second
                );
                total_distance += distance;
            });
        println!("Total distance is: {}", total_distance);
    }

    fn solve_second_task(&self) {
        let (mut first_team, mut second_team) = HystorianHysteria::split_input_into_teams();
        first_team.sort();
        second_team.sort();
        let mut second_index = 0;
        let mut similiarity_score = 0;

        first_team
            .iter()
            .enumerate()
            .for_each(|(index, first_value)| {
                let mut similiarities_found = 0;
                let mut second_value: &u32;
                let mut current_index = second_index;
                loop {
                    second_value = second_team.get(current_index).expect("should have value at index");
                    if second_value.gt(first_value) {
                        break;
                    }

                    if second_value.eq(first_value) {
                        similiarities_found += 1;
                    } else {
                        second_index = current_index;
                    }

                    current_index += 1;

                    if current_index.ge(&second_team.len()) {
                        break;
                    }
                }
                print!(
                    "similiarities for value {} found: {} - first value at pos: {} - second index is currently {} \n",
                    first_value, similiarities_found, index, second_index
                );
                if similiarities_found.gt(&0) {
                    similiarity_score += first_value.mul(similiarities_found);
                }
            });

        print!("total similiarity score is {}", similiarity_score);
    }
}
