use std::fs;

mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

fn main() {
    
    // let (gamma, epsilon) = day3::binary_diagnostic(read_file("input/day3"));
    // println!("{}", gamma * epsilon);

    // let (oxygen, scubba) = day3::life_support_rating(read_file("input/day3_2"));
    // println!("{}", oxygen * scubba);)

    // let result = day4::play_bingo(read_file("input/day4"));
    // println!("{}", result);

    // let num_vents = day5::find_hydrothermal_vents(read_file("input/day5"));
    // println!("{}", num_vents);

    // day6::lanternfish_simulation(read_file("input/day6"))

    // day7::align_crabs(read_file("input/day7"))

    // day8::seven_segment_search(read_file("input/day8"))

    day9::lava_tubes(read_file("input/day9").as_str() /*"2199943210
3987894921
9856789892
8767896789
9899965678"*/)
}

pub fn read_file(path: &str) -> String {
    fs::read_to_string(path)
        .expect("Something went wrong reading the file")
}
