use crate::{solutions::solution::Solution, utils};

#[derive(PartialEq, Eq, Clone, Copy)]
struct Coordinate {
    y_row: usize,
    x_pos: usize,
}

#[derive(PartialEq, Eq, Clone, Copy)]
struct Letter {
    letter_as_value: i8,
    position: Coordinate, //possible_next_positions: Vec<Coordinate>,
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Direction {
    None,
    DiagonalLeftDown,
    DiagonalLeftUp,
    DiagonalRightDown,
    DiagonalRightUp,
    Upwards,
    Downwards,
    Forward,
    Backwards,
}

#[derive(PartialEq, Eq, Clone)]
struct Word {
    direction: Direction,
    letters: Vec<Letter>,
}

pub struct CeresSearch;

impl Solution for CeresSearch {
    fn solve_first_task(&self) {
        let mut word_count = 0;
        let mut current_words: Vec<Word> = vec![];
        let lines = utils::file_reader::read_file_in_lines("./src/solutions/day4/word_search.txt")
            .expect("msg")
            .filter_map(|line| line.ok())
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();

        let transfer_to_value = |c: char| -> i8 {
            match c {
                'X' => 1,
                'M' => 2,
                'A' => 3,
                'S' => 4,
                _ => -1,
            }
        };

        let define_direction = |previous: &Letter, next: &Letter| -> Direction {
            let x_dif = next.position.x_pos as i32 - previous.position.x_pos as i32;
            if x_dif == 0 {
                if previous.letter_as_value < next.letter_as_value {
                    return Direction::Downwards;
                } else {
                    return Direction::Upwards;
                }
            } else if previous.position.y_row == next.position.y_row && x_dif == 1 {
                if previous.letter_as_value < next.letter_as_value {
                    return Direction::Forward;
                } else {
                    return Direction::Backwards;
                }
            } else if x_dif == 1 {
                if previous.letter_as_value < next.letter_as_value {
                    return Direction::DiagonalLeftDown;
                } else {
                    return Direction::DiagonalLeftUp;
                }
            } else if x_dif == -1 {
                if previous.letter_as_value < next.letter_as_value {
                    return Direction::DiagonalRightDown;
                } else {
                    return Direction::DiagonalRightUp;
                }
            }

            Direction::None
        };

        let match_word = |letter: &Letter, word_to_check: &mut Vec<Word>| {
            let mut words_to_add: Vec<Word> = vec![];
            for word in word_to_check.iter_mut() {
                let last_letter = word.letters.last().unwrap();
                if last_letter.position.x_pos.abs_diff(letter.position.x_pos) <= 1
                    && last_letter.position.y_row.abs_diff(letter.position.y_row) <= 1
                    && letter.letter_as_value.abs_diff(last_letter.letter_as_value) == 1
                {
                    let direction = define_direction(last_letter, letter);
                    if word.letters.len() == 1 || direction.eq(&word.direction) {
                        let mut new_word = word.clone();
                        new_word.letters.push(*letter);
                        new_word.direction = direction;
                        words_to_add.push(new_word);
                    }
                }
            }
            word_to_check.extend(words_to_add);
        };

        for (y_row, line) in lines.iter().enumerate() {
            for (x_pos, character) in line.iter().enumerate() {
                let char_as_value = transfer_to_value(*character);
                let letter = Letter {
                    letter_as_value: char_as_value,
                    position: Coordinate { y_row, x_pos },
                };
                match_word(&letter, &mut current_words);
                if char_as_value == 1 || char_as_value == 4 {
                    current_words.push(Word {
                        direction: Direction::None,
                        letters: vec![letter],
                    });
                }
            }
            current_words = current_words
                .iter()
                .filter(|word| {
                    let finished = word.letters.len() == 4;
                    if finished {
                        word_count += 1;
                    }
                    !finished && word.letters.last().unwrap().position.y_row == y_row
                })
                .map(Word::to_owned)
                .collect();
        }
        println!("XMAS found {word_count}");
    }

    //this is horrible algo design but it works
    fn solve_second_task(&self) { 
        let mut word_count = 0;
        let mut current_words: Vec<Word> = vec![];
        let mut finished_words: Vec<Word> = vec![];
        let lines = utils::file_reader::read_file_in_lines("./src/solutions/day4/word_search.txt")
            .expect("msg")
            .filter_map(|line| line.ok())
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();

        let transfer_to_value = |c: char| -> i8 {
            match c {
                'M' => 1,
                'A' => 2,
                'S' => 3,
                _ => -1,
            }
        };

        let define_direction = |previous: &Letter, next: &Letter| -> Direction {
            let x_dif = next.position.x_pos as i32 - previous.position.x_pos as i32;
            if previous.position.y_row != next.position.y_row {
                if x_dif == 1 {
                    if previous.letter_as_value < next.letter_as_value {
                        return Direction::DiagonalLeftDown;
                    } else {
                        return Direction::DiagonalLeftUp;
                    }
                } else if x_dif == -1 {
                    if previous.letter_as_value < next.letter_as_value {
                        return Direction::DiagonalRightDown;
                    } else {
                        return Direction::DiagonalRightUp;
                    }
                }
            }

            Direction::None
        };

        let match_word = |letter: &Letter, word_to_check: &mut Vec<Word>| {
            let mut words_to_add: Vec<Word> = vec![];
            for word in word_to_check.iter_mut() {
                let last_letter = word.letters.last().unwrap();
                if last_letter.position.x_pos.abs_diff(letter.position.x_pos) == 1
                    && last_letter.position.y_row.abs_diff(letter.position.y_row) == 1
                    && letter.letter_as_value.abs_diff(last_letter.letter_as_value) == 1
                {
                    let direction = define_direction(last_letter, letter);
                    if word.letters.len() == 1 || direction.eq(&word.direction) {
                        let mut new_word = word.clone();
                        new_word.letters.push(*letter);
                        new_word.direction = direction;
                        words_to_add.push(new_word);
                    }
                }
            }
            word_to_check.extend(words_to_add);
        };

        for (y_row, line) in lines.iter().enumerate() {
            for (x_pos, character) in line.iter().enumerate() {
                let char_as_value = transfer_to_value(*character);
                let letter = Letter {
                    letter_as_value: char_as_value,
                    position: Coordinate { y_row, x_pos },
                };
                match_word(&letter, &mut current_words);
                if char_as_value == 1 || char_as_value == 3 {
                    current_words.push(Word {
                        direction: Direction::None,
                        letters: vec![letter],
                    });
                }
            }

            let (finished, unfinished): (Vec<_>, Vec<_>) = current_words
                .iter()
                .partition(|word| word.letters.len() == 3);

            finished_words.extend(finished.into_iter().cloned());
            current_words = unfinished
                .into_iter()
                .filter(|word| word.letters.last().unwrap().position.y_row == y_row)
                .map(Word::to_owned)
                .collect();
        }

        for (i, finished) in finished_words.iter().enumerate() {
            for following in &finished_words[i + 1..] {
                if finished.letters.get(1).unwrap().position
                    == following.letters.get(1).unwrap().position
                {
                    word_count += 1;
                }
            }
        }

        println!("X-MAS found {word_count}");
    }
}
