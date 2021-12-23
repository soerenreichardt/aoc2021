use std::{str::FromStr, collections::HashSet, fmt::Display};

struct Paper {
    dots: HashSet<(usize, usize)>
}

impl FromStr for Paper {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let dots = s.trim()
        .lines()
        .map(|line| match line.trim().split(",").collect::<Vec<_>>().as_slice() {
            [x, y] => (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap()),
            _ => panic!()
        }).collect::<HashSet<_>>();
        
        Ok(Paper { dots })
    }
}

impl Display for Paper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let max_x = self.dots.iter().map(|&(x, _)| x).max().unwrap();
        let max_y = self.dots.iter().map(|&(_, y)| y).max().unwrap();

        let mut paper = vec![vec!['.'; max_x + 1]; max_y + 1];
        self.dots.iter().for_each(|&(x, y)| paper[y][x] = '#');

        let mut result = String::new();
        for row in paper {
            for column in row {
                result.push(column);
            }
            result.push('\n');
        }
        write!(f, "{}", result.as_str())
    }
}

impl Paper {
    pub fn fold(&mut self, (direction, amount): &(FoldDirection, usize)) {
        println!("{:?}", (direction, amount));
        match direction {
            FoldDirection::X => self.fold_x(*amount),
            FoldDirection::Y => self.fold_y(*amount)
        }
    }

    fn fold_y(&mut self, amount: usize) {
        let mut remove: HashSet<(usize, usize)> = HashSet::new();
        let mut add: HashSet<(usize, usize)> = HashSet::new();
        self.dots.iter().for_each(|&(x, y)| {
            if y > amount {
                remove.insert((x, y));
                let new_y = amount as i32 * 2 - y as i32;
                if new_y >= 0 {
                    add.insert((x, new_y as usize));
                }
            }
        });

        for point in remove {
            self.dots.remove(&point);
        }
        for point in add {
            self.dots.insert(point);
        }
    }

    fn fold_x(&mut self, amount: usize) {
        let mut remove: HashSet<(usize, usize)> = HashSet::new();
        let mut add: HashSet<(usize, usize)> = HashSet::new();
        self.dots.iter().for_each(|&(x, y)| {
            if x > amount {
                remove.insert((x, y));
                let new_x = amount as i32 * 2 - x as i32;
                if new_x >= 0 {
                    add.insert((new_x as usize, y));
                }
            }
        });
        
        for point in remove {
            self.dots.remove(&point);
        }
        for point in add {
            self.dots.insert(point);
        }
    }
}

pub fn transparent_origami(data: &str) {
    let split = data.split("fold along").collect::<Vec<_>>();
    let mut paper = Paper::from_str(split[0]).unwrap();
    let instructions = read_instructions(split[1..].to_vec());

    instructions.iter().for_each(|instruction| paper.fold(instruction));

    println!("{}", paper);
}

#[derive(Debug)]
enum FoldDirection {
    X,
    Y
}

impl FromStr for FoldDirection {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "x" => Ok(FoldDirection::X),
            "y" => Ok(FoldDirection::Y),
            _ => panic!()
        }
    }
}

fn read_instructions(data: Vec<&str>) -> Vec<(FoldDirection, usize)> {
    data
    .iter()
    .map(|line| match &line.split('=').collect::<Vec<_>>().as_slice() {
        [direction, amount] => (FoldDirection::from_str(direction.trim()).unwrap(), amount.trim().parse::<usize>().unwrap()),
        _ => panic!()
    })
    .collect::<Vec<_>>()
}