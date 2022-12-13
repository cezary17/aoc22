use crate::line_reader::lines_from_file;
use std::collections::VecDeque;
use std::fs::File;
use std::io::Write;
use itertools::Itertools;

#[allow(dead_code)]
fn parse_input(path: &str, task: i32) -> (Vec<Vec<u8>>, Vec<(usize, usize)>, (usize, usize)) {
    let input = lines_from_file(path);
    let input: Vec<Vec<u8>> = input.iter().map(|x| x.as_bytes().to_vec()).collect();
    
    let mut start = vec![];
    if task == 1{
        // find the value 'S' in the input
        start = vec![(0..input.len()).cartesian_product(0..input[0].len()).find(|&(x,y)| input[x][y] == b'S').unwrap()];
    } else { 
        // find all values 'a' and the value 'S' in input and store them in a vector
        start = (0..input.len()).cartesian_product(0..input[0].len()).filter(|&(x,y)| (input[x][y] == b'a' || input[x][y] == b'S')).collect();
    }
    
    let end = (0..input.len()).cartesian_product(0..input[0].len()).find(|&(x,y)| input[x][y] == b'E').unwrap();
    
    let input:Vec<Vec<u8>> = input.iter().map(|x| x.iter().map(|y| if *y == b'S' {b'a'} else if *y == b'E' {b'z'} else {*y}).collect()).collect();
    
    (input, start, end)
}

fn bfs(input: &[Vec<u8>], start: &[(usize, usize)], end: (usize, usize)) -> Option<usize> {
    
    let mut visited = vec![vec![false; input[0].len()]; input.len()];
    let mut queue = start.iter().map(|&(x,y)| (x, y, 0)).collect::<VecDeque<_>>();
    
    while let Some((x, y, len)) = queue.pop_front() {
        if (x, y) == end {
            return Some(len);
        }
        for (dx, dy) in [(0,-1), (-1,0), (0,1), (1,0)] {
            let (nx, ny) = ((x as isize + dx) as usize, (y as isize + dy) as usize);
            let Some(&square) = input.get(nx).and_then(|row| row.get(ny)) else { continue };
            if input[x][y] + 1 >= square && !visited[nx][ny] {
                visited[nx][ny] = true;
                queue.push_back((nx, ny, len + 1));
            }
        }
    }
    None
}

#[allow(dead_code)]
fn reverse_bfs (input: &[Vec<u8>], start: &(usize, usize), end: &Vec<(usize, usize)>) -> Option<usize> {
    let mut visited = vec![vec![false; input[0].len()]; input.len()];
    let mut queue = VecDeque::new();
    queue.push_back((start.0, start.1, 0));
    
    let mut end_matrix = vec![vec![false; input[0].len()]; input.len()];
    for (x, y) in end { end_matrix[*x][*y] = true; }
    
    let mut all_printed = Vec::new();

    while let Some((x, y, len)) = queue.pop_front() {
        if end_matrix[x][y] {
            return Some(len);
        }
        
        if !all_printed.contains(&input[x][y]) {
            all_printed.push(input[x][y]);
        }
        
        for (dx, dy) in [(0,-1), (-1,0), (0,1), (1,0)] {
            let (nx, ny) = ((x as isize + dx) as usize, (y as isize + dy) as usize);
            let Some(&square) = input.get(nx).and_then(|row| row.get(ny)) else { continue };
            // input[x][y] can be at most 1 higher than the square we are looking at
            if input[x][y] - 1 <= square && !visited[nx][ny] {
                visited[nx][ny] = true;
                queue.push_back((nx, ny, len + 1));
            }
        }
    }
    None
}

#[allow(dead_code)]
pub fn task1(input: &str) -> i32 {
    let (parsed_input, start, end) = parse_input(input, 1);
    let result = bfs(&parsed_input, &start, end).unwrap() as i32;
    println!("Result: {:?}", result);
    result
}

#[allow(dead_code)]
pub fn task2(input: &str) -> i32 {
    let (parsed_input, start, end) = parse_input(input, 2);
    let result = reverse_bfs(&parsed_input, &end, &start).unwrap() as i32;
    println!("Result: {:?}", result);
    result
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_task1() {
        let path = "input_test/day12_test.txt";
        assert_eq!(task1(path), 31);
    }
}