use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use ndarray::{Array2, Axis};

fn read_trees(filename: impl AsRef<Path>) -> Vec<Vec<u8>> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        //parse the string into a vector of chars
        .map(|l| l.chars().collect::<Vec<char>>())
        //parse the chars into a vector of u8
        .map(|l| l.iter().map(|c| c.to_digit(10).unwrap() as u8).collect::<Vec<u8>>())
        .collect()
}

fn lr_max(trees: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let mut max_matrix:Vec<Vec<u8>> = Vec::new();
    for i in 0..trees[0].len() {
        let mut row: Vec<u8> = Vec::new();
        let mut current_max: u8 = 0;
        for j in 0..trees[i].len() {
            if trees[i][j] > current_max {
                row.push(trees[i][j]);
                current_max = trees[i][j];
            } else {
                row.push(current_max);
            }
        }
        max_matrix.push(row);
        
    }
    max_matrix
}

fn rl_max(trees: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    // build a matrix with the maximal tree height encountered while going right to left
    let mut max_matrix:Vec<Vec<u8>> = Vec::new();
    // i = height, j = width
    for i in 0..trees[0].len() {
        let mut row: Vec<u8> = Vec::new();
        let mut current_max: u8 = 0;
        for j in (0..trees[i].len()).rev() {
            // println!("i: {}, j: {}", i, j);
            if trees[i][j] > current_max {
                row.push(trees[i][j]);
                current_max = trees[i][j];
            } else {
                row.push(current_max);
            }
        }
        row.reverse();
        max_matrix.push(row);
        
    }
    max_matrix
}

fn transpose_matrix(trees: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let mut arr = Array2::<u8>::default((trees.len(), trees[0].len()));
    for (i, mut row) in arr.axis_iter_mut(Axis(0)).enumerate() {
        for (j, col) in row.iter_mut().enumerate() {
            *col = trees[i][j];
        }
    }
    arr.t()
    .outer_iter()
    .map(|row| row.iter().map(|&x| x).collect::<Vec<u8>>())
    .collect()
}

#[allow(dead_code)]
pub fn task1(input: &str) -> i32 {
    let trees = read_trees(input);

    let lr_max_matrix = lr_max(&trees);
    let rl_max_matrix = rl_max(&trees);

    let transposed_trees = transpose_matrix(&trees);
    let lr_transposed_trees = lr_max(&transposed_trees);
    let rl_transposed_trees = rl_max(&transposed_trees);

    let td_max = transpose_matrix(&lr_transposed_trees);
    let dt_max = transpose_matrix(&rl_transposed_trees);

    let mut hidden_trees: Vec<Vec<u8>> = Vec::new();

    hidden_trees.push(vec![0; trees[0].len()]);
    for i in 1..trees.len()-1 {
        let mut row: Vec<u8> = Vec::new();
        row.push(0);
        for j in 1..trees[i].len()-1 {
            let mut hidden_trees: u8 = 0;
            if (trees[i][j] <= lr_max_matrix[i][j-1]) &&
                (trees[i][j] <= rl_max_matrix[i][j+1]) &&
                (trees[i][j] <= td_max[i-1][j]) &&
                (trees[i][j] <= dt_max[i+1][j]) {
                hidden_trees = 1;
            }
            row.push(hidden_trees);
        }
        row.push(0);
        hidden_trees.push(row);
    }
    hidden_trees.push(vec![0; trees[0].len()]);

    let hidden_trees_count: i32 = hidden_trees.iter().map(|row| row.iter().sum::<u8>() as i32).sum::<i32>();
    let all_trees_count:i32 = (trees.len() * trees[0].len()) as i32;
    let visible_trees_count = all_trees_count - hidden_trees_count;
    println!("visible trees_count: {}", visible_trees_count);
    visible_trees_count
}

#[allow(dead_code)]
pub fn task2(input: &str) -> i32 {
    let trees = read_trees(input);

    let lr_max_matrix = lr_max(&trees);
    let rl_max_matrix = rl_max(&trees);

    let transposed_trees = transpose_matrix(&trees);
    let lr_transposed_trees = lr_max(&transposed_trees);
    let rl_transposed_trees = rl_max(&transposed_trees);

    let td_max = transpose_matrix(&lr_transposed_trees);
    let dt_max = transpose_matrix(&rl_transposed_trees);

    let mut hidden_trees: Vec<Vec<i32>> = Vec::new();

    hidden_trees.push(vec![0; trees[0].len()]);
    let mut max_height_index = [0,0];
    let mut max_height = 0;
    for i in 1..trees.len()-1 {
        let mut row: Vec<i32> = Vec::new();
        row.push(0);
        for j in 1..trees[i].len()-1 {
            let mut scenic_score: [i32; 4] = [1, 1, 1, 1];
            let current_tree = trees[i][j];

            // get the distance between the tree and the next taller tree in the left direction
            let mut l = j-1;
            loop {
                if lr_max_matrix[i][l] < current_tree {
                    scenic_score[0] = j as i32;
                    break;
                }
                else if trees[i][l] < current_tree {
                    scenic_score[0] += 1;
                    if l == 0 {
                        break;
                    } else{
                        l -= 1;
                    }
                } else {
                    break;
                }
            }
            // get the distance between the tree and the next taller tree in the right direction
            let mut r = j+1;
            loop {
                if rl_max_matrix[i][r] < current_tree {
                    scenic_score[1] = (trees[i].len() - j - 1) as i32;
                    break;
                }
                else if trees[i][r] < current_tree {
                    scenic_score[1] += 1;
                    if r == trees[i].len() - 1 {
                        break;
                    } else{
                        r += 1;
                    }
                } else {
                    break;
                }
            }
            // get the distance between the tree and the next taller tree in the top direction
            let mut t = i-1;
            loop {
                if td_max[t][j] < current_tree {
                    scenic_score[2] = i as i32;
                    break;
                }
                else if trees[t][j] < current_tree {
                    scenic_score[2] += 1;
                    if t == 0 {
                        break;
                    } else{
                        t -= 1;
                    }
                } else {
                    break;
                }
            }
            // get the distance between the tree and the next taller tree in the bottom direction
            let mut d = i+1;
            loop {
                if dt_max[d][j] < current_tree {
                    scenic_score[3] = (trees.len() - i - 1) as i32;
                    break;
                }
                else if trees[d][j] < current_tree {
                    scenic_score[3] += 1;
                    if d == trees.len() - 1 {
                        break;
                    } else{
                        d += 1;
                    }
                } else {
                    break;
                }
            }
            let combined_score = scenic_score.iter().product();
            if combined_score > max_height {
                max_height = combined_score;
                max_height_index = [i, j];
            }
            row.push(combined_score);
        }
        row.push(0);
        hidden_trees.push(row);
    }
    hidden_trees.push(vec![0; trees[0].len()]);

    let max_score = hidden_trees.iter().map(|row| row.iter().max().unwrap()).max().unwrap();
    println!("max score: {}", max_score);
    // println!("max score index: {:?}", max_height_index);
    *max_score
}