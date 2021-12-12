use std::collections::{HashMap, HashSet};

pub fn syntax_scoring(data: String) {
    let lines = data.lines().collect::<Vec<_>>();

    let mut match_lookup = HashMap::new();
    match_lookup.insert('(', ')');
    match_lookup.insert('[', ']');
    match_lookup.insert('{', '}');
    match_lookup.insert('<', '>');

    part2(lines, &match_lookup);
}

fn part1(lines: Vec<&str>, match_lookup: &HashMap<char, char>) {
    let mut score_lookup: HashMap<char, u32> = HashMap::new();
    score_lookup.insert(')', 3);
    score_lookup.insert(']', 57);
    score_lookup.insert('}', 1197);
    score_lookup.insert('>', 25137);

    let match_lookup_values: HashSet<&char> = HashSet::from_iter(match_lookup.values());

    let mut scores = Vec::new();

    for line in lines {
        let mut stack: Vec<char> = Vec::new();
        if let Some(mismatch) = find_mismatching_char(line, &mut stack, match_lookup, &match_lookup_values) {

            scores.push(*score_lookup.get(&mismatch).unwrap());
        }
    }
    println!("{:?}", scores.iter().sum::<u32>());
}

fn find_mismatching_char(line: &str, stack: &mut Vec<char>, match_lookup: &HashMap<char, char>, match_lookup_values: &HashSet<&char>) -> Option<char> {
    for c in line.trim().chars() {
        if match_lookup.contains_key(&c) {
            stack.push(c);
        }
        if match_lookup_values.contains(&c) {
            match stack.pop() {
                Some(opening_char) => if c != *match_lookup.get(&opening_char).unwrap() {
                    return Some(c)
                } else { continue },
                None => continue
            };
        };
    }
    None
}

fn part2(lines: Vec<&str>, match_lookup: &HashMap<char, char>) {
    let mut score_lookup: HashMap<char, u32> = HashMap::new();
    score_lookup.insert(')', 1);
    score_lookup.insert(']', 2);
    score_lookup.insert('}', 3);
    score_lookup.insert('>', 4);

    let match_lookup_values: HashSet<&char> = HashSet::from_iter(match_lookup.values());

    let mut scores: Vec<u64> = Vec::new();
    for line in lines {
        let mut stack: Vec<char> = Vec::new();
        match find_mismatching_char(line, &mut stack, match_lookup, &match_lookup_values) {
            Some(_) => continue,
            None => ()
        }

        if stack.is_empty() { panic!() }

        let mut score: u64 = 0;
        stack.iter().rev().for_each(|c| {
            let matching_char = match_lookup.get(c).unwrap();
            score = score * 5 + *score_lookup.get(matching_char).unwrap() as u64;
        });
        scores.push(score);
    }

    scores.sort();
    println!("{}", scores[(scores.len() / 2)]);
}
