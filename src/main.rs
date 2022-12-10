mod line_reader;

mod d1;
mod d2;
mod d3;
mod d4;
mod d5;
mod d6;
mod d7;
mod d8;
mod d9;
mod d10;


fn main() {
    let path = "src/input_sources/day10_input.txt";
    let test_path = "src/input_sources/day10_test.txt";
    
    println!("Day 10 Task 1");
    d10::task1(path);
    println!("Day 10 Task 2");
    d10::task2(path);
    
    // println!("Debug");
    // d10::task1(test_path);
    // d9::task2(test_path);
}