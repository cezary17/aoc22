use crate::line_reader::lines_from_file;
use std::collections::VecDeque;
use std::convert::TryFrom;
use std::convert::TryInto;
use nom::bytes::complete::tag;
use nom::character::is_digit;
use nom::IResult;
use nom::multi::separated_list1;

fn parse_input(path: &str) -> Vec<(String, String)> {
    let input = lines_from_file(path);
    let mut output = Vec::new();
    let mut curr_vec:Vec<String> = Vec::new();
    for line in input {
        if line == "" {
            let tuple:(String, String) = (curr_vec[0].clone(), curr_vec[1].clone());
            output.push(tuple);
            curr_vec = Vec::new();
        } else {
            curr_vec.push(line);
        }
    }
    output
}

fn parse_list(s: &str) -> IResult<&str, Vec<&str>> {
    separated_list1(tag(","), s)(s)
}

pub fn task1(path: &str) -> i32 {
    let input = parse_input(path);
    let output = 0;
    
    for (index, line) in input.iter().enumerate() {
        let res = false;
        
        let line0 = parse_list(&line.0);
        println!("{:?}", line0.unwrap().1);
        
        // let line_0 = line.0.clone().chars().filter(|c| c == ",").collect::<String>();
        // let mut line_0_chars: VecDeque<char> = line_0.chars().collect::<VecDeque<char>>();
        // line_0_chars.pop_front();
        // line_0_chars.pop_back();
        
        
        // let line_1 = line.1.clone().chars().filter(|c| c == ",").collect::<String>();
        // let mut line_1_chars: VecDeque<char> = line_1.chars().collect::<VecDeque<char>>();
        // line_1_chars.pop_front();
        // line_1_chars.pop_back();
        
        if res {let output = output + 1;}
        // println!("Line 0: {:?} Line 1: {:?}", line_0_chars, line_1_chars);
    }
    0
}