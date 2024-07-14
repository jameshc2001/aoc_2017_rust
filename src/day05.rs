fn steps_to_escape<F>(input: &str, rule: F) -> i32
where
    F: Fn(&i32) -> i32
{
    let mut instructions: Vec<i32> = parse_input(input);
    let mut location: i32 = 0;
    let mut steps = 0;

    loop {
        match usize::try_from(location) {
            Ok(index) => {
                match instructions.get(index) {
                    Some(instruction) => {
                        location = location + instruction;
                        instructions[index] = rule(instruction);
                        steps = steps + 1;
                    },
                    None => return steps
                }
            }
            Err(_) => return steps
        }
    }
}

fn parse_input(input: &str) -> Vec<i32> {
    input
        .split("\n")
        .map(|n| { n.split_whitespace().collect::<String>() })
        .map(|num| { num.parse::<i32>().unwrap() })
        .collect::<Vec<_>>()
}

fn steps_to_escape_simple(input: &str) -> i32 {
    steps_to_escape(input, |instruction| { instruction + 1 })
}

fn steps_to_escape_advanced(input: &str) -> i32 {
    steps_to_escape(input, |instruction| {
        if instruction >= &3 { instruction - 1 } else { instruction + 1 }
    })
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    #[test]
    fn can_calculate_steps_to_escape() {
        let input = "0
            3
            0
            1
            -3";
        assert_eq!(steps_to_escape_simple(input), 5);
    }

    #[test]
    fn usize_try_from_works_as_expected() {
        assert_eq!(usize::try_from(5i32).unwrap(), 5);
        assert!(usize::try_from(-5i32).is_err());
    }

    #[test]
    fn part_1() {
        let input = fs::read_to_string("inputs/day05.txt").unwrap();
        assert_eq!(steps_to_escape_simple(input.as_str()), 396086);
    }

    #[test]
    fn can_calculate_steps_to_escape_advanced() {
        let input = "0
            3
            0
            1
            -3";
        assert_eq!(steps_to_escape_advanced(input), 10);
    }

    #[test]
    fn part_2() {
        let input = fs::read_to_string("inputs/day05.txt").unwrap();
        assert_eq!(steps_to_escape_advanced(input.as_str()), 28675390);
    }
}