use anyhow::Result;
use std::cmp::{max, min};
use std::fs::File;
use std::io::{prelude::*, BufReader};

const AREA_SIZE: usize = 1000;

struct Line {
    x1: u16,
    y1: u16,
    x2: u16,
    y2: u16,
}

struct Area {
    points: [[u16; AREA_SIZE]; AREA_SIZE],
}

impl Default for Area {
    fn default() -> Area {
        Area {
            points: [[0; AREA_SIZE]; AREA_SIZE],
        }
    }
}

impl Area {
    fn mark(&mut self, line: &Line) {
        if line.x1 == line.x2 {
            let x = line.x1 as usize;
            let low = min(line.y1, line.y2) as usize;
            let high = max(line.y1, line.y2) as usize;

            for y in low..=high {
                self.points[y][x] += 1;
            }
        } else if line.y1 == line.y2 {
            let y = line.y1 as usize;
            let low = min(line.x1, line.x2) as usize;
            let high = max(line.x1, line.x2) as usize;

            for x in low..=high {
                self.points[y][x] += 1;
            }
        } else {
            let low_y = min(line.y1, line.y2) as usize;
            let high_y = max(line.y1, line.y2) as usize;

            let low_x = min(line.x1, line.x2) as usize;
            let high_x = max(line.x1, line.x2) as usize;

            assert_eq!(high_y - low_y, high_x - low_x, "The line must be diagonal");

            let length = high_y - low_y;

            if (line.y1 < line.y2) == (line.x1 < line.x2) {
                for r in 0..=length {
                    self.points[low_y + r][low_x + r] += 1;
                }
            } else {
                for r in 0..=length {
                    self.points[high_y - r][low_x + r] += 1;
                }
            }
        }
    }

    fn count_overlapping_points(&self) -> u16 {
        let mut result = 0;

        for y in 0..AREA_SIZE {
            for x in 0..AREA_SIZE {
                if self.points[y][x] > 1 {
                    result += 1;
                }
            }
        }

        result
    }
}

pub fn main() -> Result<()> {
    let file = File::open("input/input-5-2.txt")?;
    let reader = BufReader::new(file);
    let lines = reader.lines().into_iter().map(|l| l.unwrap());

    let lines = lines.map(read_line).collect::<Vec<Line>>();
    let mut area = Area::default();

    for line in lines {
        area.mark(&line);
    }

    let result = area.count_overlapping_points();
    print_area(&area);
    println!("The result is: {}", result);

    Ok(())
}

fn read_line(string: String) -> Line {
    let mut split = string.split(" -> ");

    let mut point1 = split.next().unwrap().split(",");
    let mut point2 = split.next().unwrap().split(",");

    let x1 = point1.next().unwrap();
    let y1 = point1.next().unwrap();

    let x2 = point2.next().unwrap();
    let y2 = point2.next().unwrap();

    Line {
        x1: x1.parse::<u16>().unwrap(),
        y1: y1.parse::<u16>().unwrap(),
        x2: x2.parse::<u16>().unwrap(),
        y2: y2.parse::<u16>().unwrap(),
    }
}

fn print_area(area: &Area) {
    for y in 0..AREA_SIZE {
        for x in 0..AREA_SIZE {
            let point = area.points[y][x];
            if point == 0 {
                print!(".");
            } else {
                print!("{}", point);
            }
        }
        println!();
    }
}
