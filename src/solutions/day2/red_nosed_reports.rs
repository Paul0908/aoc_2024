use std::process::exit;

use crate::{solutions::solution::Solution, utils::file_reader::read_file_in_lines};

#[derive(PartialEq, Clone, Copy)]
enum Direction {
    UPWARDS,
    DOWNWARDS,
}
pub struct RedNosedReports;

impl RedNosedReports {
    pub fn lines_to_reports(result_line: Result<String, std::io::Error>) -> Vec<i32> {
        match result_line {
            Ok(line) => line
                .split_whitespace()
                .map(|level| {
                    level
                        .trim()
                        .parse()
                        .expect("level should be a parsable integer 32")
                })
                .collect::<Vec<i32>>(),
            Err(err) => {
                eprint!("error occured reading line {}", err);
                exit(1);
            }
        }
    }

    
}

impl Solution for RedNosedReports {
    fn solve_first_task(&self) {
        let mut valid_reports = 0;

        read_file_in_lines("./src/solutions/day2/red_nosed_reactor_reports.txt")
            .expect("red_nosed_reactor_reports.txt should be readable into lines")
            .map(|result_line| RedNosedReports::lines_to_reports(result_line))
            .collect::<Vec<Vec<i32>>>()
            .iter()
            .for_each(|report| {
                let mut optional_direction: Option<Direction> = None;
                let mut optional_previous_level = report.first().unwrap();
                let mut valid_report = true;

                for level in report.iter().skip(1) {
                    let difference = level - optional_previous_level;
                    // println!("current difference is {} for report {} in step {} - current direction is: {}", difference, index, level_index, optional_direction.map_or("none", |dir| { if dir == Direction::UPWARDS { "UPWARDS" } else { "DOWNWARDS "}}));

                    if difference.eq(&0) || difference.gt(&3) || difference.lt(&(-3)) {
                        valid_report = false;
                        break;
                    }

                    if difference.is_negative() {
                        match optional_direction {
                            Some(dir) => {
                                if dir == Direction::UPWARDS {
                                    valid_report = false;
                                    break;
                                }
                            }
                            None => optional_direction = Some(Direction::DOWNWARDS),
                        }
                    } else {
                        match optional_direction {
                            Some(dir) => {
                                if dir == Direction::DOWNWARDS {
                                    valid_report = false;
                                    break;
                                }
                            }
                            None => optional_direction = Some(Direction::UPWARDS),
                        }
                    }
                    optional_previous_level = level;
                }
                if valid_report {
                    valid_reports += 1;
                }
            });
        println!("Amount of valid reports is: {}", valid_reports);
    }

    fn solve_second_task(&self) {
        let mut valid_reports = 0;

        read_file_in_lines("./src/solutions/day2/red_nosed_reactor_reports.txt")
            .expect("red_nosed_reactor_reports.txt should be readable into lines")
            .map(|result_line| RedNosedReports::lines_to_reports(result_line))
            .collect::<Vec<Vec<i32>>>()
            .iter()
            .enumerate()
            .for_each(|(index, report)| {
                let mut possible_skips_index: Vec<usize> = vec![];
                let mut skip_tries_index = 0;
                let mut is_valid = true;
                'outer: loop {
                    let mut report_to_check = report.clone();
                    if possible_skips_index.len() > 0 {
                        if skip_tries_index.ge(&possible_skips_index.len())  {
                            is_valid = false;
                            break 'outer;
                        }
                        let to_skip_index = possible_skips_index[skip_tries_index];
                        if to_skip_index.ge(&report_to_check.len()) {
                            is_valid = false;
                            break 'outer;
                        }
                        skip_tries_index += 1;
                        let first_part = &report_to_check[possible_skips_index.first().unwrap().to_owned().checked_sub(1).unwrap_or(0)..to_skip_index];
                        println!("{} {} \n", to_skip_index + 1, possible_skips_index.last().unwrap().to_owned().clamp(0, report_to_check.len() + 2));
                        let second_part = &report_to_check[(to_skip_index + 1)..];
                        report_to_check = vec![first_part, second_part].concat();
                    }

                    let mut previous_level = report_to_check.get(0).unwrap();
                    let mut optional_direction: Option<Direction> = None;
                    let mut was_valid = false;
                    'inner: for (level_index, level) in report_to_check.iter().enumerate().skip(1) {
                        let difference = level - previous_level;
                        // println!("current difference is {} for report {} in step {} - current direction is: {}", difference, index, level_index, optional_direction.map_or("none", |dir| { if dir == Direction::UPWARDS { "UPWARDS" } else { "DOWNWARDS "}}));

                        if difference.eq(&0) || difference.gt(&3) || difference.lt(&(-3)) {
                            if possible_skips_index.len() == 0 {
                                if level_index.checked_sub(2).is_some() { possible_skips_index.push(level_index - 2) };
                                possible_skips_index.push(level_index - 1);
                                possible_skips_index.push(level_index);
                                possible_skips_index.push(level_index + 1);
                            }
                            break 'inner;
                        }

                        if difference.is_negative() {
                            match optional_direction {
                                Some(dir) => {
                                    if dir == Direction::UPWARDS {
                                        if possible_skips_index.len() == 0 {
                                            possible_skips_index.push(level_index - 2);
                                            possible_skips_index.push(level_index - 1);
                                            possible_skips_index.push(level_index);
                                            possible_skips_index.push(level_index + 1);
                                        }
                                        break 'inner;
                                    }
                                }
                                None => optional_direction = Some(Direction::DOWNWARDS),
                            }
                        } else {
                            match optional_direction {
                                Some(dir) => {
                                    if dir == Direction::DOWNWARDS {
                                        if possible_skips_index.len() == 0 {
                                            possible_skips_index.push(level_index - 2);
                                            possible_skips_index.push(level_index - 1);
                                            possible_skips_index.push(level_index);
                                            possible_skips_index.push(level_index + 1);
                                        }
                                        break 'inner;
                                    }
                                }
                                None => optional_direction = Some(Direction::UPWARDS),
                            }
                        }

                        was_valid = level_index == report_to_check.len() - 1;
                        previous_level = level;
                    }

                    if was_valid {
                        break 'outer;
                    }
                }
                if is_valid {
                    print!("{} was valid\n", index);
                    valid_reports += 1;
                }
            });
        println!(
            "Amount of valid reports with one allowed invalid level is: {}",
            valid_reports
        );
    }
}
