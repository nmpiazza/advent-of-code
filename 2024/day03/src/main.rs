use regex::Regex;

const PART_1_REGEX: &str = r"mul\([0-9]+,[0-9]+\)";
const PART_1_REGEX_WITH_CAPTURE_GROUP: &str = r"mul\((?P<left>[0-9]+),(?<right>[0-9]+)\)";
const PART_1_TEST_INPUT: &str =
    "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";


fn part_1(haystack: &str) -> i32 {
    let re = Regex::new(PART_1_REGEX).unwrap();
    // re.find_iter(haystack).map(|x| x.as_str()).collect()
    let mut matches = vec![];
    for (_, [left, right]) in re.captures_iter(haystack).map(|c| c.extract()) {
        matches.push((left, right));
    }
    
    matches.iter().map(|x| x.0 * x.1).sum()
}

fn main() {
    println!("Part 1 test input: {}", part_1(PART_1_TEST_INPUT));
}
