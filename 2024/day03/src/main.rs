use std::fs::read_to_string;

use regex::Regex;

const PART_1_REGEX: &str = r"mul\([0-9]+,[0-9]+\)";
const PART_1_REGEX_WITH_CAPTURE_GROUP: &str = r"mul\((?P<left>[0-9]+),(?<right>[0-9]+)\)";
const PART_1_TEST_INPUT: &str =
    "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
const PART_2_TEST_INPUT: &str =
    "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

const NEO_PART_2_REGEX: &str = r"(don't\(\)|do\(\)|mul\(\d+,\d+\))";

const PART1_FILENAME: &str = "src/input.txt";

fn part_1_core(haystack: &str) -> i32 {
    let re = Regex::new(PART_1_REGEX_WITH_CAPTURE_GROUP).unwrap();
    // re.find_iter(haystack).map(|x| x.as_str()).collect()
    let mut matches = vec![];
    for (_, [left, right]) in re.captures_iter(haystack).map(|c| c.extract()) {
        matches.push((left, right));
    }
    dbg!(&matches);
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

fn part_2_core(haystack: &str) -> i32 {
    /*
     Split string on "don't()"
     loop through items
         if first item
             pass to calculator
         split on "do()"
             If second half exists

    */

    let haystack_items: Vec<&str> = haystack.split("don't()").collect();
    //dbg!(&haystack_items);
    let mut accumulator: i32 = 0;

    for (pos, val) in haystack_items.iter().enumerate() {
        //println!("{}", val);
        if pos == 0 {
            accumulator = accumulator + part_1_core(val);
        } else {
            let substr: Vec<&str> = val.split("do()").collect();
            //dbg!(&substr);
            // TODO There may be multiple do()'s in a row, and this misses that case
            // TODO Need to calculate for each segment
            if substr.len() == 2 {
                //accumulator = accumulator + part_1_core(substr[1])
                //accumulator = accumulator + part_1_core(substr[1..].join("").clone().as_str())
                for element in &substr[1..] {
                    accumulator = accumulator + part_1_core(element)
                }
            }
        }
    }
    accumulator
}

fn part_2_main(filename: &str) -> i32 {
    // TODO: Join all of the lines into a single line
    part_2_core(
        &read_to_string(filename).unwrap(), //.lines()
                                            //.map(String::from)
                                            //.collect::<Vec<String>>()
                                            //.join(""),
    )

    //.map(|haystack| part_2_core(haystack.as_str()))
    //.sum()
}

fn neo_part_2_core(haystack: &str) -> i32 {
    let re = Regex::new(NEO_PART_2_REGEX).unwrap();
    let mut instructions = vec![];
    instructions = re.find_iter(haystack).map(|x| x.as_str()).collect();

    let mut should_mul: bool = true;
    let mut accumulator: i32 = 0;
    dbg!(&instructions);
    let mut final_instructions: Vec<(String, bool)> = vec![];
    for instruction in instructions.iter() {
        if *instruction == "don't()" {
            should_mul = false;
            println!("{}, {}", *instruction, should_mul);
            continue;
        }
        if *instruction == "do()" {
            should_mul = true;
            println!("{}, {}", *instruction, should_mul);
            continue;
        }
        println!("{}, {}", *instruction, should_mul);
        if should_mul {
            accumulator = accumulator + part_1_core(instruction)
        }
    }
    accumulator
}
fn neo_part_2_main(filename: &str) -> i32 {
    neo_part_2_core(&read_to_string(filename).unwrap())
}
fn main() {
    println!("Part 1 test input: {}", part_1_core(PART_1_TEST_INPUT));
    println!("Part 1 real input: {}", part_1_main(PART1_FILENAME));
    println!("Part 2 test input: {}", part_2_core(PART_2_TEST_INPUT));
    println!("Part 2 real input: {}", part_2_main(PART1_FILENAME));
    println!(
        "New Part 2 test input: {}",
        neo_part_2_core(PART_2_TEST_INPUT)
    );
    println!("New Part 2 real input: {}", neo_part_2_main(PART1_FILENAME));
}
