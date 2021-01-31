pub fn solve(lines: Vec<String>) -> (i32, i32) {
    let sol1 = solution1(&lines);
    let sol2 = solution2(&lines);
    (sol1, sol2)
}

fn parse_input(input: &str) -> (i32, i32) {
   let mut i = input
       .split("-")
       .map(|num_str| num_str.parse::<i32>().unwrap());
    //        .chars()
    //        .map(|c| (c as u8 - '0' as u8) as i32)
    //        .collect::<Vec<i32>>());
    (i.next().unwrap(), i.next().unwrap())
}

fn check_if_is_password(num: i32) -> bool {
    let num_str = num.to_string().into_bytes();
    let mut repetition = false;
    let mut growing = true;

    for i in 0..num_str.len() - 1 {
        if num_str[i] > num_str[i+1] {
            growing = false;
            break;
        } else if num_str[i] == num_str[i+1] {
            repetition = true;
        }
    }
    growing & repetition
}


fn check_if_is_password2(num: i32) -> bool {
    let num_str = num.to_string().into_bytes();
    let mut repetitions = 0;
    let mut in_group = false;
    let mut repeating_digit = 0;
    let mut growing = true;

    for i in 0..num_str.len() - 1 {
        if num_str[i] > num_str[i+1] {
            growing = false;
            break;
        }
        if num_str[i] == num_str[i+1] && num_str[i+1] != repeating_digit {
            repetitions += 1;
            repeating_digit = num_str[i];
            in_group = false;
        } else if num_str[i+1] == repeating_digit && !in_group {
            in_group = true;
            repetitions -= 1;
        }
    }
    growing && repetitions > 0
}

fn solution1(lines: &Vec<String>) -> i32 {
    let (start, end) = parse_input(&lines[0]);
    let mut different_passwords = 0;
    for i in start..=end {
        if check_if_is_password(i) {
            different_passwords += 1;
        }
    }
    different_passwords
}

fn solution2(lines: &Vec<String>) -> i32 {
    let (start, end) = parse_input(&lines[0]);
    let mut different_passwords = 0;
    for i in start..=end {
        if check_if_is_password2(i) {
            different_passwords += 1;
        }
    }
    different_passwords
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_input() {
        assert_eq!((11, 25), parse_input("11-25"));
    }


    #[test]
    fn test_custom() {
        assert_eq!(2, solution1(&vec!["11-25".to_string()]));
    }

    #[test]
    fn task1_check_if_password() {
        assert!(check_if_is_password(122345));
        assert!(check_if_is_password(111111));
        assert!(!check_if_is_password(223450));
        assert!(!check_if_is_password(123789));
    }

    #[test]
    fn task2_check_if_password() {
        assert!(check_if_is_password2(112233));
        assert!(!check_if_is_password2(123444));
        assert!(check_if_is_password2(111122));
        assert!(!check_if_is_password2(12223));
        assert!(check_if_is_password2(224444));
    }

}


