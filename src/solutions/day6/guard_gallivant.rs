use std::{
    collections::{HashMap, VecDeque},
    ops::Range,
    vec,
};

use crate::{solutions::solution::Solution, utils::file_reader::read_file_in_lines};

pub struct GuardGallivant;

impl GuardGallivant {}

impl Solution for GuardGallivant {
    fn solve_first_task(&self) {
        let lines = read_file_in_lines("./src/solutions/day6/map.txt")
            .expect("should be abled to read file");
        let mut max_x = 0;
        let mut max_y = 0;
        let mut guard_pos: (usize, usize) = (0, 0);
        let mut obstacle_coordinates_xkey: HashMap<usize, Vec<usize>> = HashMap::new();
        let mut obstacle_coordinates_ykey: HashMap<usize, Vec<usize>> = HashMap::new();
        lines.into_iter().enumerate().for_each(|(y, line_res)| {
            let line = line_res.unwrap();
            max_y = y;
            if max_x == 0 {
                max_x = line.len() - 1;
            }

            for (x, c) in line.chars().enumerate() {
                if c.eq(&'#') {
                    if let Some(obstacles) = obstacle_coordinates_ykey.get_mut(&y) {
                        obstacles.push(x);
                    } else {
                        obstacle_coordinates_ykey.insert(y, vec![x]);
                    }

                    if let Some(obstacles) = obstacle_coordinates_xkey.get_mut(&x) {
                        obstacles.push(y);
                    } else {
                        obstacle_coordinates_xkey.insert(x, vec![y]);
                    }
                } else if c.eq(&'^') {
                    guard_pos = (x, y);
                }
            }
        });

        let mut visited = vec![vec![false; max_y + 1]; max_x + 1];

        let mark_visited =
            |map: &mut Vec<Vec<bool>>, start: (usize, usize), end: (usize, usize)| {
                if start.0 == end.0 {
                    let steps = if start.1 > end.1 {
                        end.1 + 1..start.1
                    } else {
                        start.1..end.1
                    };
                    for i in steps {
                        map[start.0][i] = true;
                    }
                } else if start.1 == end.1 {
                    let steps = if start.0 > end.0 {
                        end.0 + 1..start.0
                    } else {
                        start.0..end.0
                    };

                    for i in steps {
                        map[i][start.1] = true;
                    }
                }
            };

        let mut guard_direction: u8 = 1; // 1-4 from north to west clockwhise
        visited[guard_pos.0][guard_pos.1] = true;
        'patrol: while guard_pos.0 < max_x && guard_pos.1 < max_y {
            match guard_direction {
                1 => {
                    if let Some(mut obstacles) = obstacle_coordinates_xkey
                        .get(&guard_pos.0)
                        .and_then(|v| Some(v.iter().peekable()))
                    {
                        let mut found_obstacle = None;
                        while let Some(obstacle) = obstacles.next_if(|o| **o < guard_pos.1) {
                            found_obstacle = Some(obstacle);
                        }
                        if let Some(ob) = found_obstacle {
                            mark_visited(&mut visited, guard_pos, (guard_pos.0, *ob));
                            guard_direction = 2;
                            guard_pos.1 = *ob + 1;
                            continue;
                        }
                    }
                    mark_visited(&mut visited, guard_pos, (guard_pos.0, 0));
                    visited[guard_pos.0][0] = true;
                    break 'patrol;
                }
                2 => {
                    if let Some(mut obstacles) = obstacle_coordinates_ykey
                        .get(&guard_pos.1)
                        .and_then(|v| Some(v.iter().rev().peekable()))
                    {
                        let mut found_obstacle = None;
                        while let Some(obstacle) = obstacles.next_if(|o| **o > guard_pos.0) {
                            found_obstacle = Some(obstacle);
                        }
                        if let Some(ob) = found_obstacle {
                            mark_visited(&mut visited, guard_pos, (*ob, guard_pos.1));
                            guard_direction = 3;
                            guard_pos.0 = *ob - 1;
                            continue;
                        }
                    }
                    mark_visited(&mut visited, guard_pos, (max_x, guard_pos.1));
                    visited[max_x][guard_pos.1] = true;
                    break 'patrol;
                }
                3 => {
                    // difference to one is reversed iterator and condition in next_if
                    if let Some(mut obstacles) = obstacle_coordinates_xkey
                        .get(&guard_pos.0)
                        .and_then(|v| Some(v.iter().rev().peekable()))
                    {
                        let mut found_obstacle = None;
                        while let Some(obstacle) = obstacles.next_if(|o| **o > guard_pos.1) {
                            found_obstacle = Some(obstacle);
                        }
                        if let Some(ob) = found_obstacle {
                            mark_visited(&mut visited, guard_pos, (guard_pos.0, *ob));
                            guard_direction = 4;
                            guard_pos.1 = *ob - 1;
                            continue;
                        }
                    }
                    mark_visited(&mut visited, guard_pos, (guard_pos.0, max_y));
                    visited[guard_pos.0][max_y] = true;
                    break 'patrol;
                }
                4 => {
                    if let Some(mut obstacles) = obstacle_coordinates_ykey
                        .get(&guard_pos.1)
                        .and_then(|v| Some(v.iter().peekable()))
                    {
                        let mut found_obstacle = None;
                        while let Some(obstacle) = obstacles.next_if(|o| **o < guard_pos.0) {
                            found_obstacle = Some(obstacle);
                        }
                        if let Some(ob) = found_obstacle {
                            mark_visited(&mut visited, guard_pos, (*ob, guard_pos.1));
                            guard_direction = 1;
                            guard_pos.0 = *ob + 1;
                            continue;
                        }
                    }
                    mark_visited(&mut visited, guard_pos, (0, guard_pos.1));
                    visited[0][guard_pos.1] = true;
                    break 'patrol;
                }
                _ => panic!("guard only has 4 directions"),
            }
        }
        // !! PRINT OUT THE WALKING MAP !!
        // for y in 0..=max_y {
        //     for x in 0..=max_x {
        //         print!("{}", if visited[x][y] == 1 { "X" } else { "." });
        //     }
        //     println!();
        // }
        // !!----------------------------!!

        let steps = visited
            .into_iter()
            .flat_map(|y| y.into_iter())
            .filter(|visited| *visited)
            .count();
        println!("distinct steps: {:?}", steps);
    }

    fn solve_second_task(&self) {
        println!("TODO");
    }
}
