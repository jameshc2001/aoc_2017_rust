
fn captcha_sum(input: &str) -> u32 {
    captcha_sum_for_interval(input, 1)
}

fn half_way_captcha_sum(input: &str) -> u32 {
    captcha_sum_for_interval(input, input.len() / 2)
}

fn captcha_sum_for_interval(input: &str, half_len: usize) -> u32 {
    let len = input.len();
    let chars: Vec<_> = input.chars().collect();

    chars.iter().enumerate().map(|(i, c)| {
        let next = chars[(i + half_len) % len];
        if c.eq(&next) { c.to_digit(10).unwrap() } else { 0 }
    }).sum()
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    #[test]
    fn simple_captcha_sums() {
        assert_eq!(captcha_sum("1122"), 3);
        assert_eq!(captcha_sum("1111"), 4);
        assert_eq!(captcha_sum("1234"), 0);
        assert_eq!(captcha_sum("91212129"), 9)
    }

    #[test]
    fn part_1() {
        let input = fs::read_to_string("inputs/day01.txt").unwrap();
        assert_eq!(captcha_sum(input.as_str()), 1175);
    }

    #[test]
    fn half_way_captcha_sums() {
        assert_eq!(half_way_captcha_sum("1212"), 6);
        assert_eq!(half_way_captcha_sum("1221"), 0);
        assert_eq!(half_way_captcha_sum("123425"), 4);
        assert_eq!(half_way_captcha_sum("123123"), 12);
        assert_eq!(half_way_captcha_sum("12131415"), 4);
    }

    #[test]
    fn part_2() {
        let input = fs::read_to_string("inputs/day01.txt").unwrap();
        assert_eq!(half_way_captcha_sum(input.as_str()), 1166);
    }
}