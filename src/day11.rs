use std::collections::HashSet;
use std::str::FromStr;

struct OctopusField {
    values: Vec<Vec<u32>>
}

impl FromStr for OctopusField {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let values = s.lines().map(|line| line.chars().map(|c| c.to_digit(10).unwrap() as u32).collect::<Vec<_>>()).collect::<Vec<_>>();
        Ok(OctopusField { values })
    }
}

impl OctopusField {
    fn increment_value(&mut self, x: usize, y: usize) {
        self.values[y][x] += 1;
    }
}

struct FlashComputer {
    flashed: HashSet<(usize, usize)>
}

impl FlashComputer {
    fn new() -> Self {
        FlashComputer {
            flashed: HashSet::new()
        }
    }

    fn compute(&mut self, octopus_field: &mut OctopusField) -> usize {
        let mut next_positions = Vec::new();
        for (y, row) in octopus_field.values.iter().enumerate() {
            for (x, _) in row.iter().enumerate() {
                next_positions.push((x, y));
            }
        }

        while (!next_positions.is_empty()) {
            let (x, y) = next_positions.pop().unwrap();
            self.process_location(&mut octopus_field.values, x, y)
                .iter()
                .for_each(|&next_position| next_positions.push(next_position));
        }

        self.flashed.iter().for_each(|(x, y)| octopus_field.values[*y][*x] = 0);
        let flash_count = self.flashed.len();
        self.flashed.clear();

        // println!("{:?}", octopus_field.values);
        flash_count
    }

    fn process_location(&mut self, data: &mut Vec<Vec<u32>>, x: usize, y: usize) -> Vec<(usize, usize)> {
        let neighborhood = self.compute_neighborhood(data, x, y);

        data[y][x] += 1;

        let mut next_positions = Vec::new();
        if neighborhood[1][1] == 9 && !self.flashed.contains(&(x, y)) {
            self.flashed.insert((x, y));
            for (row, neighbor_row) in neighborhood.iter().enumerate() {
                for (column, value) in neighbor_row.iter().enumerate() {
                    if row == 1 && column == 1 { continue }
                    if *value != 1337 {
                        let neighbor_x = x + column - 1;
                        let neighbor_y = y + row - 1;

                        let neighbor_position = (neighbor_x, neighbor_y);
                        if !self.flashed.contains(&neighbor_position) {
                            next_positions.push(neighbor_position);
                        }
                    }
                }
            }
        }

        next_positions
    }

    fn compute_neighborhood(&self, data: &mut Vec<Vec<u32>>, x: usize, y: usize) -> [[u32; 3]; 3] {
        let mut neighborhood = [[1337; 3]; 3];
        let row_length = data[0].len();

        neighborhood[1][1] = data[y][x];

        if x > 0 && y > 0 { neighborhood[0][0] = data[y - 1][x - 1]; }

        if x > 0 { neighborhood[1][0] = data[y][x - 1]; }
        if y > 0 { neighborhood[0][1] = data[y - 1][x]; }

        if x < row_length - 1 && y < data.len() - 1 { neighborhood[2][2] = data[y + 1][x + 1]; }

        if x < row_length - 1 { neighborhood[1][2] = data[y][x + 1]; }
        if y < data.len() - 1 { neighborhood[2][1] = data[y + 1][x]; }

        if y > 0 && x < row_length - 1 { neighborhood[0][2] = data[y - 1][x + 1]; }
        if x > 0 && y < data.len() - 1 { neighborhood[2][0] = data[y + 1][x - 1]; }
        neighborhood
    }
}

pub fn dumbo_octopus(data: String) {
    let mut octopus_field = OctopusField::from_str(&data).unwrap();
    let mut flash_computer = FlashComputer::new();

    part2(&mut octopus_field, &mut flash_computer);
}

fn part1(mut octopus_field: &mut OctopusField, flash_computer: &mut FlashComputer) {
    let mut flash_count = 0;
    for _ in (0..100) {
        flash_count += flash_computer.compute(&mut octopus_field);
    }
    println!("{}", flash_count);
}

fn part2(mut octopus_field: &mut OctopusField, flash_computer: &mut FlashComputer) {
    let target_count = octopus_field.values.len() * octopus_field.values[0].len();
    let mut day_counter = 0;
    loop {
        day_counter += 1;
        if flash_computer.compute(&mut octopus_field) == target_count {
            break;
        }
    }
    println!("{}", day_counter);
}
