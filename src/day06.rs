use std::collections::{HashMap, HashSet};

fn get_memory_banks(input: &str) -> Vec<u32> {
    input.split_whitespace()
        .map(|num| { num.parse::<u32>().unwrap() })
        .collect::<Vec<u32>>()
}

fn index_of_highest(memory_banks: &Vec<u32>) -> usize {
    assert!(memory_banks.len() > 0);
    let mut highest = 0;
    let mut index_of_highest = 0;

    for (i, b) in memory_banks.iter().enumerate() {
        if b > &highest {
            highest = *b;
            index_of_highest = i;
        }
    }

    index_of_highest
}


fn redistribute(memory_banks: &Vec<u32>) -> Vec<u32> {
    let banks = memory_banks.len();
    let redistribute_bank = index_of_highest(&memory_banks);
    let redistribute_blocks = memory_banks.get(redistribute_bank).unwrap();

    let mut new_memory_banks = memory_banks.clone();
    new_memory_banks[redistribute_bank] = 0;

    let mut current_bank = redistribute_bank;
    for _ in 0..*redistribute_blocks {
        current_bank = (current_bank + 1) % banks;
        new_memory_banks[current_bank] += 1;
    }

    new_memory_banks
}


fn redistributions_to_duplicate(input: &str) -> u32 {
    let mut memory_banks = get_memory_banks(input);
    let mut configurations: HashSet<Vec<u32>> = HashSet::new();
    let mut redistributions = 0;
    configurations.insert(memory_banks.clone());

    loop {
        memory_banks = redistribute(&memory_banks);
        redistributions += 1;
        if configurations.contains(&memory_banks) { break }
        else { configurations.insert(memory_banks.clone()); }
    }

    redistributions
}

fn length_of_cycle(input: &str) -> u32 {
    let mut memory_banks = get_memory_banks(input);
    let mut configurations: HashMap<Vec<u32>, u32> = HashMap::new();
    let mut redistributions = 0;
    configurations.insert(memory_banks.clone(), redistributions);

    loop {
        memory_banks = redistribute(&memory_banks);
        redistributions += 1;
        if configurations.contains_key(&memory_banks) { break }
        else { configurations.insert(memory_banks.clone(), redistributions); }
    }

    redistributions - *configurations.get(&memory_banks).unwrap()
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn can_get_memory_banks() {
        let input = "0 2 7 0";
        let expected = vec![0, 2, 7, 0];
        assert_eq!(expected, get_memory_banks(input))
    }

    #[test]
    fn can_find_index_of_highest() {
        let memory_banks = vec![1, 2, 3, 3, 2];
        assert_eq!(2, index_of_highest(&memory_banks))
    }

    #[test]
    fn can_redistribute() {
        let start = vec![0, 2, 7, 0];
        let expected = vec![2, 4, 1, 2];
        assert_eq!(expected, redistribute(&start))
    }

    #[test]
    fn can_get_redistributions_to_duplicate() {
        let input = "0 2 7 0";
        assert_eq!(5, redistributions_to_duplicate(input))
    }

    #[test]
    fn part_1() {
        let input = fs::read_to_string("inputs/day06.txt").unwrap();
        assert_eq!(6681, redistributions_to_duplicate(input.as_str()))
    }

    #[test]
    fn can_get_length_of_cycle() {
        let input = "0 2 7 0";
        assert_eq!(4, length_of_cycle(input))
    }

    #[test]
    fn part_2() {
        let input = fs::read_to_string("inputs/day06.txt").unwrap();
        assert_eq!(2392, length_of_cycle(input.as_str()))
    }
}