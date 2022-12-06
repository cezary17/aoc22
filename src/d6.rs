use crate::line_reader::lines_from_file;
use std::collections::HashSet;

pub fn task1() {
    let path = "src/input_sources/day6_input.txt";
    let mut input = lines_from_file(path);
    let input: Vec<char> = input.pop().unwrap().chars().collect::<Vec<char>>();
    
    let winsize = 4;
    
    let mut window  = input.windows(winsize);

    let mut res: i32 = 0;
    loop {
        let curr = window.next().unwrap();
        let curr_set: HashSet<&char> = HashSet::from_iter(curr.clone());
        if curr.len() == curr_set.len() {
            res += winsize as i32;
            break
        } else { 
            res += 1;
        }
    }
    println!("{res}")   
}

pub fn task2() {
    let path = "src/input_sources/day6_input.txt";
    let mut input = lines_from_file(path);
    let input: Vec<char> = input.pop().unwrap().chars().collect::<Vec<char>>();

    let winsize = 14;

    let mut window  = input.windows(winsize);

    let mut res: i32 = 0;
    loop {
        let curr = window.next().unwrap();
        let curr_set: HashSet<&char> = HashSet::from_iter(curr.clone());
        if curr.len() == curr_set.len() {
            res += winsize as i32;
            break
        } else {
            res += 1;
        }
    }
    println!("{res}")
}