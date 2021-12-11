use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::process::id;
use std::str::Chars;

struct Segment {
    identifiers: HashSet<char>,
    locked: bool,
}

struct Numbers {
    values: HashMap<u32, HashSet<u32>>,
}

impl Default for Numbers {
    fn default() -> Self {
        let mut values = HashMap::new();
        values.insert(0, HashSet::from_iter(vec![0, 1, 2, 4, 5, 6]));
        values.insert(1, HashSet::from_iter(vec![2, 5]));
        values.insert(2, HashSet::from_iter(vec![0, 2, 3, 4, 6]));
        values.insert(3, HashSet::from_iter(vec![0, 2, 3, 5, 6]));
        values.insert(4, HashSet::from_iter(vec![1, 2, 3, 5]));
        values.insert(5, HashSet::from_iter(vec![0, 1, 3, 5, 6]));
        values.insert(6, HashSet::from_iter(vec![0, 1, 3, 4, 5, 6]));
        values.insert(7, HashSet::from_iter(vec![0, 2, 5]));
        values.insert(8, HashSet::from_iter(vec![0, 1, 2, 3, 4, 5, 6]));
        values.insert(9, HashSet::from_iter(vec![0, 1, 2, 3, 5, 6]));

        Numbers { values }
    }
}

impl Numbers {
    fn find_by_positions(&self, search_positions: &HashSet<u32>) -> u32 {
        let found_numbers = self.values.iter()
            .filter(|(_, value)| value.len() == search_positions.len() && (search_positions - value).len() == 0)
            .map(|(key, _)| key)
            .collect::<Vec<_>>();

        if found_numbers.len() != 1 {
            panic!();
        }

        *found_numbers[0]
    }
}

impl Segment {
    fn new() -> Self {
        Segment { identifiers: HashSet::new(), locked: false }
    }
}

pub fn seven_segment_search(data: String) {
    let input = process_data(&data);

    part2(input);
}

fn part2(input: Vec<(Vec<&str>, Vec<&str>)>) {
    let mut overall_result = 0;
    for (mut pattern, output) in input {
        let mut solved_numbers: HashMap<u32, HashSet<char>> = HashMap::new();
        let mut segments = [Segment::new(), Segment::new(), Segment::new(), Segment::new(), Segment::new(), Segment::new(), Segment::new()];
        let numbers = Numbers::default();

        solve_known_patterns(&mut solved_numbers, &mut pattern, &mut segments);
        solve_nine(&mut solved_numbers, &pattern, &mut segments);
        solve_six(&mut solved_numbers, &pattern, &mut segments);
        solve_zero(&mut solved_numbers, &pattern, &mut segments);

        overall_result += find_number_for_output_signal(&output, &segments, &numbers);

        debug(&segments);
    }

    println!("{}", overall_result);
}

fn solve_known_patterns(mut solved_numbers: &mut HashMap<u32, HashSet<char>>, mut pattern: &mut Vec<&str>, mut segments: &mut [Segment; 7]) {
    pattern.sort_by(|a, b| a.len().partial_cmp(&b.len()).unwrap());
    pattern.iter().for_each(|signal| {
        let signal_char_set = HashSet::from_iter(signal.chars());
        match signal.len() {
            2 => {
                let chars = signal.chars().collect::<Vec<_>>();
                segments[2].identifiers.insert(chars[0]);
                segments[2].identifiers.insert(chars[1]);
                segments[5].identifiers.insert(chars[0]);
                segments[5].identifiers.insert(chars[1]);
                solved_numbers.insert(1, signal_char_set);
            }
            3 => {
                let identifiers = &segments[2].identifiers;
                let diff = &signal_char_set - identifiers;
                segments[0].identifiers.insert(*diff.iter().collect::<Vec<_>>()[0]);
                solved_numbers.insert(7, signal_char_set);
            }
            4 => {
                let identifiers = &segments[2].identifiers;
                let diff = &signal_char_set - identifiers;
                let diff_chars = diff.iter().collect::<Vec<_>>();
                segments[1].identifiers.insert(*diff_chars[0]);
                segments[1].identifiers.insert(*diff_chars[1]);
                segments[3].identifiers.insert(*diff_chars[0]);
                segments[3].identifiers.insert(*diff_chars[1]);
                solved_numbers.insert(4, signal_char_set);
            }
            7 => {
                let one = solved_numbers.get(&1).unwrap();
                let four = solved_numbers.get(&4).unwrap();
                let seven = solved_numbers.get(&7).unwrap();

                let identifiers = one.union(four).copied().collect::<HashSet<char>>().union(seven).copied().collect::<HashSet<char>>();
                let diff = &signal_char_set - &identifiers;
                let diff_chars = diff.iter().collect::<Vec<_>>();
                segments[4].identifiers.insert(*diff_chars[0]);
                segments[4].identifiers.insert(*diff_chars[1]);
                segments[6].identifiers.insert(*diff_chars[0]);
                segments[6].identifiers.insert(*diff_chars[1]);
                solved_numbers.insert(8, signal_char_set);
            }
            _ => ()
        }
    });
}

