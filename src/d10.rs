use crate::line_reader::lines_from_file;
use std::collections::VecDeque;

struct register {
    register_x: i32,
    curr_index: i32,
}

impl Default for register {
    fn default() -> Self {
        register {
            register_x: 1,
            curr_index: 0,
        }
    }
}

fn parse_instructions(input: &Vec<String>) -> VecDeque<i32> {
    // splitting the input lines
    let input: Vec<Vec<&str>> = input.iter().map(|x| x.split_whitespace().collect()).collect();
    // queue for the operations
    let mut opqueue = VecDeque::new();
    
    for line in input {
        let op: &str = line[0];
        let arg: Option<i32> = line.get(1).map(|x| x.parse::<i32>().unwrap());
        match op {
            "noop" => opqueue.push_back(0),
            "addx" => {
                opqueue.push_back(0);
                opqueue.push_back(arg.unwrap());
            }
            _ => unreachable!("bruh moment: {}", op),
        }
    }
    opqueue
}

pub fn task1(path: &str) -> i32 {
    let input: Vec<String> = lines_from_file(path);
    
    let opqueue = parse_instructions(&input);
    
    let mut register = register::default(); 
    
    let mut signal_strengths: Vec<i32> = Vec::new();
    
    for (i, val) in opqueue.iter().enumerate() {
        let i = (i + 1) as i32;
        if (i - 20) % 40 == 0 {
            signal_strengths.push(register.register_x * i);
        }
        register.register_x += val;
    }
    let output: i32 = signal_strengths.iter().sum::<i32>();
    println!("output: {}", output);
    output
}

pub fn task2(path: &str){
    let input = lines_from_file(path);
    
    let opqueue = parse_instructions(&input);
    
    let mut cpu = register::default();
    
    let mut output: Vec<String> = Vec::new();
    let mut buf = Vec::new();
    
    for val in opqueue {
        if cpu.register_x - 1 <= cpu.curr_index && cpu.curr_index <= cpu.register_x + 1 {
            buf.push("#")
        }
        else {
            buf.push(".")
        }
        
        cpu.curr_index += 1;
        cpu.register_x += val;
        
        if cpu.curr_index % 40 == 0 {
            output.push(buf.join(""));
            buf.clear();
            cpu.curr_index = 0;
        }
    }
    for line in &output {
        println!("{}", line);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        assert_eq!(task1("src/input_sources/day10_test.txt"), 13);
    }
}