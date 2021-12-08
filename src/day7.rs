pub fn align_crabs(data: String) {
    let crab_positions = data.lines().into_iter().flat_map(|line| line.split(",").into_iter()).map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>();

    let mut median_left = (crab_positions.len() / 2);
    let mut median_right = median_left;

    let mut low_left = 0 as usize;
    let mut high_left = crab_positions.len();

    let mut low_right = low_left;
    let mut high_right = high_left;

    let mut local_best = check_alignment(&crab_positions, median_left as i32);

    high_left = median_left;
    low_right = median_left;

    let mut global_best = 1 << 30;

    while true {
        if (local_best < global_best) {
            global_best = local_best;
        }

        let left_distance = high_left - low_left;
        let right_distance = high_right - low_right;

        if left_distance <= 1 && right_distance <= 1 {
            break;
        }

        median_left = (left_distance / 2) + low_left;
        median_right = (right_distance / 2) + low_right;

        println!("median left {}", median_left);
        println!("median right {}", median_right);

        let left = check_alignment(&crab_positions, median_left as i32);
        let right = check_alignment(&crab_positions, median_right as i32);
        
        if left < right {
            low_right = median_left;
            high_right = high_left;
            high_left = median_left;

            local_best = left;
        } else {
            low_left = low_right;
            high_left = median_right;
            low_right = median_right;

            local_best = right;
        }
    }

    println!("{}", global_best);
}

fn check_alignment(crabs: &Vec<i32>, position: i32) -> i32 {
    crabs.iter().fold(0, |mut fuel, crab| {
        let distance = (*crab - position).abs();
        fuel += (distance * (distance + 1)) / 2;
        fuel
    })
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_data() {
        align_crabs("16,1,2,0,4,2,7,1,2,14".to_string())
    }
}