fn solve_nine(mut solved_numbers: &mut HashMap<u32, HashSet<char>>, pattern: &Vec<&str>, mut segments: &mut [Segment; 7]) {
    let seven_and_four: HashSet<char> = solved_numbers.get(&7).unwrap()
        .union(solved_numbers.get(&4).unwrap())
        .copied()
        .collect();

    solve_for_pattern(&mut solved_numbers, &pattern, &mut segments, &seven_and_four, 6, 9, 6);
}

fn solve_six(mut solved_numbers: &mut HashMap<u32, HashSet<char>>, pattern: &Vec<&str>, mut segments: &mut [Segment; 7]) {
    let match_pattern = &segments[1].identifiers
        .union(&segments[0].identifiers).copied().collect::<HashSet<_>>()
        .union(&segments[4].identifiers).copied().collect::<HashSet<_>>()
        .union(&segments[6].identifiers).copied().collect::<HashSet<_>>();

    solve_for_pattern(&mut solved_numbers, &pattern, &mut segments, match_pattern, 6, 6, 5);
}

fn solve_zero(mut solved_numbers: &mut HashMap<u32, HashSet<char>>, pattern: &Vec<&str>, mut segments: &mut [Segment; 7]) {
    let match_pattern = &segments[0].identifiers
        .union(&segments[2].identifiers).copied().collect::<HashSet<_>>()
        .union(&segments[4].identifiers).copied().collect::<HashSet<_>>()
        .union(&segments[5].identifiers).copied().collect::<HashSet<_>>()
        .union(&segments[6].identifiers).copied().collect::<HashSet<_>>();

    solve_for_pattern(&mut solved_numbers, &pattern, &mut segments, match_pattern, 6, 0, 1);
}

fn solve_for_pattern(
    mut solved_numbers: &mut HashMap<u32, HashSet<char>>,
    pattern: &Vec<&str>,
    mut segments: &mut [Segment; 7],
    match_pattern: &HashSet<char>,
    target_pattern_length: usize,
    solved_number: u32,
    identified_segment_position: usize
) {
    for signal in pattern {
        let signal_char_set = HashSet::from_iter(signal.chars());
        let diff = &signal_char_set - match_pattern;
        if diff.len() == 1 && signal.len() == target_pattern_length {
            solved_numbers.insert(solved_number, signal_char_set);
            let diff_char = diff.iter().nth(0).unwrap();
            segments[identified_segment_position].identifiers.retain(|c| diff_char == c);
            remove_char_from_other_segments(diff_char, segments, identified_segment_position);
        }
    }
}

fn remove_char_from_other_segments(c: &char, mut segments: &mut [Segment; 7], exclude_from_removal: usize) {
    for (pos, segment) in segments.iter_mut().enumerate() {
        if pos == exclude_from_removal { continue }

        segment.identifiers.remove(c);
    }
}

fn find_number_for_output_signal(output_signals: &Vec<&str>, segments: &[Segment; 7], numbers: &Numbers) -> u32 {
    let mut row_result = 0;
    for output_signal in output_signals {
        let positions = find_positions_for_signal(segments, output_signal);
        let decoded_number = numbers.find_by_positions(&positions);
        row_result = row_result * 10 + decoded_number;
    }

    return row_result;

    fn find_positions_for_signal(segments: &[Segment; 7], signal: &str) -> HashSet<u32> {
        let mut positions = HashSet::new();

        for c in signal.chars() {
            for (pos, segment) in segments.iter().enumerate() {
                if segment.identifiers.contains(&c) {
                    positions.insert(pos as u32);
                }
            }
        }

        assert_eq!(positions.len(), signal.len());

        positions
    }
}

fn debug(segments: &[Segment; 7]) {
    for (pos, segment) in segments.iter().enumerate() {
        println!("[{}]: {:?} -- {}", pos, segment.identifiers, segment.locked);
    }
    println!("==========")
}

fn part1(input: Vec<(Vec<&str>, Vec<&str>)>) {
    let sum: usize = input.iter().map(|(_, digits)| {
        digits.iter()
            .filter(|signal| signal.len() == 2 || signal.len() == 3 || signal.len() == 4 || signal.len() == 7)
            .count()
    }).sum();

    println!("{}", sum);
}

fn process_data(data: &String) -> Vec<(Vec<&str>, Vec<&str>)> {
    data.lines().into_iter()
        .filter(|line| !line.is_empty())
        .map(|line| line.trim().split("|")
            .collect::<Vec<&str>>())
        .into_iter()
        .map(|splits| (splits[0], splits[1]))
        .map(|(pattern, digits)| (pattern.split_whitespace().collect(), digits.split_whitespace().collect()))
        .collect()
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test() {
        seven_segment_search("edc bgecad gfaced deacb de gdfcbae aecfb gabdc acdbfg egbd | gbdac edc cefdag afegdc".to_string());
    }
}
