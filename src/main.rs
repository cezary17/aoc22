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


fn main() {
    let path = "src/input_sources/day9_input.txt";
    let test_path = "src/input_sources/day9_test.txt";
    let test_path2 = "src/input_sources/day9_test2.txt";
    
    println!("Day 9 Task 1");
    d9::task1(path);
    println!("Day 9 Task 2");
    d9::task2(path);
    
    // println!("Debug");
    // d9::task1(test_path);
    // d9::task2(test_path2);
}