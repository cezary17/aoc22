use serde::Deserialize;
use std::cmp::Ordering;
use crate::line_reader::lines_from_file;

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
enum Packet {
    Term(u8),
    Complex(Vec<Packet>),
}

impl Eq for Packet {}

impl PartialEq<Self> for Packet {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl PartialOrd<Self> for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Packet::Term(a), Packet::Term(b)) => a.cmp(b),
            (Packet::Complex(a), Packet::Complex(b)) => a.cmp(b),
            (Packet::Term(a), Packet::Complex(b)) => [Packet::Term(*a)][..].cmp(b),
            (Packet::Complex(a), Packet::Term(b)) => a.as_slice().cmp(&[Packet::Term(*b)]),
        }
    }
}

#[allow(dead_code)]
fn parse_input_task1(path: &str) -> Vec<(String, String)> {
    let input = lines_from_file(path);
    let mut output = Vec::new();
    let mut curr_vec: Vec<String> = Vec::new();
    for line in input {
        if line == "" {
            let tuple: (String, String) = (curr_vec[0].clone(), curr_vec[1].clone());
            output.push(tuple);
            curr_vec = Vec::new();
        } else {
            curr_vec.push(line);
        }
    }
    output
}

#[allow(dead_code)]
fn parse_input_task2(path: &str) -> Vec<String> {
    let input = lines_from_file(path);
    input.into_iter().filter(|x| x != "").collect::<Vec<String>>()
}

#[allow(dead_code)]
pub fn task1(path: &str) -> i32 {
    let input = parse_input_task1(path);
    let packets = input.iter().map(|(a, b)| (serde_json::from_str::<Packet>(&a).unwrap(), serde_json::from_str::<Packet>(&b).unwrap())).collect::<Vec<(Packet, Packet)>>();
    let mut count = 1;
    let mut res = 0;
    for (a, b) in packets {
        if a < b {
            res += count;
        }
        count += 1;
    }
    println!("Result: {}", res);
    res
}

#[allow(dead_code)]
pub fn task2(path: &str) -> i32 {
    let input = parse_input_task2(path);
    let mut packets = input.iter().map(|x| serde_json::from_str::<Packet>(&x).unwrap()).collect::<Vec<Packet>>();
    
    let pac2: Packet = serde_json::from_str("[[2]]").unwrap();
    let pac6: Packet = serde_json::from_str("[[6]]").unwrap();
    
    packets.push(pac2.clone());
    packets.push(pac6.clone());
    
    packets.sort_unstable();

    let pac2 = packets.binary_search(&pac2).unwrap() + 1;
    let pac6 = packets.binary_search(&pac6).unwrap() + 1;
    
    let res = pac2 as i32 * pac6 as i32;
    println!("Result: {}", res);
    res
}
