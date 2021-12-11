use std::env::var;
use std::str::FromStr;

const CENTER: usize = 0;
const LEFT: usize = 1;
const RIGHT: usize = 2;
const TOP: usize = 3;
const BOTTOM: usize = 4;

struct HeightField {
    values: Vec<Vec<i32>>
}

impl FromStr for HeightField {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let values = s.split('\n').map(|line| line.chars().map(|c| c.to_digit(10).unwrap() as i32).collect::<Vec<_>>()).collect::<Vec<_>>();
        Ok(HeightField { values })
    }
}

struct MinKernel {
    min_locations: Vec<(usize, usize, i32)>
}

impl Default for MinKernel {
    fn default() -> Self {
        Self { min_locations: Vec::new() }
    }
}

impl Kernel<i32> for MinKernel {
    fn process_neighborhood(&mut self, neighborhood: [i32; 5], x: usize, y: usize) {
        let center = neighborhood[CENTER];

        let compare_values = [neighborhood[LEFT], neighborhood[RIGHT], neighborhood[TOP], neighborhood[BOTTOM]];

        let mut center_is_minimum = true;
        for value in compare_values {
            if value != -1 {
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

trait Kernel<T> {
    fn process(&mut self, data: &Vec<Vec<i32>>) {
        for (y, row) in data.iter().enumerate() {
            for (x, _) in row.iter().enumerate() {
                self.process_location(data, row.len(), x, y);
            }
        }
    }

    fn process_location(&mut self, data: &Vec<Vec<i32>>, row_length: usize, x: usize, y: usize) {
        let mut neighborhood = [-1; 5];

        neighborhood[CENTER] = data[y][x];

        if x > 0 { neighborhood[LEFT] = data[y][x-1]; }
        if y > 0 { neighborhood[TOP] = data[y-1][x]; }

        if x < row_length - 1 { neighborhood[RIGHT] = data[y][x+1]; }
        if y < data.len() - 1 { neighborhood[BOTTOM] = data[y+1][x]; }

        self.process_neighborhood(neighborhood, x, y)
    }

    fn process_neighborhood(&mut self, neighborhood: [i32; 5], x: usize, y: usize);
}

pub fn lava_tubes(data: &str) {
    let height_field = HeightField::from_str(data).unwrap();
    let mut min_kernel = MinKernel::default();
    min_kernel.process(&height_field.values);

    let min_values_sum = min_kernel.min_locations.iter().map(|(_, _, value)| value).sum::<i32>();
    println!("{}", min_values_sum)
}
