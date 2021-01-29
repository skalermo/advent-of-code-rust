use std::borrow::Borrow;

pub fn solve(lines: Vec<String>) -> (i32, i32) {
    let sol1 = solution1(lines.borrow());
    let sol2 = solution2(lines.borrow());
    (sol1, sol2)
}

fn solution1(lines: &Vec<String>) -> i32 {
    let mut sum: i32 = 0;
    for line in lines.iter() {
        let mass = line.parse::<i32>().unwrap();
        sum += calculate_needed_fuel(mass);
    }
    sum
}

fn solution2(lines: &Vec<String>) -> i32 {
    let mut sum: i32 = 0;
    for line in lines.iter() {
        let mass = line.parse::<i32>().unwrap();
        sum += calculate_needed_fuel2(mass);
    }
    sum

}

fn calculate_needed_fuel(mass: i32) -> i32 {
    mass / 3 - 2
}

fn calculate_needed_fuel2(mass: i32) -> i32 {
    let mut total_fuel = calculate_needed_fuel(mass);
    let mut fuel = calculate_needed_fuel(total_fuel);
    while fuel > 0 {
        total_fuel += fuel;
        fuel = calculate_needed_fuel(fuel);
    }
    total_fuel
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn task1_example() {
        assert_eq!(2, calculate_needed_fuel(12));
    }

    #[test]
    fn task1_example2() {
        assert_eq!(2, calculate_needed_fuel(14));
    }

    #[test]
    fn task1_example3() {
        assert_eq!(654, calculate_needed_fuel(1969));
    }

    #[test]
    fn task1_example4() {
        assert_eq!(33583, calculate_needed_fuel(100756));
    }

    #[test]
    fn task2_example1() {
        assert_eq!(2, calculate_needed_fuel2(14));
    }

    #[test]
    fn task2_example2() {
        assert_eq!(966, calculate_needed_fuel2(1969));
    }

    #[test]
    fn task2_example3() {
        assert_eq!(50346, calculate_needed_fuel2(100756));
    }
    // const EXAMPLE: &str = "1721\n979\n366\n299\n675\n1456";

    // #[test]
    // fn example() {
    //     assert_eq!(solve(&EXAMPLE.parse().unwrap(), 2), Some(514579));
    // }
    //
    // #[test]
    // fn example2() {
    //     assert_eq!(solve(&EXAMPLE.parse().unwrap(), 3), Some(241861950));
    // }
}