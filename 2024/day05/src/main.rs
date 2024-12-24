use std::fs::read_to_string;

const PART_1_TEST: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

enum AOCError {
    Part_1_Error,
}

fn part_1_main(input: &str) -> Result<i32, AOCError> {
    //split input sections to part 1 and part 2
    // parse input section 1 to rules
    // parse input section 2 to updates
    // check if each update is valid and denote valid v invalid
    // for each valid update, find middle page, sum
    // return sum

    Ok(0)
}
fn main() {
    println!("Part 1 Test: {}", part_1_main());
}
