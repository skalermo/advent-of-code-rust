use std::borrow::Borrow;

pub fn solve(lines: Vec<String>) -> (i32, i32) {
    let sol1 = solution1(lines.borrow());
    let sol2 = solution2(lines.borrow());
    (sol1, sol2)
}


fn solution1(lines: &Vec<String>) -> i32 {
    let mut program = vec_from_line(lines[0].borrow());
    program[1] = 12;
    program[2] = 2;
    run_intcode_program(program)
}

fn solution2(lines: &Vec<String>) -> i32 {
    // noun and verb each should be between 0 and 99 inclusive
    // use bruteforce
    let desired_output = 19690720;
    let input = vec_from_line(lines[0].borrow());
    let mut noun= 0;
    let mut verb= 0;
'outer:
    for n in 0..99 {
        for v in 0..99 {
            let mut program = input.clone();
            program[1] = n;
            program[2] = v;
            if run_intcode_program(program) == desired_output {
                noun = n;
                verb = v;
                break 'outer;
            }
        }
    }
    100 * noun + verb
}

fn vec_from_line(line: &str) -> Vec<i32> {
    line.split(",")
        .into_iter()
        .map(|word| word.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
}

fn run_intcode_program(mut program: Vec<i32>) -> i32 {
    let mut cur_pos = 0;
    let mut cur_opcode = program[cur_pos];
    while cur_opcode != 99 {
        let first_input_pos = program[cur_pos+1];
        let second_input_pos = program[cur_pos+2];
        let output_pos = program[cur_pos+3];

        match cur_opcode {
            1 => {
                program[output_pos as usize] =
                    program[first_input_pos as usize] + program[second_input_pos as usize];
            },
            2 => {
                program[output_pos as usize] =
                    program[first_input_pos as usize] * program[second_input_pos as usize];
            },
            _ => panic!(format!("Undefined opcode: {}", cur_opcode)),
        }
        cur_pos += 4;
        cur_opcode = program[cur_pos];
    }
    program[0]
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn task1_example() {
        let program_raw = "1,9,10,3,2,3,11,0,99,30,40,50";
        let program = vec_from_line(program_raw);
        assert_eq!(3500, run_intcode_program(program));
    }

    #[test]
    fn task1_example2() {
        let program_raw = "1,0,0,0,99";
        let program = vec_from_line(program_raw);
        assert_eq!(2, run_intcode_program(program));
    }

    #[test]
    fn task1_example3() {
        let program_raw = "2,3,0,3,99";
        let program = vec_from_line(program_raw);
        assert_eq!(2, run_intcode_program(program));
    }

    #[test]
    fn task1_example4() {
        let program_raw = "2,4,4,5,99,0";
        let program = vec_from_line(program_raw);
        assert_eq!(2, run_intcode_program(program));
    }

    #[test]
    fn task1_example5() {
        let program_raw = "1,1,1,4,99,5,6,0,99";
        let program = vec_from_line(program_raw);
        assert_eq!(30, run_intcode_program(program));
    }
}