use std::collections::{BinaryHeap, HashMap};

#[derive(PartialEq, Eq, Clone, Copy)]
struct State {
    cost: usize,
    position: (usize, usize)
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub fn chiton(data: &str) {
    let values = scaled_values(data.lines().map(|line| line.trim().chars().collect::<Vec<_>>()).collect::<Vec<_>>());

    let target = (values[0].len() - 1, values.len() - 1);

    let mut distances: HashMap<(usize, usize), usize> = HashMap::new();

    let mut heap: BinaryHeap<State> = BinaryHeap::new();

    distances.insert((0, 0), 0);

    heap.push(State { position: (0, 0), cost: 0});

    while let Some(State { position, cost }) = heap.pop() {
        if position ==  target { break; }

        if cost > *distances.entry(position).or_insert(usize::MAX) { continue; }

        let mut neighbors: Vec<State>  = Vec::new();

        if position.0 > 0 { 
            let new_position = (position.0 - 1, position.1);
            let cost_at_position = values[new_position.1][new_position.0] as usize;
            neighbors.push(State { position: new_position, cost: distances.get(&position).unwrap() + cost_at_position })
        }
        if position.1 > 0 {
            let new_position = (position.0, position.1 - 1);
            let cost_at_position = values[new_position.1][new_position.0] as usize;
            neighbors.push(State { position: new_position, cost: distances.get(&position).unwrap() + cost_at_position })
        }
        if position.0 < target.0 {
            let new_position = (position.0 + 1, position.1);
            let cost_at_position = values[new_position.1][new_position.0] as usize;
            neighbors.push(State { position: new_position, cost: distances.get(&position).unwrap() + cost_at_position })
        }
        if position.1 < target.1 {
            let new_position = (position.0, position.1 + 1);
            let cost_at_position = values[new_position.1][new_position.0] as usize;
            neighbors.push(State { position: new_position, cost: distances.get(&position).unwrap() + cost_at_position })
        }

        for neighbor in neighbors {
            if neighbor.cost < *distances.entry(neighbor.position).or_insert(usize::MAX) {
                heap.push(neighbor);
                distances.insert(neighbor.position, neighbor.cost);
            }
        }
    }

    println!("{:?}", distances.get(&target).unwrap())
}

fn scaled_values(original_values: Vec<Vec<char>>) -> Vec<Vec<u8>> {
    let parsed_values = original_values.iter().map(|line| line.iter().map(|c| c.to_digit(10).unwrap()).collect::<Vec<_>>()).collect::<Vec<_>>();

    let height = parsed_values.len();
    let width = parsed_values[0].len();

    let mut values: Vec<Vec<u8>> = vec![vec![0; width * 5]; height * 5];

    for y in 0..5 {
        let y_offset = height * y;
        for x in 0..5 {
            let x_offset = width * x;

            for (val_y, row) in parsed_values.iter().enumerate() {
                for (val_x, value) in row.iter().enumerate() {
                    let new_value = *value as u8 + y as u8 + x as u8;
                    let offset = if new_value / 10 >= 1 { 1 } else { 0 };
                    values[y_offset + val_y][x_offset + val_x] = (new_value % 10) + offset;
                }
            }

        }
    }

    values
}