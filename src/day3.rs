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

pub fn life_support_rating(data: String) -> (u32, u32) {
    let mut oxygen_lines = data.trim().lines().collect::<Vec<&str>>();
    let mut scubber_lines = oxygen_lines.clone();

    let num_oxygen_lines = oxygen_lines.len();
    let num_bits_per_line = if num_oxygen_lines > 0 { oxygen_lines.first().unwrap().chars().count() } else { return (0, 0) };

    let (_, oxygen_rating) = (0..num_bits_per_line).fold((oxygen_lines, 0), |(mut lines, oxygen_rating), position| {
        if (lines.len() == 1) {
            let bit = lines.first().unwrap().trim().chars().nth(position).unwrap().to_digit(10).unwrap();
            return (lines, oxygen_rating | (bit << (num_bits_per_line - position - 1)))
        }
        let one_bits_for_position = count_one_bits_for_position(&lines, position);
        if one_bits_for_position * 2 >= lines.len() as u32 {
            lines = lines.into_iter()
                .filter(|line| {
                    let bit = line.trim().chars().nth(position).unwrap().to_digit(10).unwrap();
                    bit == 1
                })
                .collect();
            (lines, oxygen_rating | (1 << (num_bits_per_line - position - 1)))
        } else {
            lines = lines.into_iter()
                .filter(|line| {
                    let bit = line.trim().chars().nth(position).unwrap().to_digit(10).unwrap();
                    bit == 0
                })
                .collect();
            (lines, oxygen_rating)
        }
    });

    let (_, scubber_rating) = (0..num_bits_per_line).fold((scubber_lines, 0), |(mut lines, scubber_rating), position| {
        if (lines.len() == 1) {
            let bit = lines.first().unwrap().trim().chars().nth(position).unwrap().to_digit(10).unwrap();
            return (lines, scubber_rating | (bit << (num_bits_per_line - position - 1)))
        }
        let one_bits_for_position = count_one_bits_for_position(&lines, position);
        if one_bits_for_position * 2 < lines.len() as u32 {
            lines = lines.into_iter()
                .filter(|line| {
                    let bit = line.trim().chars().nth(position).unwrap().to_digit(10).unwrap();
                    bit == 1
                })
                .collect();
            (lines, scubber_rating | (1 << (num_bits_per_line - position - 1)))
        } else {
            lines = lines.into_iter()
                .filter(|line| {
                    let bit = line.trim().chars().nth(position).unwrap().to_digit(10).unwrap();
                    bit == 0
                })
                .collect();
            (lines, scubber_rating)
        }
    });

    (oxygen_rating, scubber_rating)
}

fn count_one_bits_for_position(data: &Vec<&str>, pos: usize) -> u32 {
    data.iter()
        .map(|line| line.trim().chars().nth(pos).unwrap())
        .map(|char| char.to_digit(10).unwrap())
        .fold(0, |bit_count, bit| bit_count + bit)
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

    #[test]
    fn test_count_one_bits_for_position() {
        assert_eq!(count_one_bits_for_position(&vec!["10", "11"], 0), 2);
    }

    #[test]
    fn test_life_support_rating() {
        assert_eq!(life_support_rating("
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
        01010
        ".to_string()), (23, 10));
    }
}
