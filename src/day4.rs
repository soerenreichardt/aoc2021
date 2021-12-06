use core::num;
use std::{collections::HashSet, ptr::null};
use std::collections::HashMap;

use crate::main;

struct BingoBoard {
    grids: Vec<Grid>,
    won_grids: HashSet<usize>,
    last_won_grid_pos: Option<(usize, u32)>,
}

enum Solve {
    FIRST,
    LAST,
}

impl BingoBoard {
    fn new(grids: Vec<Grid>) -> Self {
        BingoBoard { grids, won_grids: HashSet::new(), last_won_grid_pos: None }
    }

    fn process_number(&mut self, number: u32, solve: &Solve) -> Option<&mut Grid> {
        let winning_grid = self.grids.iter_mut()
            .enumerate()
            .map(|(pos, grid)| {
                grid.process_number(number);
                println!("{}", grid.game_won());
                (pos, grid)
            })
            .find(|(pos, grid)| grid.game_won() && !self.won_grids.contains(pos));

        println!("{:?}", self.won_grids);
        match winning_grid {
            Some((pos, grid)) => {
                self.won_grids.insert(pos);

                match solve {
                    Solve::FIRST => Some(grid),
                    Solve::LAST => {
                        self.last_won_grid_pos = Some((pos, number));
                        None
                    }
                    _ => None
                }
            }
            None => None
        }
    }

    fn process_numbers(&mut self, numbers: Vec<u32>, solve: &Solve) -> u32 {
        let mut processed_numbers: HashSet<u32> = HashSet::new();
        println!("{:?}", self.grids[1].numbers);
        for number in &numbers {
            processed_numbers.insert(*number);
            // println!("{}", number);
            match self.process_number(*number, solve) {
                Some(grid) => {
                    let unmarked_numbers_sum = BingoBoard::sum_unmarked_numbers(&processed_numbers, grid);
                    return unmarked_numbers_sum * number;
                }
                None => {}
            }
            println!("{}", number);
            println!("{:?}", self.grids[1].found_in_column);
        }
        match self.last_won_grid_pos {
            Some((pos, number)) => {
                let earlier_processed_numbers = BingoBoard::build_hashset_until_winning_number(&numbers, number);
                let unmarked_numbers_sum = BingoBoard::sum_unmarked_numbers(&earlier_processed_numbers, &mut self.grids[pos]);
                println!("{:?}", earlier_processed_numbers);
                return unmarked_numbers_sum * number;
            },
            None => 0
        }
    }

    fn build_hashset_until_winning_number(numbers: &Vec<u32>, win_number: u32) -> HashSet<u32> {
        let mut processed_numbers = HashSet::new();
        numbers.iter().take_while(|number| **number != win_number).for_each(|number| {
            processed_numbers.insert(*number);
            ()
        });
        processed_numbers
    }

    fn sum_unmarked_numbers(processed_numbers: &HashSet<u32>, grid: &mut Grid) -> u32 {
        grid.numbers
            .iter()
            .flat_map(|rows| rows.iter())
            .filter(|number| !processed_numbers.contains(number))
            .sum()
    }
}

#[derive(Hash, PartialEq, Eq)]
struct Grid {
    dimension: usize,
    numbers: Vec<Vec<u32>>,
    found_in_row: Vec<u32>,
    found_in_column: Vec<u32>,
}

impl Grid {
    fn new(numbers: Vec<Vec<u32>>) -> Grid {
        let row_length = numbers.first().unwrap().len();
        let uniform_row_length = numbers.iter().map(|row| row.len()).all(|length| length == row_length);
        if !(uniform_row_length && row_length == numbers.len()) {
            panic!("Grid is not quadratic")
        }

        Grid { dimension: row_length, numbers, found_in_row: vec![0; row_length], found_in_column: vec![0; row_length] }
    }

    fn process_number(&mut self, inserted_number: u32) {
        for (y, row) in self.numbers.iter().enumerate() {
            for (x, number) in row.iter().enumerate() {
                if *number == inserted_number {
                    self.found_in_row[y] += 1;
                    self.found_in_column[x] += 1;
                }
            }
        }
    }

    fn game_won(&self) -> bool {
        for column in &self.found_in_column {
            if *column == self.dimension as u32 {
                return true;
            }
        }

        for row in &self.found_in_row {
            if *row == self.dimension as u32 {
                return true;
            }
        }

        false
    }
}

pub fn play_bingo(data: String) -> u32 {
    let lines: Vec<&str> = data.lines().collect();

    let input = lines.first().unwrap();

    let grids: Vec<Grid> = lines.iter().skip(1).fold(Vec::new(), |mut grids_rows: Vec<Vec<Vec<u32>>>, line| {
        match line.trim() {
            a if a.is_empty() => {
                let grid_builder = Vec::new();
                grids_rows.push(grid_builder);
                grids_rows
            }
            _ => {
                let row: Vec<u32> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();
                grids_rows.last_mut().unwrap().push(row);
                grids_rows
            }
        }
    }).into_iter().map(|rows| Grid::new(rows)).collect();

    let mut board = BingoBoard::new(grids);

    let numbers: Vec<u32> = input.split(",").into_iter().map(|number_string| number_string.parse().unwrap()).collect();
    board.process_numbers(numbers, &Solve::LAST)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn foo() {
        println!("{}", play_bingo("7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

                                  22 13 17 11  0
                                  8  2 23  4 24
                                  21  9 14 16  7
                                  6 10  3 18  5
                                  1 12 20 15 19

                                  3 15  0  2 22
                                  9 18 13 17  5
                                  19  8  7 25 23
                                  20 11 10 24  4
                                  14 21 16 12  6

                                  14 21 17 24  4
                                  10 16 15  9 19
                                  18  8 23 26 20
                                  22 11 13  6  5
                                  2  0 12  3  7".to_string()));
    }
}
