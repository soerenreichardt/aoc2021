
pub fn lanternfish_simulation(data: String) {
    let mut population_state = data.lines().into_iter()
        .flat_map(|line| line.split(",").into_iter())
        .map(|i| i.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let mut state: [u64;9] = [0; 9];

    population_state.iter().for_each(|&fish| state[fish as usize] += 1);

    for _ in 0..256 {
        let tmp = state[0];
        state[0] = state[1];
        state[1] = state[2];
        state[2] = state[3];
        state[3] = state[4];
        state[4] = state[5];
        state[5] = state[6];
        state[6] = state[7] + tmp;
        state[7] = state[8];
        state[8] = tmp;
    }

    println!("{}", state.iter().sum::<u64>());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data() {
        lanternfish_simulation("3,4,3,1,2".to_string());
    }
}
