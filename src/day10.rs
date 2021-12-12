use std::collections::{HashMap, HashSet};

pub fn syntax_scoring(data: String) {
    let lines = data.lines().collect::<Vec<_>>();

    let mut match_lookup = HashMap::new();
    match_lookup.insert('(', ')');
    match_lookup.insert('[', ']');
    match_lookup.insert('{', '}');
    match_lookup.insert('<', '>');

    part1(lines, &match_lookup);
}

fn part1(lines: Vec<&str>, match_lookup: &HashMap<char, char>) {
    let mut score_lookup = HashMap::new();
    score_lookup.insert(')', 3);
    score_lookup.insert(']', 57);
    score_lookup.insert('}', 1197);
    score_lookup.insert('>', 25137);

    let match_lookup_values: HashSet<&char> = HashSet::from_iter(match_lookup.values());

    let mut stack = Vec::new();
    let mut scores = Vec::new();

    for line in lines {
        let mut score = 0;
        for c in line.trim().chars() {
            if match_lookup.contains_key(&c) {
                stack.push(c);
            }
            if match_lookup_values.contains(&c) {
                match stack.pop() {
                    Some(opening_char) => if c != *match_lookup.get(&opening_char).unwrap() {
                        score += score_lookup.get(&c).unwrap()
                    },
                    None => ()
                }
            }
        }
        scores.push(score);
    }
    println!("{:?}", scores.iter().sum::<u32>());
}
