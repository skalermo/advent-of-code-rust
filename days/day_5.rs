pub fn solve(lines: Vec<String>) -> (i32, i32) {
    let sol1 = solution1(&lines);
    let sol2 = solution2(&lines);
    (sol1, sol2)
}

fn solution1(lines: &Vec<String>) -> i32 {
    let nums = parse_input(&lines[0]);
    run_intcode_program(nums, 1)
}

fn solution2(lines: &Vec<String>) -> i32 {
    let nums = parse_input(&lines[0]);
    run_intcode_program(nums, 5)
    // unimplemented!()
}

fn parse_input(line: &str) -> Vec<i32> {
    line
        .split(",")
        .into_iter()
        .map(|num_str| num_str.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
}

fn run_intcode_program(mut program: Vec<i32>, input: i32) -> i32 {
    let mut cur_output = 0;
    let mut ins_ptr = 0;

    loop {
        let (modes, opcode) = handle_modes_opcode(program[ins_ptr]);
        let mut arg1 = 0;
        let mut arg2 = 0;
        let mut arg3 = 0;

        // handle instruction arguments
        match opcode {
            1 | 2 | 7 | 8 => {
                if let Some(modes) = modes {
                    if modes[0] == 1 {
                        arg1 = program[ins_ptr+1];
                    } else {
                        arg1 = program[program[ins_ptr+1] as usize];
                    }
                    if modes[1] == 1 {
                        arg2 = program[ins_ptr+2];
                    } else {
                        arg2 = program[program[ins_ptr+2] as usize];
                    }
                } else {
                    arg1 = program[program[ins_ptr+1] as usize];
                    arg2 = program[program[ins_ptr+2] as usize];
                }
                arg3 = program[ins_ptr+3] as usize;

                ins_ptr += 4;
            },
            3 => {
                arg1 = program[ins_ptr+1];

                ins_ptr += 2;
            },
            4 => {
                if let Some(modes) = modes {
                    if modes[0] == 1 {
                        arg1 = program[ins_ptr+1];
                    } else {
                        arg1 = program[program[ins_ptr+1] as usize];
                    }
                } else {
                    arg1 = program[program[ins_ptr+1] as usize];
                }

                ins_ptr += 2;
            },
            5 | 6 => {
                if let Some(modes) = modes {
                    if modes[0] == 1 {
                        arg1 = program[ins_ptr+1];
                    } else {
                        arg1 = program[program[ins_ptr+1] as usize];
                    }
                    if modes[1] == 1 {
                        arg2 = program[ins_ptr+2];
                    } else {
                        arg2 = program[program[ins_ptr+2] as usize];
                    }
                } else {
                    arg1 = program[program[ins_ptr+1] as usize];
                    arg2 = program[program[ins_ptr+2] as usize];
                }

                ins_ptr += 3;
            }
            _ => {
                ins_ptr += 1;
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
                // cur_output = program[arg1 as usize];
                cur_output = arg1
            }
            5 => {
                if arg1 != 0 {
                    ins_ptr = arg2 as usize;
                }
            },
            6 => {
                if arg1 == 0 {
                    ins_ptr = arg2 as usize;
                }
            },
            7 => {
                program[arg3] = (arg1 < arg2) as i32;
            },
            8 => {
                program[arg3] = (arg1 == arg2) as i32;
            }
            _ => {}
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
        assert_eq!(1, run_intcode_program(parse_input(line), 1));
    }

    #[test]
    fn task2_example1_less_than_position_mode() {
        let line = "3,9,7,9,10,9,4,9,99,-1,8";
        assert_eq!(1, run_intcode_program(parse_input(line), 7));
        assert_eq!(0, run_intcode_program(parse_input(line), 9));
    }

    #[test]
    fn task2_example2_eq_position_mode() {
        let line = "3,9,8,9,10,9,4,9,99,-1,8";
        assert_eq!(1, run_intcode_program(parse_input(line), 8));
        assert_eq!(0, run_intcode_program(parse_input(line), 9));
    }

    #[test]
    fn task2_example3_less_than_immediate_mode() {
        let line = "3,3,1107,-1,8,3,4,3,99";
        assert_eq!(1, run_intcode_program(parse_input(line), 7));
        assert_eq!(0, run_intcode_program(parse_input(line), 9));
    }

    #[test]
    fn task2_example4_eq_immediate_mode() {
        let line = "3,3,1108,-1,8,3,4,3,99";
        assert_eq!(1, run_intcode_program(parse_input(line), 8));
        assert_eq!(0, run_intcode_program(parse_input(line), 9));
    }

    #[test]
    fn task2_example5_jump_if_false_position_mode() {
        let line = "3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9";
        assert_eq!(0, run_intcode_program(parse_input(line), 0));
        assert_eq!(1, run_intcode_program(parse_input(line), 9));
    }

    #[test]
    fn task2_example6_jump_if_true_immediate_mode() {
        let line = "3,3,1105,-1,9,1101,0,0,12,4,12,99,1";
        assert_eq!(0, run_intcode_program(parse_input(line), 0));
        assert_eq!(1, run_intcode_program(parse_input(line), 9));
    }

    #[test]
    fn task2_example7_larger_example() {
        let line = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,\
        20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,\
        1,46,1101,1000,1,20,4,20,1105,1,46,98,99";
        assert_eq!(999, run_intcode_program(parse_input(line), 7));
        assert_eq!(1000, run_intcode_program(parse_input(line), 8));
        assert_eq!(1001, run_intcode_program(parse_input(line), 9));
    }


}
