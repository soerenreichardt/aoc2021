use std::fs;

mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;

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

    // day9::lava_tubes(read_file("input/day9").as_str())

    // day10::syntax_scoring(read_file("input/day10"));

    // day11::dumbo_octopus(read_file("input/day11"));

    // day12::passage_pathing(read_file("input/day12").as_str())

    // day13::transparent_origami(read_file("input/day13").as_str())

    // day14::extended_polymerization(read_file("input/day14").as_str())

    day15::chiton(read_file("input/day15").as_str())
}

pub fn read_file(path: &str) -> String {
    fs::read_to_string(path)
        .expect("Something went wrong reading the file")
}
