use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::{HashMap, HashSet};

static REGEX: Lazy<Regex> = Lazy::new(|| { Regex::new("\\w+").unwrap() });

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
    let matches = REGEX.find_iter(line).collect::<Vec<_>>();
    let name = matches[0].as_str().to_string();
    let weight = matches[1].as_str().parse::<i32>().unwrap();
    let sub_tower_names = matches.iter().skip(2).map(|m| { m.as_str().to_string() }).collect::<HashSet<_>>();

    Tower { name, weight, sub_tower_names }
}

fn get_bottom_tower(towers: &Vec<Tower>) -> String {
    let sub_towers = towers.iter()
        .flat_map(|t| { &t.sub_tower_names })
        .collect::<HashSet<_>>();
    let all_towers = towers.iter()
        .map(|t| { &t.name })
        .collect::<HashSet<_>>();

    all_towers.difference(&sub_towers).next().unwrap().to_string()
}

fn get_bottom_tower_from_input(input: &str) -> String {
    get_bottom_tower(&get_towers_from_input(input))
}

fn get_total_weights_for_towers(towers: &Vec<Tower>) -> HashMap<String, i32> {
    let start = get_bottom_tower(&towers);
    let towers: HashMap<&String, &Tower> = towers.iter().map(|t| (&t.name, t)).collect();
    let weights = &mut HashMap::new();
    get_total_weight(&towers, &start, weights);
    weights.clone()
}

fn get_total_weight(towers: &HashMap<&String, &Tower>, current_tower: &String, weights: &mut HashMap<String, i32>) -> i32 {
    let current_tower = towers.get(&current_tower).unwrap();
    let tower_weight = current_tower.weight;
    let sub_tower_weight: i32 = current_tower.sub_tower_names.iter().map(|t| get_total_weight(towers, t, weights)).sum();

    let total_weight = tower_weight + sub_tower_weight;
    weights.insert(current_tower.name.clone(), total_weight);
    total_weight
}

#[derive(PartialEq, Debug, Clone, Eq)]
struct TowerAndTotal {
    tower: Tower,
    total_weight: i32
}

fn get_correct_weight_for_bad_tower(input: &str) -> i32 {
    let towers = get_towers_from_input(input);
    let tower_weights = get_total_weights_for_towers(&towers);
    let towers: HashMap<&String, &Tower> = towers.iter().map(|t| (&t.name, t)).collect();

    let answers: &mut Vec<i32> = &mut Vec::new();

    for (_, t) in &towers {
        if t.sub_tower_names.len() < 3 { continue }

        let mut sub_towers: Vec<TowerAndTotal> = t.sub_tower_names.iter()
            .map(|t| { TowerAndTotal {
                tower: (*towers.get(t).unwrap()).clone(),
                total_weight: *tower_weights.get(t).unwrap() }
            })
            .collect();
        sub_towers.sort_by_key(|t| t.total_weight);

        if sub_towers[0].total_weight != sub_towers[1].total_weight {
            let diff = sub_towers[1].total_weight - sub_towers[0].total_weight;
            answers.push(sub_towers[0].tower.weight + diff);
        }

        if sub_towers[sub_towers.len() - 1].total_weight != sub_towers[sub_towers.len() - 2].total_weight {
            let diff = sub_towers[sub_towers.len() - 2].total_weight - sub_towers[sub_towers.len() - 1].total_weight;
            answers.push(sub_towers[sub_towers.len() - 1].tower.weight + diff);
        }
    }

    *answers.iter().min().unwrap() //bad
}

//part 2: recursive function for creating a graph (tree) that has the total weight at each tower.
//starting at the base tower, look at the sub_towers and choose the one with the different weight
//go to this tower and repeat until no differing weight found. This means you are at the 'bad' tower.
//consider the tower before this one to determine the correct weight


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    const SAMPLE_INPUT: &str = "pbga (66)
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
        assert_eq!("tknk", get_bottom_tower_from_input(SAMPLE_INPUT));
    }

    #[test]
    fn can_get_answer_for_part_1() {
        let input = fs::read_to_string("inputs/day07.txt").unwrap();
        assert_eq!("azqje", get_bottom_tower_from_input(&input));
    }

    #[test]
    fn can_get_total_weights_for_sample_input() {
        let map = get_total_weights_for_towers(&get_towers_from_input(SAMPLE_INPUT));
        assert_eq!(778, *map.get("tknk").unwrap());
        assert_eq!(251, *map.get("ugml").unwrap());
        assert_eq!(243, *map.get("fwft").unwrap());
        assert_eq!(243, *map.get("padx").unwrap());
        assert_eq!(57, *map.get("xhth").unwrap());
    }

    #[test]
    fn can_get_weight_for_bad_tower() {
        assert_eq!(60, get_correct_weight_for_bad_tower(SAMPLE_INPUT));
    }

    #[test]
    fn can_get_answer_for_part_2() {
        let input = fs::read_to_string("inputs/day07.txt").unwrap();
        assert_eq!(646, get_correct_weight_for_bad_tower(&input));
    }
}