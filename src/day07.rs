use regex::Regex;
use std::collections::HashSet;

#[derive(PartialEq, Debug, Clone, Eq)]
struct Tower {
    name: String,
    weight: i32,
    sub_tower_names: HashSet<String>
}

fn get_towers_from_input(input: &str) -> Vec<Tower> {
    input.split("\n")
        .map(|line| { get_tower_from_line(line) })
        .collect::<Vec<_>>()
}

fn get_tower_from_line(line: &str) -> Tower {
    let regex = Regex::new("\\w+").unwrap();

    let matches = regex.find_iter(line).collect::<Vec<_>>();
    let name = matches[0].as_str().to_string();
    let weight = matches[1].as_str().parse::<i32>().unwrap();
    let sub_tower_names = matches.iter().skip(2).map(|m| { m.as_str().to_string() }).collect::<HashSet<_>>();

    Tower { name, weight, sub_tower_names }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_get_towers() {
        let input = "a (2)
            b (3)
            c (1) -> a, b";
        let towers = get_towers_from_input(input);
        let expected = vec![
            Tower { name: "a".to_string(), weight: 2, sub_tower_names: HashSet::new() },
            Tower { name: "b".to_string(), weight: 3, sub_tower_names: HashSet::new() },
            Tower { name: "c".to_string(), weight: 1, sub_tower_names: HashSet::from(["a".to_string(), "b".to_string()]) },
        ];
        assert_eq!(expected, towers);
    }

    #[test]
    fn regex_experimentation() {
        let actual = get_tower_from_line("c (1) -> a, b");
        let expected = Tower { name: "c".to_string(), weight: 1, sub_tower_names: HashSet::from(["a".to_string(), "b".to_string()]) };
        assert_eq!(expected, actual);
    }

    #[test]
    fn can_get_bottom_tower() {

    }
}