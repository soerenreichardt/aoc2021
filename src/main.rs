use std::fs;

mod day3;

fn main() {
    
    let (gamma, epsilon) = day3::binary_diagnostic(read_file("input/day3"));
    println!("{}", gamma * epsilon);
}

pub fn read_file(path: &str) -> String {
    fs::read_to_string(path)
        .expect("Something went wrong reading the file")
}
