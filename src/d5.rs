use crate::line_reader::lines_from_file;

#[allow(dead_code)]
pub fn task1() {
    let path: &str = "src/input_sources/day5_input.txt";
    let input: Vec<String> = lines_from_file(path);

    let split = input.split_at(8);
    let stacks_input = split.0;

    // Parsing the stacks
    
    let mut stacks = Vec::new();
    let indexes = [1,5,9,13,17,21,25,29,33];
    for i in indexes {
        let mut curr_stack = Vec::new();
        for level in stacks_input {
            let b = level.as_bytes()[i];
            let c = b as char;
            if c as u8 != 0x20 {
                curr_stack.push(c);
            }
        }
        curr_stack.reverse();
        stacks.push(curr_stack)
        
    }
    
    // Parsing the moves
    
    let moves_input = split.1;
    let moves_input = &moves_input[2..];
    
    let mut moves_vec: Vec<Vec<i32>> = Vec::new();
    for m in moves_input {
        let m_split = m.split(' ');
        let mut nums_only = Vec::new();
        for num in m_split {
            match num.parse::<i32>() {
                Ok(n) => nums_only.push(n),
                Err(_) => continue,
            }
        }
        moves_vec.push(nums_only);
    }
    
    for op in moves_vec {
        let count = op[0];
        let from = (op[1]-1) as usize;
        let to = (op[2]-1) as usize;
        
        let mut curr_vec:Vec<char> = Vec::new();
        for _ in 0..count {
            let temp: &mut Vec<char> = &mut stacks[from];
            let val = temp.pop();
            curr_vec.push(val.unwrap());
        }
        
        let dest: &mut Vec<char> = &mut stacks[to];
        dest.append(&mut curr_vec);
    }
    
    let mut acc:String = String::new();
    for mut s in stacks {
        let cur = s.pop().unwrap().to_string();
        acc.push_str(cur.as_str());
    }
    println!("{acc}")
}

#[allow(dead_code)]
pub fn task2() {
    let path: &str = "src/input_sources/day5_input.txt";
    let input: Vec<String> = lines_from_file(path);

    let split = input.split_at(8);
    let stacks_input = split.0;
    
    // Parsing the stacks

    let mut stacks = Vec::new();
    let indexes = [1,5,9,13,17,21,25,29,33];
    for i in indexes {
        let mut curr_stack = Vec::new();
        for level in stacks_input {
            let b = level.as_bytes()[i];
            let c = b as char;
            if c as u8 != 0x20 {
                curr_stack.push(c);
            }
        }
        curr_stack.reverse();
        stacks.push(curr_stack)
    }

    // Parsing the moves

    let moves_input = split.1;
    let moves_input = &moves_input[2..];

    let mut moves_vec: Vec<Vec<i32>> = Vec::new();
    for m in moves_input {
        let m_split = m.split(' ');
        let mut nums_only = Vec::new();
        for num in m_split {
            match num.parse::<i32>() {
                Ok(n) => nums_only.push(n),
                Err(_) => continue,
            }
        }
        moves_vec.push(nums_only);
    }

    for op in moves_vec {
        let count = op[0];
        let from = (op[1]-1) as usize;
        let to = (op[2]-1) as usize;
        
        let mut curr_vec:Vec<char> = Vec::new();
        for _ in 0..count {
            let temp: &mut Vec<char> = &mut stacks[from];
            let val = temp.pop();
            curr_vec.push(val.unwrap());
        }
        curr_vec.reverse();

        let dest: &mut Vec<char> = &mut stacks[to];
        dest.append(&mut curr_vec);
    }

    let mut acc:String = String::new();
    for mut s in stacks {
        let cur = s.pop().unwrap().to_string();
        acc.push_str(cur.as_str());
    }
    print!("{acc}")
}