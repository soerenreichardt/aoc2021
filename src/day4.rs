use core::num;

struct Grid {
    numbers: Vec<Vec<u32>>,
    found_in_rows: Vec<u32>,
    found_in_columns: Vec<u32>,
    won: bool
}

struct GridBuilder {
    numbers: Vec<Vec<u32>>
}

impl GridBuilder {
    fn new() -> Self {
        GridBuilder {
            numbers: Vec::new()
        }
    }

    fn add_row(&mut self, row: Vec<u32>) -> &Self {
        self.numbers.push(row);
        self
    }

    fn build(self) -> Grid {
        Grid { numbers: self.numbers, found_in_rows: vec![0; 5], found_in_columns: vec![0; 5], won: false }
    }
}

struct Board {
    grids: Vec<Grid>
}

impl From<Vec<&str>> for Board {
    fn from(lines: Vec<&str>) -> Self {
        let grid_builders = lines.into_iter().fold(Vec::new(), |mut builders, line| {
            match line.trim() {
                "" => {
                    builders.push(GridBuilder::new());
                    builders
                },
                str_row => {
                    let row = str_row.split_whitespace().map(|str_num| str_num.parse::<u32>().unwrap()).collect();
                    builders.last_mut().unwrap().add_row(row);
                    builders
                }
            }
        });
        
        let grids: Vec<Grid> = grid_builders.into_iter().map(|grid_builder| grid_builder.build()).collect();
        Board { grids }
    }
}

impl Grid {
    fn check_number(&mut self, number: u32) -> &mut Self {
        for (y, row) in self.numbers.iter().enumerate() {
            for (x, num) in row.iter().enumerate() {
                if *num == number {
                    self.found_in_columns[x] += 1;
                    self.found_in_rows[y] += 1;

                    if self.found_in_columns.contains(&5) || self.found_in_rows.contains(&5) {
                        self.won = true;
                    }
                }
            }
        }

        self
    }

    fn sum_unmarked(&self, marked_numbers: Vec<u32>) -> u32 {
        let mut sum = 0;
        for (y, row) in self.numbers.iter().enumerate() {
            for (x, num) in row.iter().enumerate() {
                if !marked_numbers.contains(num) {
                    sum += num;
                }
            }
        }
        sum
    }
}

pub fn play_bingo(data: String) -> u32 {
    let lines: Vec<&str> = data.lines().collect();

    let bingo_input: Vec<u32> = lines[0].split(",").map(|str_num| str_num.trim().parse::<u32>().unwrap()).collect();

    let mut board = Board::from(lines.into_iter().skip(1).collect::<Vec<&str>>());
    let mut grids = board.grids.iter_mut().collect::<Vec<_>>();

    for number in &bingo_input {
        let (won, remaining) = grids
            .into_iter()
            .map(|grid| grid.check_number(*number))
            .partition::<Vec<_>, _>(|grid| grid.won);

        if (won.len() == 1 && remaining.is_empty()) {
            println!("here");
            return number * won[0].sum_unmarked(marked_numbers(&bingo_input, *number));
        }

        grids = remaining;
    }

    0
}

fn marked_numbers(input: &Vec<u32>, target: u32) -> Vec<u32> {
    let mut input_seq: Vec<u32> = input
        .iter()
        .take_while(|&&n| n != target)
        .map(|n| *n)
        .collect();
    input_seq.push(target);
    input_seq
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
