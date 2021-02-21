mod days;
mod utils;

use days::{day_1, day_2, day_3, day_4, day_5, day_6, day_7, day_8};
use utils::lines_from_file;

fn get_input_for_day(day: i32) -> String {
    format!("inputs/day_{}/input", day)
}

pub fn main() {
    let day_to_solve = 8;

    let solve = match day_to_solve {
        1 => day_1::solve,
        2 => day_2::solve,
        3 => day_3::solve,
        4 => day_4::solve,
        5 => day_5::solve,
        6 => day_6::solve,
        7 => day_7::solve,
        8 => day_8::solve,
        _ => panic!(format!("Solution not implemented for day {}", day_to_solve)),
    };

    let solutions = solve(lines_from_file(get_input_for_day(day_to_solve).as_str()));
    println!("{:?}", solutions);
}
