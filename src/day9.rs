use std::collections::HashSet;
use std::env::var;
use std::str::FromStr;

const CENTER: usize = 0;
const LEFT: usize = 1;
const RIGHT: usize = 2;
const TOP: usize = 3;
const BOTTOM: usize = 4;

struct HeightField {
    values: Vec<Vec<u32>>
}

impl FromStr for HeightField {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let values = s.split('\n').map(|line| line.chars().map(|c| c.to_digit(10).unwrap() as u32).collect::<Vec<_>>()).collect::<Vec<_>>();
        Ok(HeightField { values })
    }
}

struct MinKernel {
    min_locations: Vec<(usize, usize, u32)>
}

impl Default for MinKernel {
    fn default() -> Self {
        Self { min_locations: Vec::new() }
    }
}

impl Kernel for MinKernel {
    fn process_neighborhood(&mut self, neighborhood: [u32; 5], x: usize, y: usize) {
        let center = neighborhood[CENTER];

        let compare_values = [neighborhood[LEFT], neighborhood[RIGHT], neighborhood[TOP], neighborhood[BOTTOM]];

        let mut center_is_minimum = true;
        for value in compare_values {
            if value != 1337 {
                if value <= center {
                    center_is_minimum = false;
                }
            }
        }

        if center_is_minimum {
            self.min_locations.push((x, y, center + 1));
        } 
    }
}

struct WccKernel {
    seen_positions: HashSet<(usize, usize)>,
    basin_sizes: Vec<usize>,
    next_fields: Vec<(usize, usize)>
}

impl Kernel for WccKernel {
    fn process_neighborhood(&mut self, neighborhood: [u32; 5], x: usize, y: usize) {
        self.seen_positions.insert((x, y));

        if neighborhood[LEFT] != 9 && neighborhood[LEFT] != 1337 {
            let next_position = (x - 1, y);
            if !self.seen_positions.contains(&next_position) { self.next_fields.push(next_position); }
        }
        if neighborhood[RIGHT] != 9 && neighborhood[RIGHT] != 1337 {
            let next_position = (x + 1, y);
            if !self.seen_positions.contains(&next_position) { self.next_fields.push(next_position); }
        }
        if neighborhood[TOP] != 9 && neighborhood[TOP] != 1337 {
            let next_position = (x, y - 1);
            if !self.seen_positions.contains(&next_position) { self.next_fields.push(next_position); }
        }
        if neighborhood[BOTTOM] != 9 && neighborhood[BOTTOM] != 1337 {
            let next_position = (x, y + 1);
            if !self.seen_positions.contains(&next_position) { self.next_fields.push(next_position); }
        }
    }
}

impl WccKernel {
    fn new() -> Self {
        WccKernel {
            seen_positions: HashSet::new(),
            basin_sizes: Vec::new(),
            next_fields: Vec::new()
        }
    }

    fn compute(&mut self, min_locations: &Vec<(usize, usize, u32)>, height_field: &HeightField) {
        let data = &height_field.values;
        for (x, y, _) in min_locations {
            self.next_fields.push((*x, *y));

            while !self.next_fields.is_empty() {
                let (x, y) = self.next_fields.pop().unwrap();
                self.process_location(data, x, y);
            }

            self.basin_sizes.push(self.seen_positions.len());
            self.seen_positions.clear();
        }
    }
}

trait Kernel {
    fn process(&mut self, data: &Vec<Vec<u32>>) {
        for (y, row) in data.iter().enumerate() {
            for (x, _) in row.iter().enumerate() {
                self.process_location(data, x, y);
            }
        }
    }

    fn process_location(&mut self, data: &Vec<Vec<u32>>, x: usize, y: usize) {
        let mut neighborhood = [1337; 5];
        let row_length = data[0].len();

        neighborhood[CENTER] = data[y][x];

        if x > 0 { neighborhood[LEFT] = data[y][x-1]; }
        if y > 0 { neighborhood[TOP] = data[y-1][x]; }

        if x < row_length - 1 { neighborhood[RIGHT] = data[y][x+1]; }
        if y < data.len() - 1 { neighborhood[BOTTOM] = data[y+1][x]; }

        self.process_neighborhood(neighborhood, x, y)
    }

    fn process_neighborhood(&mut self, neighborhood: [u32; 5], x: usize, y: usize);
}

pub fn lava_tubes(data: &str) {
    let height_field = HeightField::from_str(data).unwrap();
    let mut min_kernel = MinKernel::default();
    min_kernel.process(&height_field.values);

    // part1(&mut min_kernel);
    part2(&min_kernel, &height_field)
}

fn part1(min_kernel: &mut MinKernel) {
    let min_locations = &min_kernel.min_locations;
    let min_values_sum = min_locations.iter().map(|(_, _, value)| value).sum::<u32>();
    println!("{}", min_values_sum)
}

fn part2(min_kernel: &MinKernel, height_field: &HeightField) {
    let mut wcc_kernel = WccKernel::new();
    wcc_kernel.compute(&min_kernel.min_locations, height_field);
    let mut basin_sizes = wcc_kernel.basin_sizes;
    basin_sizes.sort();
    if basin_sizes.len() >= 3 {
        let biggest_basins_product = basin_sizes.pop().unwrap() * basin_sizes.pop().unwrap() * basin_sizes.pop().unwrap();
        println!("{:?}", biggest_basins_product);
    }
}
