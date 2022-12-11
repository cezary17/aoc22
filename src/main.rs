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


fn main() {
    let path = "input/2022/day11.txt";
    let test_path = "input_test/day11_test.txt";
    
    println!("Day 11 Task 1");
    d11::task1(path);
    println!("Day 11 Task 2");
    d11::task2(path);
    
    // println!("Debug");
    // d11::task1(test_path);
    // d11::task2(test_path);
}