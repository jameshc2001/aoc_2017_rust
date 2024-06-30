
fn checksum<F>(input: &str, method: F) -> i32
where
    F: Fn(Vec<i32>) -> i32
{
    input.split("\n")
        .map(|line| parse_line(line))
        .map(|nums| method(nums))
        .sum()
}

fn parse_line(line: &str) -> Vec<i32> {
    line.split_whitespace()
        .collect::<Vec<_>>()
        .iter()
        .flat_map(|num| { num.parse::<i32>() })
        .collect::<Vec<_>>()
}

fn min_max_difference_checksum(input: &str) -> i32 {
    checksum(input, |nums| {
        let min = nums.iter().min().unwrap();
        let max = nums.iter().max().unwrap();
        max - min
    })
}

fn division_checksum(input: &str) -> i32 {
    checksum(input, |nums| {
        for (i, n) in nums.iter().enumerate() {
            for (j, o) in nums.iter().enumerate() {
                if i == j { continue }
                if n % o == 0 { return n / o }
            }
        }
        panic!("no division found")
    })
}


#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    #[test]
    fn simple_checksum() {
        let input = "5 1 9 5
            7 5 3
            2 4 6 8";
        assert_eq!(min_max_difference_checksum(input), 18);
    }

    #[test]
    fn part_1() {
        let input = fs::read_to_string("inputs/day02.txt").unwrap();
        assert_eq!(min_max_difference_checksum(input.as_str()), 36766);
    }

    #[test]
    fn simple_division_checksum() {
        let input = "5 9 2 8
            9 4 7 3
            3 8 6 5";
        assert_eq!(division_checksum(input), 9);
    }

    #[test]
    fn part_2() {
        let input = fs::read_to_string("inputs/day02.txt").unwrap();
        assert_eq!(division_checksum(input.as_str()), 261);
    }
}