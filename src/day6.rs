pub fn lanternfish_simulation(data: String) {
    let mut population_state = data.lines().into_iter()
        .flat_map(|line| line.split(",").into_iter())
        .map(|i| i.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    for day in 0..256 {
        let mut new_population_state = Vec::new();
        for fish in population_state {
            if fish == 0 {
                new_population_state.push(6);
                new_population_state.push(8);
            } else {
                new_population_state.push(fish - 1);
            }
        }
        population_state = new_population_state;
    }

    println!("{}", population_state.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data() {
        lanternfish_simulation("3,4,3,1,2".to_string());
    }
}
