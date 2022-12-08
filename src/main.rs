mod line_reader;

mod d1;
mod d2;
mod d3;
mod d4;
mod d5;
mod d6;
mod d7;
mod d8;


fn main() {
    let path = "src/input_sources/day8_input.txt";
    let test_path = "src/input_sources/day8_test.txt";
    
    println!("Day 8 Task 1");
    d8::task1(path);
    println!("Day 7 Task 2");
    d8::task2(path);
    
    // println!("Debug");
    // d8::task1(test_path);
    // d8::task2(test_path);
}