use std::{collections::HashMap, fs::read_to_string};

const INPUT_FILENAME: &str = "src/part1_input.txt";

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn naive(lines: Vec<String>) {
    let (left, right): (Vec<i32>, Vec<i32>) = lines
        .into_iter()
        .map(|line| {
            let parts: Vec<String> = line.split_whitespace().map(String::from).collect();
            (
                parts[0].parse::<i32>().unwrap(),
                parts[1].parse::<i32>().unwrap(),
            )
        })
        .unzip();

    let mut frequency_map: HashMap<i32, (i32, i32)> = HashMap::new();

    for l_val in left.iter() {
        // Increment left frequency, reinsert precalculated right frequency
        if let Some((l_freq, r_freq)) = frequency_map.get(l_val) {
            frequency_map.insert(*l_val, (l_freq + 1, *r_freq));
        } else {
            // Calculate right frequency on first run
            let mut frequency: i32 = 0;
            for r_val in right.iter() {
                if l_val == r_val {
                    frequency = frequency + 1;
                }
            }
            // This will only be run on the first time, so left frequency is always 1 and the right
            // frequency is the calculated value
            frequency_map.insert(*l_val, (1, frequency));
        }
    }

    let mut similarity: i32 = 0;
    for (l_val, (l_freq, r_freq)) in frequency_map.iter() {
        similarity = similarity + (l_val * l_freq * r_freq);
    }
    println!("{}", similarity);
}

fn main() {
    let test_input: Vec<String> = vec!["3   4", "4   3", "2   5", "1   3", "3   9", "3   3"]
        .iter()
        .map(|x| x.to_string())
        .collect();
    let lines: Vec<String> = read_lines(INPUT_FILENAME);
    naive(test_input);
    naive(lines);
}
