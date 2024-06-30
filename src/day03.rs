use std::collections::HashMap;

use crate::shared::coord_2d::{ALL_DIRECTIONS, Coord2D, DOWN, LEFT, ORIGIN, RIGHT, UP};

fn distance_to_access_port(target_num: i32) -> i32 {
    let ordered_directions = vec![&RIGHT, &UP, &LEFT, &DOWN];
    let mut current_num = 1;
    let mut current_location = ORIGIN;
    let mut step_size = 0;

    while current_num != target_num {
        for direction in &ordered_directions {
            if *direction == &RIGHT || *direction == &LEFT { step_size += 1; }
            if step_size + current_num >= target_num { step_size = target_num - current_num; }

            current_location = current_location + &(*direction * step_size);
            current_num += step_size;

            if current_num == target_num { break }
        }
    }

    return current_location.manhattan_distance(&ORIGIN);
}

fn next_largest(target_num: i32) -> i32 {
    let ordered_directions = vec![&RIGHT, &UP, &LEFT, &DOWN];
    let mut current_location = ORIGIN;
    let mut step_size = 0;
    let mut grid = HashMap::new();
    grid.insert(current_location, 1);

    loop {
        for direction in &ordered_directions {
            if *direction == &RIGHT || *direction == &LEFT { step_size += 1; }

            for _ in 0..step_size {
                current_location = current_location + direction;
                let new_num = get_num_from_neighbours(&grid, &current_location);
                if new_num > target_num { return new_num }
                grid.insert(current_location, new_num);
            }
        }
    }
}

fn get_num_from_neighbours(grid: &HashMap<Coord2D, i32>, location: &Coord2D) -> i32 {
    ALL_DIRECTIONS.iter().map(|direction| {
        grid.get(&(location + direction)).unwrap_or(&0)
    }).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn easy_distances() {
        assert_eq!(distance_to_access_port(1), 0);
        assert_eq!(distance_to_access_port(12), 3);
        assert_eq!(distance_to_access_port(23), 2);
        assert_eq!(distance_to_access_port(1024), 31);
    }

    #[test]
    fn part_1() {
        assert_eq!(distance_to_access_port(265149), 438);
    }

    #[test]
    fn part_2() {
        assert_eq!(next_largest(265149), 266330);
    }
}