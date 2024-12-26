use std::fs::read_to_string;

const PART_1_PATH: &str = "./src/input.txt";
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

#[allow(dead_code)]
#[derive(Debug)]
enum AOCError {
    Part1Error,
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
struct Rule {
    left: usize,
    right: usize,
}
#[allow(dead_code)]
impl Rule {
    fn new(input: &str) -> Rule {
        let mut iter = input.splitn(2, '|');
        Rule {
            left: iter.next().unwrap().parse::<usize>().ok().unwrap(),
            right: iter.next().unwrap().parse::<usize>().ok().unwrap(),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
struct Update {
    //pages: Map<usize, String>,
    pages: Vec<usize>,
    rules: Vec<Rule>,
    valid: bool,
}

#[allow(dead_code)]
impl Update {
    fn new(input: &str, rules: &Vec<Rule>) -> Update {
        let pages: Vec<usize> = input
            .split(',')
            .map(|x| x.parse::<usize>().ok().unwrap())
            .into_iter()
            .collect();
        let mut update: Update = Update {
            pages: pages.clone(),
            rules: vec![],
            valid: false,
        };
        update.set_matching_rules(rules);
        // { pages: input .split(',') .map(|x| x.parse::<usize>().ok().unwrap()) .into_iter() .collect(), rules: vec![], };

        update
    }

    fn middle(&self) -> usize {
        self.pages[self.pages.len() / 2]
    }

    fn set_matching_rules(&mut self, rule_vec: &Vec<Rule>) {
        for rule in rule_vec {
            if self.pages.contains(&rule.left) && self.pages.contains(&rule.right) {
                self.rules.push(rule.clone())
            }
        }
    }
    fn is_valid(&mut self) -> bool {
        //self.valid = self.rules.iter().all(|x| self.pages.iter().position(predicate))
        let mut short_circuit: bool = false;
        for rule in self.rules.clone() {
            let l: usize = self
                .pages
                .clone()
                .iter()
                .position(|val| *val == rule.left)
                .expect("usize for left of rule in update");
            let r: usize = self
                .pages
                .clone()
                .iter()
                .position(|val| *val == rule.right)
                .expect("usize for right of rule in update");

            //println!("l: {} < r: {} == {}", l, r, l < r);
            if l > r {
                short_circuit = true;
                break;
            }
        }

        //println!("self.valid: {}", self.valid);
        //println!("short_circuit: {}", short_circuit);
        //println!( "self.valid: {} && short_circuit: {}  == {}", self.valid, short_circuit, !(self.valid & short_circuit));

        self.valid = !short_circuit;

        self.valid
    }
}

fn parse_and_process(input: &str) -> (Vec<Rule>, Vec<Update>) {}
#[allow(dead_code)]
fn part_1_main(input: &str) -> Result<usize, AOCError> {
    // split input sections to part 1 and part 2
    let mut raw_rules: Vec<&str> = vec![];
    let mut raw_updates: Vec<&str> = vec![];
    let mut update_flag: bool = false;

    for line in input.lines() {
        if line == "" {
            update_flag = true;
            continue;
        }
        if update_flag {
            raw_updates.push(line)
        } else {
            raw_rules.push(line)
        }
    }
    //dbg!(&raw_rules);
    //dbg!(&raw_updates);

    // parse input section 1 to rules
    let rules: Vec<Rule> = raw_rules.iter().map(|x| Rule::new(x)).collect();
    // parse input section 2 to updates
    let mut updates: Vec<Update> = raw_updates.iter().map(|x| Update::new(x, &rules)).collect();

    // dbg!(rules);
    // check if each update is valid and denote valid v invalid
    let mut sum: usize = 0;
    for update in updates.iter_mut() {
        if update.is_valid() {
            let middle: usize = update.middle();
            //println!( "List: {:?}, Valid: {}, Middle: {}", update.pages, update.valid, middle);
            sum = sum + middle;
        }
    }
    //dbg!(&updates);

    //updates[0].is_valid();
    //dbg!(&updates[0]);
    // dbg!(updates);

    // for each valid update, find middle page, sum
    // return sum

    Ok(sum)
}
fn main() {
    println!("Part 1 Test: {}", part_1_main(PART_1_TEST).unwrap());
    println!(
        "Part 1: {}",
        part_1_main(&read_to_string(PART_1_PATH).unwrap()).unwrap()
    );
}
