pub fn solve(lines: Vec<String>) -> (i32, i32) {
    let sol1 = solution1(&lines);
    let sol2 = solution2(&lines);
    (sol1, sol2)
}

fn solution1(lines: &Vec<String>) -> i32 {
    let nums = parse_input(&lines[0]);
    run_intcode_program(nums)
}

fn solution2(_lines: &Vec<String>) -> i32 {
    0
    // unimplemented!()
}

fn parse_input(line: &str) -> Vec<i32> {
    line
        .split(",")
        .into_iter()
        .map(|num_str| num_str.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
}

fn run_intcode_program(mut program: Vec<i32>) -> i32 {
    let input = 1;
    let mut cur_output = 0;
    let mut cur_pos = 0;

    loop {
        let (modes, opcode) = handle_modes_opcode(program[cur_pos]);
        let mut arg1 = 0;
        let mut arg2 = 0;
        let mut arg3 = 0;

        // handle instruction arguments
        match opcode {
            1 | 2 => {
                if let Some(modes) = modes {
                    if modes[0] == 1 {
                        arg1 = program[cur_pos+1];
                    } else {
                        arg1 = program[program[cur_pos+1] as usize];
                    }
                    if modes[1] == 1 {
                        arg2 = program[cur_pos+2];
                    } else {
                        arg2 = program[program[cur_pos+2] as usize];
                    }
                } else {
                    arg1 = program[program[cur_pos+1] as usize];
                    arg2 = program[program[cur_pos+2] as usize];
                }
                arg3 = program[cur_pos+3] as usize;

                cur_pos += 4;
            },
            3 | 4 => {
                arg1 = program[cur_pos+1];

                cur_pos += 2;
            },
            _ => {
                cur_pos += 1;
            },
        }

        // handle instruction execution
        match opcode {
            99 => { break; },
            1 => {
                // addition instruction
                program[arg3] = arg1 + arg2;
            },
            2 => {
                // multiplication instruction
                program[arg3] = arg1 * arg2;
            }
            3 => {
                // input value
                program[arg1 as usize] = input;
            },
            4 => {
                // output value
                cur_output = program[arg1 as usize];
            }
            _ => {
                // test output
            }

        }
    }
    cur_output
}

fn handle_modes_opcode(num: i32) -> (Option<Vec<i32>>, i32) {
    if num < 100 {
        (None, num)
    } else {
        let modes = num / 100;
        let opcode = num % 10;
        (Some(vec![modes%10, modes/10]), opcode)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_handle_modes_opcode() {
        assert_eq!((None, 2), handle_modes_opcode(2));
        assert_eq!((Some(vec![1, 0]), 2), handle_modes_opcode(102));
        assert_eq!((Some(vec![0, 1]), 2), handle_modes_opcode(1002));
        assert_eq!((Some(vec![1, 1]), 2), handle_modes_opcode(1102));
    }

    #[test]
    fn task1_example1 () {
        let line = "3,0,4,0,99";
        assert_eq!(1, run_intcode_program(parse_input(line)));
    }
}
