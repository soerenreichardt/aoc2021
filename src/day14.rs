use std::collections::HashMap;

pub fn extended_polymerization(data: &str) {
    let lines = data.lines().collect::<Vec<_>>();
    let mut input = lines[0].chars().collect::<Vec<_>>();

    let mut rules: HashMap<(char, char), char> = HashMap::new();
    lines[1..].iter().filter(|line| !line.is_empty()).for_each(|line| {
        let rule = line.trim().split(" -> ").collect::<Vec<_>>();
        let mut from = rule[0].chars().collect::<Vec<_>>();
        let mut into = rule[1].chars();

        rules.insert((from[0], from[1]), into.nth(0).unwrap());
    });

    let mut rules_map: HashMap<(char, char), u64> = HashMap::new();
    input.windows(2).for_each(|window| {
        *rules_map.entry((window[0], window[1])).or_insert(0) += 1;
    });

    for _ in 0..40 {
        let mut new_rules_map: HashMap<(char, char), u64> = HashMap::new();
        rules_map.iter().for_each(|(&(a, b), count)| {
            if let Some(&matching) = rules.get(&(a, b)) {
                *new_rules_map.entry((a, matching)).or_insert(0) += count;
                *new_rules_map.entry((matching, b)).or_insert(0) += count;
            } else {
                *new_rules_map.entry((a, b)).or_insert(0) += 1;
            }
        });
        rules_map = new_rules_map
    }

    let mut char_count: HashMap<char, u64> = HashMap::new();
    rules_map.iter().for_each(|(&(a, b), count)| {
        let counter = char_count.entry(a).or_insert(0);
        *counter += count;

        let counter = char_count.entry(b).or_insert(0);
        *counter += count;
    });

    println!("{:?}", char_count);

    let mut counts = char_count.values().collect::<Vec<_>>();
    counts.sort();
    let big = *counts.last().unwrap() / 2 + 1;
    let small = *counts.first().unwrap() / 2 + 1;
    println!("{}", big - small + 1);
}