use std::fs;

mod day3;
mod day4;   

fn main() {
    
    // let (gamma, epsilon) = day3::binary_diagnostic(read_file("input/day3"));
    // println!("{}", gamma * epsilon);

    // let (oxygen, scubba) = day3::life_support_rating(read_file("input/day3_2"));
    // println!("{}", oxygen * scubba);)

    let result = day4::play_bingo(read_file("input/day4"));
    println!("{}", result);
}

pub fn read_file(path: &str) -> String {
    fs::read_to_string(path)
        .expect("Something went wrong reading the file")
}
