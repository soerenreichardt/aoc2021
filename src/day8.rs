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
    fn find_by_length(&self, signal: &str) -> Option<&HashSet<u32>> {
        if signal.len() == 7 {
            return None;
        }

        let found_by_length = self.values
            .iter()
            .filter(|(_, value)| value.len() == signal.len())
            .map(|(_, value)| value)
            .collect::<Vec<_>>();

        if found_by_length.len() == 1 {
            return Some(found_by_length[0]);
        }
        None
    }

    fn find_by_positions_and_signal(&self, search_positions: &HashSet<u32>, signal: &str) -> Vec<&u32> {
        self.values.iter()
            .filter(|(_, value)| signal.len() == value.len() && search_positions.iter().all(|pos| value.contains(pos)))
            .map(|(key, _)| key)
            .collect::<Vec<_>>()
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
    for (pattern, output) in input {
        let mut segments = [Segment::new(), Segment::new(), Segment::new(), Segment::new(), Segment::new(), Segment::new(), Segment::new()];
        let numbers = Numbers::default();

        for _ in 0..2 {
            pattern.iter().for_each(|signal| {
                if let Some(positions) = numbers.find_by_length(signal) {
                    solve_known_pattern(&mut segments, signal, positions);
                    remove_signal_from_other_segments(&mut segments, signal, positions);
                }

                set_unknown_pattern(&mut segments, signal);

                // debug(&segments);
            });
        }

        let mut decoded_numbers: HashMap<u32, HashSet<char>> = HashMap::new();
        pattern.iter().for_each(|&signal| {
            if let Some(number) = find_best_matching_number(&mut segments, &numbers, signal, &mut decoded_numbers) {
                let number_positions = numbers.values.get(&number).unwrap();
                for (pos, segment) in segments.iter_mut().enumerate() {
                    if number_positions.contains(&(pos as u32)) {
                        segment.identifiers.retain(|c| signal.contains(|i| i == *c));
                    } else {
                        segment.identifiers = &segment.identifiers - &HashSet::from_iter(signal.chars().collect::<Vec<_>>().iter().map(|c| *c));
                    }
                }
            }
            debug(&segments);
        });

        let mut row_result = 0;
        for signal in output {
            let number = find_exact_matching_number(&mut segments, &numbers, signal);
            row_result = (row_result * 10) + number;
        }
        println!("{}", row_result);
        overall_result += row_result;
    }

    println!("{}", overall_result);
}

fn find_exact_matching_number(segments: &mut [Segment; 7], numbers: &Numbers, signal: &str) -> u32 {
    let mut positions = HashSet::new();
    for c in signal.chars() {
        for position in find_all_segment_positions_with_identifier(segments, c) {
            positions.insert(position);
        }
    }

    let found_numbers = numbers.find_by_positions_and_signal(&positions, signal);
    if found_numbers.len() == 1 {
        return *found_numbers[0];
    }

    println!("sig: {}, found numbers: {:?}", signal, found_numbers);
    panic!()
}

fn find_best_matching_number(segments: &mut [Segment; 7], numbers: &Numbers, signal: &str, decoded_numbers: &mut HashMap<u32, HashSet<char>>) -> Option<u32> {
    println!("signal: {}", signal);

    let mut counter = [0; 10];
    for c in signal.chars() {
        let segment_positions = find_all_segment_positions_with_identifier(segments, c);

        let found_numbers = numbers.find_by_positions_and_signal(&segment_positions, signal);

        found_numbers.iter()
            .filter(|num| !decoded_numbers.contains_key(num))
            .for_each(|&&num| counter[num as usize] += 1);
        println!("signal: {}, found numbers: {:?}", c, found_numbers);
    }

    let mut max_pos = 0;
    let mut max_value = 0;
    for (pos, value) in counter.iter().enumerate() {
        if *value > max_value {
            max_value = *value;
            max_pos = pos;
        }
    }

    let num_winners = counter.iter().filter(|&&i| i == max_value).count();
    if (num_winners == 1 && max_value * 2 >= signal.len()) {
        println!("best matching number: {}", max_pos);
        decoded_numbers.insert(max_pos as u32, HashSet::from_iter(signal.chars()));
        return Some(max_pos as u32);
    } else {
        return None;
    }
}

fn find_all_segment_positions_with_identifier(segments: &[Segment; 7], identifier: char) -> HashSet<u32> {
    let mut positions = HashSet::new();
    for (pos, segment) in segments.iter().enumerate() {
        if segment.identifiers.is_empty() || segment.identifiers.contains(&identifier) {
            positions.insert(pos as u32);
        }
    }
    positions
}

fn match_signal_to_number(segments: &mut [Segment; 7], numbers: &Numbers, signal: &str, number: u32) {
    let positions = numbers.values.get(&number).unwrap();
    for position in positions {
        for c in signal.chars() {
            segments[*position as usize].identifiers.retain(|identifier| &c == identifier);
        }
    }
}

fn debug(segments: &[Segment; 7]) {
    for (pos, segment) in segments.iter().enumerate() {
        println!("[{}]: {:?} -- {}", pos, segment.identifiers, segment.locked);
    }
    println!("==========")
}

fn solve_known_pattern(segments: &mut [Segment; 7], signal: &str, known_positions: &HashSet<u32>) {
    for (pos, segment) in segments.iter_mut().enumerate() {
        if segment.locked && segment.identifiers.len() <= known_positions.len() {
            continue;
        }
        if known_positions.contains(&(pos as u32)) {
            // found an assignment with less values
            segment.identifiers.clear();
            for c in signal.chars() {
                segment.identifiers.insert(c);
            }
            segment.locked = true;
        }
    }
}

fn set_unknown_pattern(segments: &mut [Segment; 7], signal: &str) {
    let mut signal_identifiers: HashSet<char> = HashSet::from_iter(signal.chars().collect::<Vec<_>>());
    let mut free_positions: HashSet<u32> = HashSet::from_iter((0..7).collect::<Vec<_>>());
    for (pos, segment) in segments.iter().enumerate() {
        for identifier in &segment.identifiers {
            if signal_identifiers.contains(&identifier) {
                signal_identifiers.remove(&identifier);
                free_positions.remove(&(pos as u32));
            }
        }
    }

    for free_position in &free_positions {
        for signal_identifier in &signal_identifiers {
            segments[*free_position as usize].identifiers.insert(*signal_identifier);
        }
    }
}

fn remove_signal_from_other_segments(segments: &mut [Segment; 7], signal: &str, positions: &HashSet<u32>) {
    for (pos, segment) in segments.iter_mut().enumerate() {
        if segment.identifiers.len() == 1 { continue }
        if !positions.contains(&(pos as u32)) {
            for c in signal.chars() {
                segment.identifiers.remove(&c);
            }
        }
    }
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
