use crate::line_reader::lines_from_file;

pub fn task1() {
    let path: &str = "src/input_sources/day4_input.txt";
    let input: Vec<String> = lines_from_file(path);
    let mut counter: i32 = 0;

    for line in &input {
        let ranges: Vec<&str> = line.split(',').collect();
        let ranges: Vec<Vec<i32>> = ranges.iter().map(|x| {
            x.clone().split('-').map(|s| s.parse::<i32>().unwrap()).collect()
        }).collect();

        if contained_check(&ranges) {
            counter += 1;
        }
    }
    println!("{counter}")
}


pub fn task2() {
    let path: &str = "src/input_sources/day4_input.txt";
    let input: Vec<String> = lines_from_file(path);
    let mut counter: i32 = 0;

    for line in &input {
        let ranges: Vec<&str> = line.split(',').collect();
        let ranges: Vec<Vec<i32>> = ranges.iter().map(|x| {
            x.clone().split('-').map(|s| s.parse::<i32>().unwrap()).collect()
        }).collect();

        if overlap_check(&ranges) {
            counter += 1;
        }
    }
    println!("{counter}")
}

fn contained_check (ranges: &Vec<Vec<i32>>) -> bool {
    //split ranges into a and b vectors
    let  a: Vec<i32> = Vec::from_iter(ranges[0].clone());
    let  b: Vec<i32> = Vec::from_iter(ranges[1].clone());

    return if a[0] >= b[0] && a[1] <= b[1] {
        true
    } else if b[0] >= a[0] && b[1] <= a[1] {
        true
    } else {
        false
    }
}

fn overlap_check (ranges: &Vec<Vec<i32>>) -> bool {
    //split ranges into a and b vectors
    let  a: Vec<i32> = Vec::from_iter(ranges[0].clone());
    let  b: Vec<i32> = Vec::from_iter(ranges[1].clone());

    // return true if a and b have any overlap, dont have to be fully contained
    return if a[0] >= b[0] && a[0] <= b[1] {
        true
    } else if b[0] <= a[1] && b[0] >= a[0] {
        true
    } else {
        false
    }

}
