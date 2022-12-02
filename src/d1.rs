use std::collections::BinaryHeap;
use crate::line_reader::lines_from_file;

#[allow(dead_code)]
pub fn d1_task1() {

    let path_to_input = "src/input_sources/day1_input.txt";

    let elf_heap = make_heap(path_to_input);

    let elf_max = elf_heap.peek().unwrap().clone();

    println!("Most packed elf: {}", elf_max)
}

#[allow(dead_code)]
pub fn d2_task() {
    let path_to_input = "src/input_sources/day1_input.txt";

    let mut elf_heap = make_heap(path_to_input);

    let elf_max_1 = elf_heap.pop().unwrap();

    let elf_max_2 = elf_heap.pop().unwrap();

    let elf_max_3 = elf_heap.pop().unwrap();

    let total = elf_max_1 + elf_max_2 + elf_max_3;

    println!("Total: {}", total)

}
#[allow(dead_code)]
fn make_heap(path: &str) -> BinaryHeap<i32> {
    let calories =  lines_from_file(path);
    let mut elf_heap: BinaryHeap<i32> = BinaryHeap::new();
    let mut curr: i32 = 0;

    for item in calories {
        let item_int: i32 = match item.parse() {
            Ok(num) => num,
            Err(_) => 0,
        };
        if item_int > 0 {
            curr += item_int;
        }
        else {
            elf_heap.push(curr);
            curr = 0;
        }
    }

    return elf_heap
}