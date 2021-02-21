use itertools::{Itertools};

pub fn solve(lines: Vec<String>) -> (i32, i32) {
    let sol1 = solution1(&lines);
    let sol2 = solution2(&lines);
    (sol1, sol2)
}

fn solution1(lines: &Vec<String>) -> i32 {
    let program = parse_input(&lines[0]);

    let mut max_output = 0;
    let phase_settings = (0..=4).permutations(5);
    for phases in phase_settings {
        // create amplifiers with provided phase settings
        let mut amplifiers = Vec::with_capacity(5);
        for p in phases {
            amplifiers.push(Amplifier::new(program.clone(), p));
        }

        // get output from amplifiers
        let mut last_output = 0;
        for amp in amplifiers.iter_mut() {
            last_output = amp.run_intcode_program(last_output);
        }

        if last_output > max_output {
            max_output = last_output;
        }
    }

    max_output
}

fn solution2(lines: &Vec<String>) -> i32 {
    let program = parse_input(&lines[0]);

    let mut max_output = 0;
    let phase_settings = (5..=9).permutations(5);
    for phases in phase_settings {
        // create amplifiers with provided phase settings
        let mut amplifiers = Vec::with_capacity(5);
        for p in phases {
            amplifiers.push(Amplifier::new(program.clone(), p));
        }

        // get output from amplifiers
        let mut last_output = 0;
        while amplifiers.last().unwrap().waits_for_input {
            for amp in amplifiers.iter_mut() {
                last_output = amp.run_intcode_program(last_output);
            }

            if last_output > max_output {
                max_output = last_output;
            }
        }
    }
    max_output
}

fn parse_input(line: &str) -> Vec<i32> {
    line
        .split(",")
        .into_iter()
        .map(|num_str| num_str.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
}

struct Amplifier {
    pub program: Vec<i32>,
    pub instruction_pointer: usize,
    pub phase_setting: i32,
    pub phase_setting_read: bool,
    pub cur_output: Option<i32>,
    pub waits_for_input: bool,
}

impl Amplifier {
    fn new(initial_program: Vec<i32>, phase_setting: i32) -> Self {
        Amplifier {
            program: initial_program,
            instruction_pointer: 0,
            phase_setting,
            phase_setting_read: false,
            cur_output: None,
            waits_for_input: true,
        }
    }

    fn get_modes_and_opcode(&self) -> (Option<Vec<i32>>, i32) {
        // modes:
        // 0 for position mode, 1 for immediate mode
        // None if modes not provided

        let num = self.program[self.instruction_pointer];
        if num < 100 {
            (None, num)
        } else {
            let modes = num / 100;
            let opcode = num % 10;
            (Some(vec![modes%10, modes/10]), opcode)
        }
    }

    fn get_value_from_program(&self, offset: usize, mode: i32) -> i32 {
        match mode {
            0 => {
                // position mode
                self.program[self.program[self.instruction_pointer + offset] as usize]
            },
            1 => {
                // immediate mode
                self.program[self.instruction_pointer + offset]
            },
            _ => unreachable!(),
        }
    }

    fn run_intcode_program(&mut self, input: i32) -> i32 {
        loop {
            let (modes, opcode) = self.get_modes_and_opcode();
            let mut arg1 = 0;
            let mut arg2 = 0;
            let mut arg3 = 0;


            // handle instruction arguments
            match opcode {
                1 | 2 | 7 | 8 => {
                    // addition, multiplication, less than, equals instructions
                    if let Some(modes) = modes {
                        // use modes if provided
                        arg1 = self.get_value_from_program(1, modes[0]);
                        arg2 = self.get_value_from_program(2, modes[1]);
                    } else {
                        // modes are not provided, use position mode
                        arg1 = self.get_value_from_program(1, 0);
                        arg2 = self.get_value_from_program(2, 0);
                    }
                    arg3 = self.get_value_from_program(3, 1);

                    self.instruction_pointer += 4;
                },
                3 => {
                    // input instruction
                    arg1 = self.get_value_from_program(1, 1);

                    if !self.phase_setting_read {
                        self.phase_setting_read = true;
                        arg2 = self.phase_setting;
                    } else if self.waits_for_input {
                        self.waits_for_input = false;
                        arg2 = input;
                    } else {
                        self.waits_for_input = true;
                        break;
                    }

                    self.instruction_pointer += 2;
                },
                4 => {
                    // output instruction
                    if let Some(modes) = modes {
                        arg1 = self.get_value_from_program(1, modes[0]);
                    } else {
                        arg1 = self.get_value_from_program(1, 0);
                    }

                    self.instruction_pointer += 2;
                },
                5 | 6 => {
                    // jumps instructions
                    if let Some(modes) = modes {
                        arg1 = self.get_value_from_program(1, modes[0]);
                        arg2 = self.get_value_from_program(2, modes[1]);
                    } else {
                        arg1 = self.get_value_from_program(1, 0);
                        arg2 = self.get_value_from_program(2, 0);
                    }

                    self.instruction_pointer += 3;
                }
                _ => {
                    self.instruction_pointer += 1;
                },
            }
            // end handle instruction arguments

            // handle instruction execution
            match opcode {
                99 => {
                    self.waits_for_input = false;
                    break;
                },
                1 => {
                    // addition instruction
                    self.program[arg3 as usize] = arg1 + arg2;
                },
                2 => {
                    // multiplication instruction
                    self.program[arg3 as usize] = arg1 * arg2;
                }
                3 => {
                    // input value
                    self.program[arg1 as usize] = arg2;
                },
                4 => {
                    // output value
                    // cur_output = self.program[arg1 as usize];
                    self.cur_output = Some(arg1);
                }
                5 => {
                    if arg1 != 0 {
                        self.instruction_pointer = arg2 as usize;
                    }
                },
                6 => {
                    if arg1 == 0 {
                        self.instruction_pointer = arg2 as usize;
                    }
                },
                7 => {
                    self.program[arg3 as usize] = (arg1 < arg2) as i32;
                },
                8 => {
                    self.program[arg3 as usize] = (arg1 == arg2) as i32;
                },
                _ => {},
            }
        }
        // end handle instruction execution

        let output = self.cur_output.unwrap();
        self.cur_output = None;
        output
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_permutations() {
        let perms = (1..=3).permutations(3);
        itertools::assert_equal(perms, vec![
            vec![1, 2, 3],
            vec![1, 3, 2],
            vec![2, 1, 3],
            vec![2, 3, 1],
            vec![3, 1, 2],
            vec![3, 2, 1],
        ]);
    }

    #[test]
    fn task1_example1() {
        let input = "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0".to_string();
        assert_eq!(43210, solution1(&vec![input]))
    }

    #[test]
    fn task1_example2() {
        let input = "3,23,3,24,1002,24,10,24,1002,23,-1,23,\
101,5,23,23,1,24,23,23,4,23,99,0,0".to_string();
        assert_eq!(54321, solution1(&vec![input]))
    }

    #[test]
    fn task1_example3() {
        let input = "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,\
1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0".to_string();
        assert_eq!(65210, solution1(&vec![input]))
    }

    #[test]
    fn task2_example1() {
        let input = "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,\
27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5".to_string();
        assert_eq!(139629729, solution2(&vec![input]))
    }

    #[test]
    fn task2_example2() {
        let input = "3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,\
-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,\
53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10".to_string();
        assert_eq!(18216, solution2(&vec![input]))
    }
}