use crate::line_reader::lines_from_file;
use std::collections::HashSet;

#[allow(dead_code)]
pub fn task1() {
    let path: &str = "src/input_sources/day3_input.txt";
    let input: Vec<String> = lines_from_file(path);
    let mut priority: i32 = 0;

    for backpack in input {
        let split_backpack: (&str, &str) = backpack.split_at(backpack.len() / 2);

        let compartment_0: HashSet<u8> = HashSet::from_iter(split_backpack.0.as_bytes().to_vec());
        let compartment_1: HashSet<u8> = HashSet::from_iter(split_backpack.1.as_bytes().to_vec());

        let intersection: HashSet<u8> = compartment_0.intersection(&compartment_1).cloned().collect();

        let duplicate_item: u8 = intersection.into_iter().collect::<Vec<u8>>()[0];

        priority += u8_to_priority(duplicate_item)

    }
    println!("{priority}")
}

#[allow(dead_code)]
pub fn task2() {
    let path: &str = "src/input_sources/day3_input.txt";
    let input: Vec<String> = lines_from_file(path);

    let mut priority: i32 = 0;
    let elf_teams = input.chunks(3);

    for team in elf_teams {

        // Make a vec of hashsets of the 3 elfs in team
        let mut elfs: Vec<HashSet<u8>> = Vec::new();
        for elf in team {
            elfs.push(HashSet::from_iter(elf.as_bytes().to_vec()));
        }

        let mut elf_iter = elfs.iter();
        let intersection = elf_iter.clone().fold(elf_iter.next().unwrap().clone(), |acc, x| acc.intersection(x).cloned().collect());

        let duplicate_item: u8 = intersection.into_iter().collect::<Vec<u8>>()[0];
        priority += u8_to_priority(duplicate_item);

    }
    println!("{priority}");
}

fn u8_to_priority(item:u8) -> i32 {
    match item {
        65..=90 => return i32::from(item) - 64 + 26,
        97..=122 => return i32::from(item) - 96,
        _ => panic!("Bruh moment")
    }
}