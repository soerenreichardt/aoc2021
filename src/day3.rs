use core::num;

pub fn binary_diagnostic(data: String) -> (i32, i32) {

    let mut lines = data.lines();
    let number_length = lines.nth(0).unwrap().len();
    let most_common_treshold = lines.collect::<Vec<&str>>().len() / 2;

    let one_bit_counts = count_one_bits(data, number_length);

    let (gamma, epsilon, _) = one_bit_counts.iter().rev()
    .fold((0, 0, 0), |(mut gamma, mut epsilon, pos), one_bit_count| {
        if one_bit_count >= &most_common_treshold {
            gamma = gamma | (1 << pos)
        } else {
            epsilon = epsilon | (1 << pos)
        }

        (gamma, epsilon, pos + 1)
    });

    (gamma, epsilon)
}

fn count_one_bits(data: String, number_length: usize) -> Vec<usize> {
    data.lines().fold(vec![0; number_length], |mut acc: Vec<usize>, row| {
        let chars = row.trim().chars();

        for (pos, char) in chars.into_iter().enumerate() {
            let bit_at_pos = char.to_digit(10).unwrap() as usize;
            acc[pos] = acc[pos] + bit_at_pos;
        }

        acc
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_one_bits() {
        let input = "
        00100
        11110
        10110
        10111
        10101
        01111
        00111
        11100
        10000
        11001
        00010
        01010";

        assert_eq!(count_one_bits(input.to_string(), 5), [7, 5, 8, 7, 5]);
    }
}
