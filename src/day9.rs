use std::str::FromStr;

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
    min_values: Vec<i32>
}

impl Default for MinKernel {
    fn default() -> Self {
        Self { min_values: Vec::new() }
    }
}

impl Kernel<i32> for MinKernel {
    fn process_neighborhood(&mut self, neighborhood: [[i32; 3]; 3], x: usize, y: usize) {
        let center = neighborhood[1][1];

        let compare_values = [neighborhood[1][0], neighborhood[0][1], neighborhood[2][1], neighborhood[1][2]];

        let mut center_is_minimum = true;
        for value in compare_values {
            if value != -1 {
                if value <= center {
                    center_is_minimum = false;
                }
            }
        }

        if center_is_minimum {
            println!("x[{}] : y[{}]", x, y);
            println!("{:?}", neighborhood); 
            self.min_values.push(center + 1);
        } 
    }
}

trait Kernel<T> {
    fn process(&mut self, data: Vec<Vec<i32>>) {
        for (y, row) in data.iter().enumerate() {
            for (x, number) in row.iter().enumerate() {
                let mut neighborhood = [[-1; 3]; 3];

                neighborhood[1][1] = *number;
                if x > 0 && y > 0 {
                    neighborhood[0][0] = data[y-1][x-1];
                } 
                if x > 0 {
                    neighborhood[1][0] = data[y][x-1];
                } 
                if y > 0 {
                    neighborhood[0][1] = data[y-1][x];
                }

                if x < row.len() - 1 && y < data.len() - 1 {
                    neighborhood[2][2] = data[y+1][x+1];
                } 
                if x < row.len() - 1 {
                    neighborhood[1][2] = data[y][x+1];
                } 
                if y < data.len() - 1 {
                    neighborhood[2][1] = data[y+1][x];
                }

                if x > 0 && y < data.len() - 1 {
                    neighborhood[2][0] = data[y+1][x-1];
                }
                if y > 0 && x < row.len() - 1 {
                    neighborhood[0][2] = data[y-1][x+1];
                }

                self.process_neighborhood(neighborhood, x, y) 
            }
        }
    }

    fn process_neighborhood(&mut self, neighborhood: [[i32; 3]; 3], x: usize, y: usize);
}

pub fn lava_tubes(data: &str) {
    let height_field = HeightField::from_str(data).unwrap();
    let mut min_kernel = MinKernel::default();
    min_kernel.process(height_field.values);

    let min_values_sum = min_kernel.min_values.iter().sum::<i32>();
    println!("{}", min_values_sum)
}