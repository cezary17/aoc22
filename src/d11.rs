use std::ops::Deref;
use crate::line_reader::lines_from_file;

#[derive(Debug)]
pub struct Monkey {
    items: Vec<u64>,
    operation: String,
    test: i32,
    throw_to: [i32;2],
    monkey_tag: i32,
}

// implement default for Monkey
impl Default for Monkey {
    fn default() -> Self {
        Monkey {
            items: vec![],
            operation: "".to_string(),
            test: 0,
            throw_to: [0,0],
            monkey_tag: 0,
        }
    }
}

#[allow(dead_code)]
pub fn generator(path: &str) -> Vec<Monkey> {
    let input = lines_from_file(path);

    let mut parsed_input: Vec<Vec<String>> = Vec::new();
    let mut curr_monkey_vec:Vec<String> = Vec::new();
    
    for line in input {
        let line = line.trim();
        let line_vec:Vec<&str> = line.split_whitespace().collect();
        if line_vec.len() == 0 {
            parsed_input.push(curr_monkey_vec);
            curr_monkey_vec = Vec::new();
        } else {
            curr_monkey_vec.push(line.to_string());
        }
    }
    parsed_input.push(curr_monkey_vec);
    
    let mut monkeys: Vec<Monkey> = Vec::new();
    for _ in &parsed_input {
        let monkey_obj = Monkey::default();
        monkeys.push(monkey_obj);
    }

    for monkey_input_vec in parsed_input {
        let mut curr_monkey_index = 0;
        for line in monkey_input_vec {
            let mut line_vec: Vec<&str> = line.split_whitespace().collect();
            line_vec[0] = line_vec[0].trim_end_matches(":");
            match line_vec[0] {
                "Monkey" => {
                    // strip : from the second element
                    let monkey_index = line_vec[1].trim_end_matches(":").parse::<usize>().unwrap();
                    curr_monkey_index = monkey_index;
                    monkeys[curr_monkey_index].monkey_tag = monkey_index as i32;
                },
                "Starting" => {
                    let items: Vec<u64> = line_vec[2..].iter().map(|x| x.trim_end_matches(",").parse::<u64>().unwrap()).collect();
                    monkeys[curr_monkey_index].items = items;
                },
                "Operation" => {
                    let mut rev = line_vec.iter().rev();
                    let operand = rev.next().unwrap().deref().clone();
                    let op = rev.next().unwrap().deref().clone();
                    match operand {
                        "old" => {
                            let formatted = format!("{} s", op);
                            monkeys[curr_monkey_index].operation = formatted;
                        }
                        _ => {
                            let formatted = format!("{} {}", op, operand);
                            monkeys[curr_monkey_index].operation = formatted;
                        }
                    }
                },
                "Test" => {
                    let mut rev = line_vec.iter().rev();
                    let value = rev.next().unwrap().deref();
                    let value = value.parse::<i32>().unwrap();
                    monkeys[curr_monkey_index].test = value;
                },
                "If" => {
                    let bool = line_vec[1].trim_end_matches(":");
                    let mut rev = line_vec.iter().rev();
                    let value = rev.next().unwrap().deref().parse::<i32>().unwrap();
                    match bool { 
                        "false" => monkeys[curr_monkey_index].throw_to[0] = value,
                        "true" => monkeys[curr_monkey_index].throw_to[1] = value,
                        _ => panic!("Bruh Moment (If match)"),
                    }
                },
                _ => {},
            }
        }
    }
    monkeys
}

