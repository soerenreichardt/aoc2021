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

    // let mut rules_map: HashMap<(char, char), u32> = HashMap::new();
    // input.windows(2).for_each(|window| {
    //     if let Some(&matching) = rules.get(&(window[0], window[1])) {
    //         *rules_map.entry((window[0], window[1])).or_insert(0) -= 1;

    //         *rules_map.entry((window[0], matching)).or_insert(0) += 1;
    //         *rules_map.entry((matching, window[1])).or_insert(0) += 1;
    //     }
    // });
    for _ in 0..10 {
        let mut new_input: Vec<char> = Vec::new();
        new_input.push(input[0]);
        input.windows(2).for_each(|window| {
            if let Some(matching) = rules.get(&(window[0], window[1])) {
                new_input.push(*matching);
            }
            new_input.push(window[1]);
        });
        input = new_input;
    }

    let mut char_count: HashMap<char, u32> = HashMap::new();
    input.iter().for_each(|&c| {
        let counter = char_count.entry(c).or_insert(0);
        *counter += 1;
    });

    let mut counts = char_count.values().collect::<Vec<_>>();
    counts.sort();
    println!("{}", *counts.last().unwrap() - *counts.first().unwrap());
}