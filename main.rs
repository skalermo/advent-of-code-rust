mod utils;
mod days;

use utils::lines_from_file;
use days::{day_1, day_2};

fn get_input_for_day(day: i32) -> String {
    format!("inputs/day_{}/input", day)
}

pub fn main() {
    let day_to_solve = 2;

    let solve = match day_to_solve {
        1 => day_1::solve,
        2 => day_2::solve,
        _ => panic!(format!("Solution not implemented for day {}", day_to_solve)),
    };

    let solutions = solve(lines_from_file(get_input_for_day(day_to_solve).as_str()));
    println!("{:?}", solutions);
}