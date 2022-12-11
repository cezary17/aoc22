use crate::line_reader::lines_from_file;

fn touching_spaces(position: (i32, i32)) -> Vec<(i32, i32)> {
    let mut result: Vec<(i32, i32)> = Vec::new();
    result.push((position.0 , position.1)); // current position
    result.push((position.0 - 1, position.1)); // left
    result.push((position.0 - 1, position.1 + 1)); // left up
    result.push((position.0, position.1 + 1)); // up
    result.push((position.0 + 1, position.1 + 1)); // right up
    result.push((position.0 + 1, position.1)); // right
    result.push((position.0 + 1, position.1 - 1)); // right down
    result.push((position.0, position.1 - 1)); // down
    result.push((position.0 - 1, position.1 - 1)); // left down
    result
}

fn chase_head(head_pos: (i32, i32), tail_pos: (i32, i32)) -> (i32, i32) {
    // linear movements first
    let mut res = (0,0);
    if head_pos.0 == tail_pos.0 {
        if head_pos.1 > tail_pos.1 {
            res = (tail_pos.0, tail_pos.1 + 1);
        } else {
            res = (tail_pos.0, tail_pos.1 - 1);
        }
    } else if head_pos.1 == tail_pos.1 {
        if head_pos.0 > tail_pos.0 {
            res = (tail_pos.0 + 1, tail_pos.1);
        } else {
            res = (tail_pos.0 - 1, tail_pos.1);
        }
    }
    // diagonal movements
    else {
        if head_pos.0 > tail_pos.0 {
            if head_pos.1 > tail_pos.1 {
                res = (tail_pos.0 + 1, tail_pos.1 + 1);
            } else {
                res = (tail_pos.0 + 1, tail_pos.1 - 1);
            }
        }
        else {
            if head_pos.1 > tail_pos.1 {
                res = (tail_pos.0 - 1, tail_pos.1 + 1);
            } else {
                res = (tail_pos.0 - 1, tail_pos.1 - 1);
            }
        }
    }
    res
}

#[allow(dead_code)]
fn debug_printer(coordinates: &Vec<(i32, i32)>) {
    let mut grid = [['.'; 100]; 100];

    for (x, y) in coordinates {
        let true_x = (x + 50) as usize;
        let true_y = (y + 50) as usize;
        grid[true_y][true_x] = '#';
    }

    grid[50][50] = 's';

    for row in grid.iter().rev() {
        for cell in row.iter() {
            print!("{}", cell);
        }
        println!();
    }
}

#[allow(dead_code)]
pub fn task1(path: &str) -> i32 {
    let input = lines_from_file(path);
    
    // starting position is [0,0]
    let mut head_position = (0,0);
    let mut tail_position = (0,0);
    
    let mut tail_counter:Vec<(i32, i32)> = Vec::new();
    tail_counter.push(tail_position);
    
    for line in input {
        let line_vec = line.split_whitespace().collect::<Vec<&str>>();
        let (direction, distance) = (line_vec[0], line_vec[1].parse::<i32>().unwrap());
        for _ in 0..distance {
            match direction {
                "R" => head_position.0 += 1,
                "L" => head_position.0 -= 1,
                "U" => head_position.1 += 1,
                "D" => head_position.1 -= 1,
                _ => unreachable!(),
            }
            let touching = touching_spaces(tail_position);
            if !touching.contains(&head_position) {
                tail_position = chase_head(head_position, tail_position);
                if !tail_counter.contains(&tail_position) {
                    tail_counter.push(tail_position);
                }
            }
        }
        
    }
    println!("Tail counter length: {}", tail_counter.len());
    tail_counter.len() as i32
    
} 

#[allow(dead_code)]
pub fn task2(path: &str) -> i32 {
    let input = lines_from_file(path);

    let mut head_position = (0,0);
    let mut tails: Vec<(i32, i32)> = vec![(0, 0); 9];
    
    let mut tail_counter:Vec<(i32, i32)> = Vec::new();
    tail_counter.push((0, 0));
    
    for line in input {
        let line_vec = line.split_whitespace().collect::<Vec<&str>>();
        let (direction, distance) = (line_vec[0], line_vec[1].parse::<i32>().unwrap());
        for _ in 0..distance {
            match direction {
                "R" => head_position.0 += 1,
                "L" => head_position.0 -= 1,
                "U" => head_position.1 += 1,
                "D" => head_position.1 -= 1,
                _ => unreachable!(),
            }
            
            let mut current_head = head_position.clone();
            
            for i in 0..tails.len() {
                    let touching = touching_spaces(tails[i]);
                    if !touching.contains(&current_head) {
                        tails[i] = chase_head(current_head, tails[i]);
                        if (!tail_counter.contains(&tails[i])) && i == tails.len() - 1 {
                            tail_counter.push(tails[i]);
                        }
                    }
                    current_head = tails[i];
            }
        }
    }
    println!("Tail counter length: {}", tail_counter.len());
    tail_counter.len() as i32

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        assert_eq!(task1("src/input_sources/day9_test.txt"), 13);
    }

    #[test]
    fn part2_works() {
        assert_eq!(task2("src/input_sources/day9_test2.txt"), 36);
    }
}