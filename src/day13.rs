use std::{str::FromStr, collections::HashSet};

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

impl Paper {
    pub fn fold(&mut self, (direction, amount): &(FoldDirection, usize)) {
        match direction {
            FoldDirection::X => self.fold_x(*amount),
            FoldDirection::Y => self.fold_y(*amount)
        }
    }

    fn fold_y(&mut self, amount: usize) {
        let mut folded_content: Vec<Vec<bool>> = Vec::new();

        for i in 0..amount {
            let mut merged_line: Vec<bool> = Vec::new();
            for (lhs, rhs) in self.content[i].iter().zip(self.content[amount * 2 - i].iter()) {
                merged_line.push(*lhs || *rhs);
            }
            folded_content.push(merged_line);
        }
        self.content = folded_content;
    }

    fn fold_x(&mut self, amount: usize) {
        let mut folded_content: Vec<Vec<bool>> = Vec::new();

        for row in 0..self.content.len() {
            let mut merged_line: Vec<bool> = Vec::new();
            for column in 0..amount {
                merged_line.push(self.content[row][column] || self.content[row][amount * 2 - column])
            }
        }
        self.content = folded_content;
    }
}

pub fn transparent_origami(data: &str) {
    let split = data.split("fold along").collect::<Vec<_>>();
    println!("{:?}", split);
    let mut paper = Paper::from_str(split[0]).unwrap();
    let instructions = read_instructions(split[1..].to_vec());

    instructions.iter().for_each(|instruction| paper.fold(instruction));

    println!("{:?}", paper.content);
}

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
        [direction, amount] => (FoldDirection::from_str(direction).unwrap(), amount.parse::<usize>().unwrap()),
        _ => panic!()
    })
    .collect::<Vec<_>>()
}