use core::num;

struct BingoBoard {
    grids: Vec<Grid>,
    processed_numbers: Vec<u32>
}

impl BingoBoard {
    fn new(grids: Vec<Grid>) -> Self {
        BingoBoard { grids: grids, processed_numbers: Vec::new() }
    }

    fn process_number(&mut self, number: u32) -> Option<u32> {
        self.processed_numbers.push(number);
        match self.grids.iter_mut()
                                    .map(|grid| {
                                        grid.process_number(number);
                                        grid
                                    })
                                    .find(|grid| grid.game_won()) {
            Some(grid) => {
                let unmarked_numbers = grid.numbers
                    .iter()
                    .flat_map(|rows| rows.iter())
                    .filter(|number| !self.processed_numbers.contains(*number));
                let unmarked_numbers_sum: u32 = unmarked_numbers.sum();
                Some(unmarked_numbers_sum)
            },
            None => None,
        }
    }

    fn process_numbers(mut self, numbers: Vec<u32>) -> u32 {
        for number in numbers {
            // println!("{}", number);
            match self.process_number(number) {
                Some(unmarked_sum) => return unmarked_sum * number,
                None => {},
            }
        }
    
        0
    }
}

struct Grid {
    dimension: usize,
    numbers: Vec<Vec<u32>>,
    found_in_row: Vec<u32>,
    found_in_column: Vec<u32>
}

impl Grid {
    fn new(numbers: Vec<Vec<u32>>) -> Grid {
        let row_length = numbers.first().unwrap().len();
        let uniform_row_length = numbers.iter().map(|row| row.len()).all(|length| length == row_length);
        if !(uniform_row_length && row_length == numbers.len()) {
            panic!("Grid is not quadratic")
        }
        
        Grid { dimension: row_length, numbers: numbers, found_in_row: vec![0; row_length], found_in_column: vec![0; row_length] }
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
                return true
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

    let grids: Vec<Grid> = lines.iter().skip(1).fold(Vec::new(),  |mut grids_rows: Vec<Vec<Vec<u32>>>, line| {
        match line.trim() {
            a if a.is_empty() => {
                let grid_builder = Vec::new();
                grids_rows.push(grid_builder);
                grids_rows
            },
            _ => {
                let row: Vec<u32> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();
                grids_rows.last_mut().unwrap().push(row);
                grids_rows
            }
        }
    }).into_iter().map(|rows| Grid::new(rows)).collect();
    
    let board = BingoBoard::new(grids);

    let numbers: Vec<u32> = input.split(",").into_iter().map(|number_string| number_string.parse().unwrap()).collect();
    board.process_numbers(numbers)
}