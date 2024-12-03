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

    pub fn is_valid_report(report: &[i32]) -> bool {
        let mut optional_direction: Option<Direction> = None;

        report.windows(2).all(|levels| {
            let (previous_level, current_level) = (levels[0], levels[1]);
            let difference = current_level - previous_level;

            if difference.eq(&0) || difference.gt(&3) || difference.lt(&(-3)) {
                return false;
            }

            if difference.is_negative() {
                match optional_direction {
                    Some(dir) => {
                        if dir == Direction::UPWARDS {
                            return false;
                        }
                    }
                    None => optional_direction = Some(Direction::DOWNWARDS),
                }
            } else {
                match optional_direction {
                    Some(dir) => {
                        if dir == Direction::DOWNWARDS {
                            return false;
                        }
                    }
                    None => optional_direction = Some(Direction::UPWARDS),
                }
            }
            true
        })
    }
}

impl Solution for RedNosedReports {
    fn solve_first_task(&self) {
        let valid_reports =
            read_file_in_lines("./src/solutions/day2/red_nosed_reactor_reports.txt")
                .expect("red_nosed_reactor_reports.txt should be readable into lines")
                .map(|result_line| RedNosedReports::lines_to_reports(result_line))
                .collect::<Vec<Vec<i32>>>()
                .iter()
                .filter(|report| RedNosedReports::is_valid_report(report))
                .count();
        println!("Amount of valid reports is: {}", valid_reports);
    }

    fn solve_second_task(&self) {
        let valid_reports =
            read_file_in_lines("./src/solutions/day2/red_nosed_reactor_reports.txt")
                .expect("red_nosed_reactor_reports.txt should be readable into lines")
                .map(|result_line| RedNosedReports::lines_to_reports(result_line))
                .collect::<Vec<Vec<i32>>>()
                .iter()
                .filter(|report| {
                    (0..report.len()).any(|index| {
                        let report_before_index = &report[0..index];
                        let report_after_index = &report[(index + 1)..];
                        let report_to_check = vec![report_before_index, report_after_index].concat();
                        RedNosedReports::is_valid_report(&report_to_check)
                    })
                })
                .count();

        println!(
            "Amount of valid reports with one allowed invalid level is: {}",
            valid_reports
        );
    }
}
