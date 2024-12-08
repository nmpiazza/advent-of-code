use std::fs::read_to_string;

pub fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

pub fn parse_line(line: String) -> Vec<i32> {
    line.split_whitespace()
        .map(|x| -> i32 { x.parse::<i32>().unwrap() })
        .collect()
}

pub fn naive_main(filename: &str) -> i32 {
    let input: Vec<String> = read_lines(filename);
    let mut count: i32 = 0;
    for line in input {
        let parsed_line: Vec<i32> = parse_line(line);
        let safe: bool = naive_is_safe(&parsed_line);
        if safe {
            count += 1
        }
    }

    count
}

fn naive_is_safe(input: &Vec<i32>) -> bool {
    let mut differences: Vec<i32> = vec![];

    //let mut adjacent_difference: i32 = 0;
    for (index, val) in input.iter().enumerate() {
        if index == 0 {
            continue;
        }
        differences.push(input[index - 1] - val);
    }
    //dbg!(&differences);

    (differences.iter().all(|&x| x > 0) || differences.iter().all(|&x| x < 0))
        && differences.iter().all(|&x| 1 <= x.abs() && x.abs() <= 3)
}

fn part_2(filename: &str) -> i32 {
    let input: Vec<String> = read_lines(filename);
    let mut count: i32 = 0;
    for line in input {
        let parsed_line: Vec<i32> = parse_line(line);
        let safe: bool = naive_is_safe(&parsed_line);
        if safe {
            count += 1
        }
        // Added logic block for part 2.
        // This is a very simple and not elegant solution
        else {
            for (index, value) in parsed_line.iter().enumerate() {
                let mut tmp_vec: Vec<i32> = parsed_line.clone();
                tmp_vec.remove(index);
                if naive_is_safe(&tmp_vec) {
                    count += 1;
                    break;
                }
            }
        }
    }

    count
}
fn main() {
    println!(
        "test_input total safe: {}",
        naive_main("src/part1_test_input.txt")
    );
    println!(
        "part_1 input total safe: {}",
        naive_main("src/part1_input.txt")
    );
    println!(
        "part_2 test_input total safe: {}",
        part_2("src/part1_test_input.txt")
    );
    println!("part_1 input total safe: {}", part_2("src/part1_input.txt"));
}
