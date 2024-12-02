use std::fs::read_to_string;

const INPUT_FILENAME: &str = "src/part1_input.txt";

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

#[allow(dead_code)]
fn naive() {
    let lines: Vec<String> = read_lines(INPUT_FILENAME);

    let mut left: Vec<String> = vec![];
    let mut right: Vec<String> = vec![];
    for line in lines {
        let parts: Vec<String> = line.split_whitespace().map(String::from).collect();
        left.push(parts[0].clone());
        right.push(parts[1].clone());
    }

    assert_eq!(left.len(), right.len());

    left.sort();
    right.sort();

    // iterate through lists, compute distance, take absolute value, add to sum
    let mut sum: i32 = 0;
    for elements in left.iter().zip(right.iter()) {
        let (l_element, r_element) = elements;
        let l: i32 = l_element.to_string().parse::<i32>().unwrap();
        let r: i32 = r_element.to_string().parse::<i32>().unwrap();
        let val = (l - r).abs();
        sum = sum + val;
    }
    println!("{}", sum)
}

fn clean() {
    let lines: Vec<String> = read_lines(INPUT_FILENAME);

    let (mut left, mut right): (Vec<String>, Vec<String>) = lines
        .into_iter()
        .map(|line| {
            let parts: Vec<String> = line.split_whitespace().map(String::from).collect();
            (parts[0].to_string(), parts[1].to_string())
        })
        .unzip();

    left.sort();
    right.sort();

    let sum: i32 = left
        .iter()
        .zip(right.iter())
        .map(|(l, r)| {
            (l.to_string().parse::<i32>().unwrap() - r.to_string().parse::<i32>().unwrap()).abs()
        })
        .sum();

    println!("{}", sum);
}

fn main() {
    //naive();
    clean();
}
