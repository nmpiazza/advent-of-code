use std::fs::read_to_string;

use regex::Regex;

const PART_1_REGEX: &str = r"mul\([0-9]+,[0-9]+\)";
const PART_1_REGEX_WITH_CAPTURE_GROUP: &str = r"mul\((?P<left>[0-9]+),(?<right>[0-9]+)\)";
const PART_1_TEST_INPUT: &str =
    "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
const PART1_FILENAME: &str = "src/input.txt";

fn part_1_core(haystack: &str) -> i32 {
    let re = Regex::new(PART_1_REGEX_WITH_CAPTURE_GROUP).unwrap();
    // re.find_iter(haystack).map(|x| x.as_str()).collect()
    let mut matches = vec![];
    for (_, [left, right]) in re.captures_iter(haystack).map(|c| c.extract()) {
        matches.push((left, right));
    }

    matches
        .iter()
        .map(|x| x.0.parse::<i32>().unwrap() * x.1.parse::<i32>().unwrap())
        .sum()
}

fn part_1_main(filename: &str) -> i32 {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .map(|haystack| part_1_core(haystack.as_str()))
        .sum()
}

fn main() {
    println!("Part 1 test input: {}", part_1_core(PART_1_TEST_INPUT));
    println!("Part 1 real input: {}", part_1_main(PART1_FILENAME));
}
