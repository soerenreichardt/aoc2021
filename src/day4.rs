struct BingoBoard {
    grids: Vec<Grid>
}

struct Grid {
    dimension: usize,
    numbers: Vec<Vec<u32>>
}

#[derive(Default)]
struct GridBuilder {
    dimension: usize,
    numbers: Vec<Vec<u32>>
}

impl GridBuilder {
    fn new() -> Self {
        GridBuilder {
            dimension: 0,
            numbers: Vec::new()
        }
    }

    fn add_row(mut self, row: Vec<u32>) -> GridBuilder {
        self.numbers.push(row);
        self
    }

    fn build(self) -> Grid {
        let row_length = self.numbers.first().unwrap().len();
        let uniform_row_length = self.numbers.iter().map(|row| row.len()).all(|length| length == row_length);
        if !(uniform_row_length && row_length == self.numbers.len()) {
            panic!("Grid is not quadratic")
        }
        
        Grid { dimension: row_length, numbers: self.numbers }
    }
}


pub fn play_bingo(data: String) {
    let lines: Vec<&str> = data.lines().collect();

    let input = lines.first().unwrap();

    lines.iter().skip(1).fold(Vec::new(),  | mut grid_builders: Vec<GridBuilder>, line| {
        match line.trim() {
            a if a.is_empty() => {
                let grid_builder = GridBuilder::new();
                grid_builders.push(grid_builder);
                grid_builders
            },
            _ => {
                let row: Vec<u32> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();
                grid_builders.last().unwrap().add_row(row);
                grid_builders
            }
        }
    }).iter().map(|grid_builder| grid_builder.build());
}