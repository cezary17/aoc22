use std::str::FromStr;
use crate::line_reader::lines_from_file;

#[derive(Debug)]
struct Sensor {
    x: i32,
    y: i32,
    x_beacon: i32,
    y_beacon: i32,
    distance: i32
}

impl Default for Sensor {
    fn default() -> Self {
        Sensor {
            x: 0,
            y: 0,
            x_beacon: 0,
            y_beacon: 0,
            distance: 0
        }
    }
}

impl Sensor {
    fn dist_sensor_beacon(&self) -> i32 {
        (self.x - self.x_beacon).abs() + (self.y - self.y_beacon).abs()
    }
    fn in_range(&self, x: i32, y: i32) -> bool {
        (self.x - x).abs() + (self.y - y).abs() <= self.distance
    }
}

impl FromStr for Sensor {
    type Err = ();
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut result = Sensor::default();
        
        // it's beautiful isn't it?
        let split_line = s
            .split(" ")
            .filter(|s| s.split("=").count() == 2)
            .map(|s| s.trim_end_matches(|c| c == ',' || c == ':'))
            .map(|s| s.split("=").nth(1).unwrap().parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        
        result.x = split_line[0];
        result.y = split_line[1];
        result.x_beacon = split_line[2];
        result.y_beacon = split_line[3];
        result.distance = result.dist_sensor_beacon();
        
        Ok(result)
    }
}

fn get_borders(sensors: &Vec<Sensor>) -> (i32, i32) {
    
    let min_x = sensors
        .iter()
        .map(|s| s.x.min(s.x_beacon))
        .min()
        .unwrap();
    
    let max_x = sensors
        .iter()
        .map(|s| s.x.max(s.x_beacon))
        .max()
        .unwrap();
    
    let max_range = sensors
        .iter()
        .map(|s| s.distance)
        .max()
        .unwrap();

    (min_x - max_range, max_x + max_range)
}

pub fn task1(path: &str) -> i32 {
    
    let sensors = lines_from_file(path)
        .iter()
        .map(|line| line.parse::<Sensor>().unwrap())
        .collect::<Vec<_>>();
    
    // const Y: i32 = 10;
    const Y: i32 = 2000000;
    
    let borders = get_borders(&sensors);

    let occupied = sensors.iter().fold(vec![], |mut acc, sensor| {
        if sensor.y == Y {
            acc.push(sensor.x);
        } else if sensor.y_beacon == Y && !acc.contains(&sensor.x_beacon) {
            acc.push(sensor.x_beacon);
        }
        acc
    });

    let mut count = 0;
    for x in borders.0..=borders.1 {
        'sensors: for sensor in &sensors {
            if sensor.in_range(x, Y) && !occupied.contains(&x) {
                count += 1;
                break 'sensors;
            }
        }
    }
    println!("Taken positions: {}", count);
    count
}

fn gen_lines(sensors: &Vec<Sensor>) -> (Vec<i32>, Vec<i32>) {
    let pos_lines = sensors.iter().map(|s| {
        let dist = s.dist_sensor_beacon();
        vec![
            s.x - s.y + dist,
            s.x - s.y - dist
        ]
    }).flatten().collect::<Vec<_>>();
    
    let neg_lines = sensors.iter().map(|s| {
        let dist = s.dist_sensor_beacon();
        vec![
            s.x + s.y + dist,
            s.x + s.y - dist
        ]
    }).flatten().collect::<Vec<_>>();
    
    (pos_lines, neg_lines)   
}

pub fn task2(path: &str) -> i64 {
    let sensors = lines_from_file(path).iter().map(|line| line.parse::<Sensor>().unwrap()).collect::<Vec<_>>();
    
    const MIN: i32 = 0;
    const MAX: i64 = 4000000;

    let (pos_lines, neg_lines) = gen_lines(&sensors);
    
    let n = 2*sensors.len();
    assert_eq!(n, pos_lines.len());
    assert_eq!(pos_lines.len(), neg_lines.len());
    
    let mut pos = MIN - 1;
    let mut neg = MIN - 1;
    
    for i in 0..n {
        for j in i+1..n {
            let (a, b) = (pos_lines[i], pos_lines[j]);
            if (a - b).abs() == 2 {
                pos = a.min(b) + 1;
            }
            let (a, b) = (neg_lines[i], neg_lines[j]);
            if (a - b).abs() == 2 {
                neg = a.min(b) + 1;
            }
        }
    }
    let x: i64 = ((pos + neg) / 2) as i64;
    let y: i64 = ((neg - pos) / 2) as i64;
    let ans: i64 = x * MAX + y;
    println!("x: {}, y: {}, ans: {}", x, y, ans);
    ans
}

#[cfg(test)]
mod tests {
    use super::*;
    const PATH: &str = "input_test/day15_test.txt";
    
    // #[test]
    // fn test_task1() {
    //     assert_eq!(task1(PATH), 26);
    // }
    
    #[test]
    fn test_task2() {
        assert_eq!(task2(PATH), 56000011);
    }
}