#[allow(dead_code)]
pub fn task1(path: &str) -> u64 {
    let mut monkeys = generator(path);
    
    let mut touching_monkeys:Vec<u64> = vec![0; monkeys.len()];
    let mut changes_stacks = vec![vec![]; monkeys.len()];
    
    for _round in 1..=20 {
        for monkey in &mut monkeys {
            
            if changes_stacks[monkey.monkey_tag as usize].len() > 0 {
                let changes = changes_stacks[monkey.monkey_tag as usize].clone();
                changes_stacks[monkey.monkey_tag as usize] = vec![];
                for change in changes {
                    monkey.items.push(change);
                }
            }
            
            for item in &monkey.items {
                touching_monkeys[monkey.monkey_tag as usize] += 1;
                let mut worry_level = 0;
                let operation_vec = monkey.operation.split_whitespace().collect::<Vec<&str>>();
                match operation_vec[0] {
                    "+" => {
                        if operation_vec[1] == "s" {
                            worry_level = *item * 2;
                        } else {
                            worry_level += *item + operation_vec[1].parse::<u64>().unwrap();
                        }
                    }
                    "*" => {
                        if operation_vec[1] == "s" {
                            worry_level = item.pow(2);
                        } else {
                            worry_level += *item * operation_vec[1].parse::<u64>().unwrap();
                        }
                    }
                    _ => unreachable!("Bruh Moment (Operation match)"),
                }
                
                worry_level = worry_level / 3;
                
                if worry_level % (monkey.test as u64) == 0 {
                    changes_stacks[monkey.throw_to[1] as usize].push(worry_level);
                } else {
                    changes_stacks[monkey.throw_to[0] as usize].push(worry_level);
                }
            }
            monkey.items.clear();
        }
    }
    touching_monkeys.sort_by(|a, b| b.cmp(a));
    println!("Two most active monkeys: {}, {}", touching_monkeys[0], touching_monkeys[1]);
    let result = touching_monkeys[0] * touching_monkeys[1];
    println!("Result: {}", result);
    result
}

#[allow(dead_code)]
fn find_gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        find_gcd(b, a % b)
    }
}

#[allow(dead_code)]
fn find_lcm(a: u64, b: u64) -> u64 {
    (a * b) / find_gcd(a, b)
}

#[allow(dead_code)]
pub fn task2(path: &str) -> u64 {
    let mut monkeys = generator(path);

    let mut touching_monkeys:Vec<u64> = vec![0; monkeys.len()];
    let mut changes_stacks = vec![vec![]; monkeys.len()];
    
    let divisors = monkeys.iter().map(|x| x.test).collect::<Vec<i32>>();
    
    let lcm = divisors.iter().fold(1, |acc, x| find_lcm(acc, *x as u64));
    
    for _round in 1..=10_000 {
        for monkey in &mut monkeys {

            if changes_stacks[monkey.monkey_tag as usize].len() > 0 {
                let changes = changes_stacks[monkey.monkey_tag as usize].clone();
                changes_stacks[monkey.monkey_tag as usize] = vec![];
                for change in changes {
                    monkey.items.push(change);
                }
            }

            for item in &monkey.items {
                touching_monkeys[monkey.monkey_tag as usize] += 1;
                let mut worry_level = 0;
                let operation_vec = monkey.operation.split_whitespace().collect::<Vec<&str>>();
                match operation_vec[0] {
                    "+" => {
                        if operation_vec[1] == "s" {
                            worry_level = *item * 2;
                        } else {
                            worry_level += *item + operation_vec[1].parse::<u64>().unwrap();
                        }
                    }
                    "*" => {
                        if operation_vec[1] == "s" {
                            worry_level = item.pow(2);
                        } else {
                            worry_level += *item * operation_vec[1].parse::<u64>().unwrap();
                        }
                    }
                    _ => unreachable!("Bruh Moment (Operation match)"),
                }

                worry_level = worry_level % lcm;

                if worry_level % (monkey.test as u64) == 0 {
                    changes_stacks[monkey.throw_to[1] as usize].push(worry_level);
                } else {
                    changes_stacks[monkey.throw_to[0] as usize].push(worry_level);
                }
            }
            monkey.items.clear();
        }
    }
    touching_monkeys.sort_by(|a, b| b.cmp(a));
    println!("Two most active monkeys: {}, {}", touching_monkeys[0], touching_monkeys[1]);
    let result = touching_monkeys[0] * touching_monkeys[1];
    println!("Result: {}", result);
    result
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]
    fn test_lcm() {
        let vec = vec![2,3,4,5,7];
        let mut lcm = vec.iter().fold(1, |acc, x| find_lcm(acc, *x as u64));
        assert_eq!(lcm, 420);
    }
    
    #[test]
    fn test_task1() {
        let res = task1("input_test/day11_test.txt");
        assert_eq!(res, 10605);
    }
    
    #[test]
    fn test_task2() {
        let res = task2("input_test/day11_test.txt");
        assert_eq!(res, 2713310158);
    }
}