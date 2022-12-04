use crate::line_reader::lines_from_file;

/*
    Opponent:
    A -> Rock
    B -> Paper
    C -> Scissors

    Me:
    X -> Rock
    Y -> Paper
    Z -> Scissors

    Points:
    1 -> Rock
    2 -> Paper
    3 -> Scissors

    0 -> Loss
    3 -> Draw
    6 -> Win

 */
#[allow(dead_code)]
pub fn task1() {
    let path_to_input = "src/input_sources/day2_input.txt";
    let input = lines_from_file(path_to_input);

    let mut points = 0;

    for game in input {
        match game.as_str() {
            "A X" => points += 4,
            "A Y" => points += 8,
            "A Z" => points += 3,
            "B X" => points += 1,
            "B Y" => points += 5,
            "B Z" => points += 9,
            "C X" => points += 7,
            "C Y" => points += 2,
            "C Z" => points += 6,
            _ => {}
        }
    }
    println!("{}", points)
}

/*
    Opponent:
    A -> Rock
    B -> Paper
    C -> Scissors

    Me:
    X -> Loss
    Y -> Draw
    Z -> Win

    Points:
    1 -> Rock
    2 -> Paper
    3 -> Scissors

    0 -> Loss
    3 -> Draw
    6 -> Win

 */
#[allow(dead_code)]
pub fn task2() {
    let path_to_input = "src/input_sources/day2_input.txt";
    let input = lines_from_file(path_to_input);

    let mut points = 0;

    for game in input {
        match game.as_str() {
            "A X" => points += 0 + 3,
            "A Y" => points += 3 + 1,
            "A Z" => points += 6 + 2,
            "B X" => points += 0 + 1,
            "B Y" => points += 3 + 2,
            "B Z" => points += 6 + 3,
            "C X" => points += 0 + 2,
            "C Y" => points += 3 + 3,
            "C Z" => points += 6 + 1,
            _ => {}
        }
    }
    println!("{}", points)
}