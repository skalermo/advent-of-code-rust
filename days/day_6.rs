use std::collections::HashMap;
use std::fmt::Formatter;

pub fn solve(lines: Vec<String>) -> (i32, i32) {
    let sol1 = solution1(&lines);
    let sol2 = solution2(&lines);
    (sol1, sol2)
}

fn solution1(lines: &Vec<String>) -> i32 {
    let mut orbitees : HashMap<&str, Orbitee> = HashMap::new();
    let orbits_info = parse_input(lines);
    for (orbitee_str, orbiter_str) in orbits_info.into_iter() {
        if let Some(orbitee) = orbitees.get_mut(orbitee_str) {
            orbitee.add_orbiter(orbiter_str);
        } else {
            let mut new_orbitee = Orbitee::new();
            new_orbitee.add_orbiter(orbiter_str);
            orbitees.insert(
                orbitee_str,
                new_orbitee,
            );
        }
        if let None = orbitees.get_mut(orbiter_str) {
            let new_orbitee = Orbitee::new();
            orbitees.insert(
                orbiter_str,
                new_orbitee,
            );
        }
    }

    let mut orbits_total = 0;
    let mut stack = vec![orbitees.get("COM").unwrap()];
    let mut orbiters_on_cur_depth: Vec<&Orbitee> = Vec::new();
    let mut cur_depth = 0;

    while !stack.is_empty() {

        // empty the stack
        while !stack.is_empty() {
            let o = stack.pop().unwrap();
            for orbiter in o.orbiters() {
                orbiters_on_cur_depth.push(orbitees.get(orbiter).unwrap());
            }
            orbits_total += cur_depth;
        }
        cur_depth += 1;
        stack.append(std::mem::replace(&mut &mut orbiters_on_cur_depth, & mut vec![]));
    }

    orbits_total
}

fn solution2(lines: &Vec<String>) -> i32 {
    let mut orbitees : HashMap<&str, Orbitee> = HashMap::new();
    let orbits_info = parse_input(lines);
    for (orbitee_str, orbiter_str) in orbits_info.into_iter() {

        if let Some(orbitee) = orbitees.get_mut(orbitee_str) {
            // orbitee encountered already
            orbitee.add_orbiter(orbiter_str);
        } else {
            // orbitee seen for the first time
            let mut new_orbitee = Orbitee::new();
            new_orbitee.add_orbiter(orbiter_str);
            orbitees.insert(
                orbitee_str,
                new_orbitee,
            );
        }
        if let Some(orbiter) = orbitees.get_mut(orbiter_str) {
            // orbiter encountered already
            orbiter.set_orbit(orbitee_str);
        }
        if let None = orbitees.get_mut(orbiter_str) {
            // orbiter seen for the first time
            let mut new_orbitee = Orbitee::new();
            new_orbitee.set_orbit(orbitee_str);
            orbitees.insert(
                orbiter_str,
                new_orbitee,
            );
        }
    }

    // find and save
    // objects between obj of interest and com
    let mut from_me_to_com = vec![];
    let mut from_santa_to_com = vec![];
    let mut cur_object = "YOU";
    while cur_object != "COM" {
        cur_object = orbitees.get(cur_object).unwrap().orbits_around();
        from_me_to_com.push(cur_object);
    }
    cur_object = "SAN";
    while cur_object != "COM" {
        cur_object = orbitees.get(cur_object).unwrap().orbits_around();
        from_santa_to_com.push(cur_object);
    }

    // count matching orbits, sum up lengths and subtract matched
    let matching_orbits = from_me_to_com.iter().rev().zip(from_santa_to_com.iter().rev()).filter(|&(a, b)| a == b).count();
    (from_me_to_com.len() + from_santa_to_com.len() - matching_orbits * 2) as i32
}

#[derive(Debug)]
struct Orbitee<'a> {
    orbits_around: &'a str,
    orbiters: Vec<&'a str>,
}

impl <'a> Orbitee <'a> {
    fn new() -> Self {
        Orbitee{ orbits_around: "", orbiters: vec![]}
    }

    fn add_orbiter(&mut self, orbiter: &'a str) {
        self.orbiters.push(orbiter);
    }

    fn set_orbit(&mut self, object: &'a str) {
        self.orbits_around = object;
    }

    fn orbiters(&self) -> Vec<&'a str> {
        self.orbiters.clone()
    }

    fn orbits_around(&self) -> &'a str {
        self.orbits_around
    }

}

impl <'a> std::fmt::Display for Orbitee <'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        writeln!(f, "orbitee with orbiters: {:?}", &self.orbiters)
    }
}

fn parse_input(input: &Vec<String>) -> Vec<(&str, &str)> {
    input
        .iter()
        .map(|line| {
            let vec_line = line.split(")").collect::<Vec<&str>>();
            match &vec_line[..] {
                &[first, second, ..] => (first, second),
                _ => unreachable!(),
            }
        })
        .collect::<Vec<(&str, &str)>>()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn task1_example() {
        let input = "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L";
        assert_eq!(42, solution1(&input.split("\n").map(|str| str.to_string()).collect()));
    }

    #[test]
    fn task2_example() {
        let input = "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L\nK)YOU\nI)SAN";
        assert_eq!(4, solution2(&input.split("\n").map(|str| str.to_string()).collect()));
    }
}