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
mod d14;


fn main() {
    
    #[allow(unused_variables)]
    let path = "input/2022/day14.txt";

    #[allow(unused_variables)]
    let test_path = "input_test/day14_test.txt";
    
    println!("Day 14 Task 1");
    d14::task1(path);
    println!("Day 14 Task 2");
    d14::task2(path);
    
    // println!("Debug");
    // d14::task1(test_path);
    // d14::task2(test_path);
}