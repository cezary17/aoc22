use std::collections::HashMap;
use std::str::FromStr;
use itertools::{max, min};

#[derive(PartialEq)]
enum Tile {
    Air,
    Rock,
    Sand
}

struct Cave {
    tiles: HashMap<(usize, usize), Tile>,
    min_x: usize,
    max_x: usize,
    max_y: usize,
    sand_drop: (usize, usize),
    overflow: bool
}

impl Default for Cave {
    fn default() -> Self {
        Cave {
            tiles: Default::default(),
            min_x: usize::MAX,
            max_x: usize::MIN,
            max_y: usize::MIN,
            sand_drop: (500, 0),
            overflow: false
        }
    }
}

#[allow(dead_code)]
impl Cave {
    fn print(&self) {
        for y in 0..=self.max_y {
            let mut output_line = Vec::new();
            
            for x in self.min_x..=self.max_x {
                
                let tile = self.get_tile(x, y);
                output_line.push(match tile {
                    Tile::Air => ' ',
                    Tile::Rock => '#',
                    Tile::Sand => 'o',
                })
            }
            println!("{}", output_line.iter().collect::<String>());
        }
    }
    
    fn get_tile(&self, x: usize, y: usize) -> &Tile {
        self.tiles.get(&(x, y)).unwrap_or(&Tile::Air)
    }
    
    fn set_tile(&mut self, x: usize, y: usize, tile: Tile) {
        self.tiles.insert((x, y), tile);
    }
    
    fn drop_sand(&mut self) {
        let mut current_x = self.sand_drop.0;
        let mut current_y = self.sand_drop.1;
        
        loop {
            let tile_below = self.get_tile(current_x, current_y + 1);
            match tile_below {
                Tile::Air => {
                    current_y += 1;
                },
                Tile::Rock | Tile::Sand => {
                    let tile_ld = self.get_tile(current_x - 1, current_y + 1);
                    let tile_right = self.get_tile(current_x + 1, current_y + 1);
                    match (tile_ld, tile_right) {
                        (Tile::Air, _) => {
                            current_x -= 1;
                            current_y += 1;
                        },
                        (_, Tile::Air) => {
                            current_x += 1;
                            current_y += 1;
                        },
                        _ => {
                            self.set_tile(current_x, current_y, Tile::Sand);
                            // self.print();
                            break;
                        }
                    }
                }
            }
            if current_y == self.max_y + 1 {
                self.overflow = true;
                break;
            }
        }
    }
    
    fn sand_count(&self) -> i32 {
        self.tiles.values().filter(|t| **t == Tile::Sand).count() as i32
    }
    
    fn add_floor (&mut self) {
        let floor_level = self.max_y + 2;
        let mew_min_x = self.min_x - floor_level;
        let mew_max_x = self.max_x + floor_level;
        self.max_y = floor_level;
        self.min_x = mew_min_x;
        self.max_x = mew_max_x;
        
        for x in self.min_x..=self.max_x {
            self.set_tile(x, floor_level, Tile::Rock);
        }
    }
    
}

impl FromStr for Cave {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        
        let mut result = Cave::default();
        
        for line in s.lines() {
            let split_line = line
                .split(" -> ")
                .map(|s| {
                    let mut parts = s.split(",");
                    let x: usize = parts.next().unwrap().parse().unwrap();
                    let y: usize = parts.next().unwrap().parse().unwrap();
                    (x, y)
                })
                .collect::<Vec<_>>();
            
            for endpoints in split_line.windows(2) {
                let start = endpoints[0];
                let end = endpoints[1];

                let x_start = start.0;
                let y_start = start.1;
                
                let x_end = end.0;
                let y_end = end.1;
                
                if x_start == x_end {
                    for ny in if start.1 < end.1 { start.1..=end.1 }  else { end.1..=start.1 } {
                        result.set_tile(start.0, ny, Tile::Rock);
                    }
                } else if y_start == y_end {
                    for nx in if start.0 < end.0 { start.0..=end.0 }  else { end.0..=start.0 } {
                        result.set_tile(nx, start.1, Tile::Rock);
                    }
                }
                else { panic!("Bruh moment elves") }
                
                let min_x = min([x_start, x_end]).unwrap();
                if min_x < result.min_x {
                    result.min_x = min_x;
                }
                
                let max_x = max([x_start, x_end]).unwrap();
                if max_x > result.max_x {
                    result.max_x = max_x;
                }
                
                let max_y = max([y_start, y_end]).unwrap();
                if max_y > result.max_y {
                    result.max_y = max_y;
                }
            }
        }
        assert!(result.min_x <= result.max_x);
        
        Ok(result)
    }
   
}

#[allow(dead_code)]
pub fn task1(path: &str) -> i32 {
    let mut cave: Cave = std::fs::read_to_string(path)
        .unwrap()
        .parse()
        .unwrap();
    
    // println!("Cave before");
    // cave.print();
    
    while !cave.overflow {
        cave.drop_sand();
    }
    
    // println!("Cave after");
    // cave.print();
    
    let sand_count = cave.sand_count();
    println!("Sand count: {}", sand_count);
    sand_count   
}

#[allow(dead_code)]
pub fn task2(path: &str) -> i32 {
    let mut cave: Cave = std::fs::read_to_string(path)
        .unwrap()
        .parse()
        .unwrap();
    
    cave.add_floor();

    // println!("Cave before");
    // cave.print();

    while cave.get_tile(cave.sand_drop.0, cave.sand_drop.1 ) == &Tile::Air {
        if cave.overflow {
            panic!("Overflowed");
        }
        cave.drop_sand();
    }

    // println!("Cave after");
    // cave.print();

    let sand_count = cave.sand_count();
    println!("Sand count: {}", sand_count);
    sand_count
}