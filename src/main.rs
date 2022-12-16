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
mod d11;
mod d12;
mod d13;


fn main() {
    
    #[allow(unused_variables)]
    let path = "input/2022/day13.txt";

    #[allow(unused_variables)]
    let test_path = "input_test/day13_test.txt";
    
    println!("Day 13 Task 1");
    d13::task1(path);
    println!("Day 13 Task 2");
    d13::task2(path);
    
    // println!("Debug");
    // d13::task1(test_path);
    // d13::task2(test_path);
}