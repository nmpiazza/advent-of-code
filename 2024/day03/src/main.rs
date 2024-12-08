use regex::Regex;

const PART_1_REGEX: &str = r"mul\([0-9]+,[0-9]+\)";
const PART_1_REGEX_WITH_CAPTURE_GROUP: &str = r"mul\((?P<LEFT>[0-9]+),(?<RIGHT>[0-9]+)\)";
const PART_1_TEST_INPUT: &str =
    "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

fn find_matches(haystack: &str) -> Vec<&str> {
    let re = Regex::new(PART_1_REGEX).unwrap();
    re.find_iter(haystack).map(|x| x.as_str()).collect()
}

fn main() {
    dbg!(find_matches(PART_1_TEST_INPUT));
}
