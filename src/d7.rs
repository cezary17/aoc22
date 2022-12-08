use crate::line_reader::lines_from_file;
use std::collections::BTreeMap;

pub fn task1(path: &str) -> i64 {
    let input = lines_from_file(path);

    let mut dir_map: BTreeMap<String, i64> = BTreeMap::new();

    let mut curr_path: Vec<String> = Vec::new();

    for line in input {
        let mut line = line.split_whitespace();
        let part1 = line.next().unwrap();
        let part2 = line.next().unwrap_or("");
        let part3 = line.next().unwrap_or("");
        match part1 {
            "$" => {
                match part2 {
                    "cd" => {
                        match part3 {
                            ".." => {
                                curr_path.pop();
                            },
                            "/" => {
                                curr_path.clear();
                                curr_path.push("/".to_string());
                            },
                            _ => {
                                curr_path.push(part3.to_string());
                            }
                        }
                    },
                    "ls" => {},
                    _ => unreachable!("Unexpected part2: {}", part2)
                }
            },

            "dir" => {},

            _ => {
                let size: i64 = part1.parse::<i64>().unwrap();

                for i in 0..curr_path.len() {
                    dir_map.entry(curr_path[0..=i].join("-").to_string()).and_modify(|v| *v += size).or_insert(size);
                }
            }
        }
    }
    let sum=  dir_map.iter().filter(|(_, &size)| size < 100000).map(|(_, size)| size).sum::<i64>();
    println!("Sum: {}", sum);
    sum
}
pub fn task2(path: &str) -> i64 {
    let input = lines_from_file(path);

    let mut dir_map: BTreeMap<String, i64> = BTreeMap::new();

    let mut curr_path: Vec<String> = Vec::new();
    curr_path.push("/".to_string());

    for line in input {
        let mut line = line.split_whitespace();
        let part1 = line.next().unwrap();
        let part2 = line.next().unwrap_or("");
        let part3 = line.next().unwrap_or("");
        match part1 {
            "$" => {
                match part2 {
                    "cd" => {
                        match part3 {
                            ".." => {
                                curr_path.pop();
                            },
                            "/" => {
                                curr_path.clear();
                                curr_path.push("/".to_string());
                            },
                            _ => {
                                curr_path.push(part3.to_string());
                            }
                        }
                    },
                    "ls" => {},
                    _ => unreachable!("Unexpected part2: {}", part2)
                }
            },

            "dir" => {},

            _ => {
                let size: i64 = part1.parse::<i64>().unwrap();
                for i in 0..curr_path.len() {
                    dir_map.entry(curr_path[0..=i].join("-").to_string()).and_modify(|v| *v += size).or_insert(size);
                }
            }
        }
    }
    const MAX_SIZE: i64 = 70_000_000;
    const UPDATE_SIZE: i64 = 30_000_000;
    
    let curr_size = MAX_SIZE - dir_map.get("/").unwrap();
    
    let mut smallest_dir = dir_map.iter().filter(|(_, &size)| size > UPDATE_SIZE - curr_size).map(|(_, size)| size).min().unwrap().clone();
    println!("Smallest dir: {}", smallest_dir);
    smallest_dir
}   

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn part1_works() {
        assert_eq!(task1("src/input_sources/day7_test.txt"), 95437);
    }

    #[test]
    fn part2_works() {
        assert_eq!(task2("src/input_sources/day7_test.txt"), 24933642);
    }
}
