mod day_1;
mod day_2;

use std::env;
use std::fs;

macro_rules! test_file {
    ($fname:expr) => {
        concat!(env!("CARGO_MANIFEST_DIR"), "/resources/", $fname) // assumes Linux ('/')!
    };
}

macro_rules! load_file {
    ($fname:expr) => {
        fs::read_to_string(test_file!($fname)).expect("Something went wrong reading the file")
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::day_1::*;

    #[test]
    fn advent_day1_calories() {
        let input = load_file!("day1_calories.txt");
        let calories = find_elf_with_most_calories(&input);
        assert_eq!(calories, 66186);
    }

    #[test]
    fn advent_day1_largest_3() {
        let input = load_file!("day1_calories.txt");
        let calories = find_top_n_elves_total_calories(&input, 3);
        assert_eq!(calories, 196804);
    }

    use crate::day_2::*;

    #[test]
    fn advent_day2_part1() {
        let input = load_file!("day2_strategy.txt");
        let result = calculate_rps_strategy_guide_score(&input);
        assert_eq!(result, 11449);
    }

    #[test]
    fn advent_day2_part2() {
        //
    }
}
