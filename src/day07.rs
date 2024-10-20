use regex::Regex;
use std::collections::HashSet;
use once_cell::sync::Lazy;

static REGEX: Lazy<Regex> = Lazy::new(|| { Regex::new("\\w+").unwrap() });

#[derive(PartialEq, Debug, Clone, Eq)]
struct Tower {
    name: String,
    weight: i32,
    sub_tower_names: HashSet<String>
}

fn get_towers_from_input(input: &str) -> Vec<Tower> {
    let regex = Regex::new("\\w+").unwrap();
    input.split("\n")
        .map(|line| { get_tower_from_line(line) })
        .collect::<Vec<_>>()
}

fn get_tower_from_line(line: &str) -> Tower {
    let matches = REGEX.find_iter(line).collect::<Vec<_>>();
    let name = matches[0].as_str().to_string();
    let weight = matches[1].as_str().parse::<i32>().unwrap();
    let sub_tower_names = matches.iter().skip(2).map(|m| { m.as_str().to_string() }).collect::<HashSet<_>>();

    Tower { name, weight, sub_tower_names }
}

fn get_bottom_tower(input: &str) -> String {
    let towers = get_towers_from_input(input);

    let sub_towers = towers.iter()
        .flat_map(|t| { &t.sub_tower_names })
        .collect::<HashSet<_>>();
    let all_towers = towers.iter()
        .map(|t| { &t.name })
        .collect::<HashSet<_>>();

    all_towers.difference(&sub_towers).next().unwrap().to_string()
}



#[cfg(test)]
mod tests {
    use std::fs;
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
        let input = "pbga (66)
            xhth (57)
            ebii (61)
            havc (66)
            ktlj (57)
            fwft (72) -> ktlj, cntj, xhth
            qoyq (66)
            padx (45) -> pbga, havc, qoyq
            tknk (41) -> ugml, padx, fwft
            jptl (61)
            ugml (68) -> gyxo, ebii, jptl
            gyxo (61)
            cntj (57)";
        assert_eq!("tknk", get_bottom_tower(input));
    }

    #[test]
    fn can_get_answer_for_part_1() {
        let input = fs::read_to_string("inputs/day07.txt").unwrap();
        assert_eq!("azqje", get_bottom_tower(&input));
    }
}