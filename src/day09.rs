use anyhow::{Context, Result};
use std::fs::File;
use std::io::{prelude::*, BufReader};

const MAX_POINT_VALUE: u8 = 9;

#[derive(Debug)]
struct Heightmap {
    height: usize,
    width: usize,
    values: Vec<u8>,
}

impl Heightmap {
    fn from_input(input: Vec<String>) -> Heightmap {
        let height = input.len();
        let width = input.first().map(|l| l.len()).unwrap_or(0);
        let mut values = vec![0; height * width];

        for h in 0..height {
            for (w, c) in input[h].char_indices() {
                values[h * width + w] = c.to_digit(10).unwrap() as u8;
            }
        }

        Heightmap {
            width,
            height,
            values,
        }
    }

    fn print_out_with_visited(&self, visited: &Vec<bool>) {
        for y in 0..self.height {
            for x in 0..self.width {
                let point = self.point_at(x as isize, y as isize);

                if visited[y * self.width + x] {
                    print!("{}🟢", point);
                } else if x == 0
                    || x == self.width - 1
                    || y == 0
                    || y == self.height - 1
                    || visited[(y - 1) * self.width + x]
                    || visited[y * self.width + x + 1]
                    || visited[(y + 1) * self.width + x]
                    || visited[y * self.width + x - 1]
                {
                    if point == 9 {
                        print!("{}🟠", point);
                    } else {
                        print!("{}🟡", point);
                    }
                } else {
                    print!("{}🔵", point);
                }
            }
            println!();
        }
    }

    #[inline]
    fn point_at(&self, x: isize, y: isize) -> u8 {
        if x < 0 || (x as usize) >= self.width || y < 0 || (y as usize) >= self.height {
            MAX_POINT_VALUE
        } else {
            self.values[(y as usize) * self.width + (x as usize)]
        }
    }

    fn is_low_point(&self, x: isize, y: isize) -> bool {
        let point = self.point_at(x, y);

        if point >= self.point_at(x, y - 1)
            || point >= self.point_at(x - 1, y)
            || point >= self.point_at(x + 1, y)
            || point >= self.point_at(x, y + 1)
        {
            false
        } else {
            true
        }
    }

    fn calculate_basin_size(&self, visited: &mut Vec<bool>, x: usize, y: usize) -> u32 {
        visited[y * self.width + x] = true;

        let point = self.point_at(x as isize, y as isize);
        if point == 8 {
            return 1;
        }

        // println!("( basin at {}, {}", x, y);

        let mut size = 1;

        let near_point = self.point_at(x as isize, y as isize - 1);
        if near_point > point && near_point != 9 && !visited[(y - 1) * self.width + x] {
            size += self.calculate_basin_size(visited, x, y - 1);
        }

        let near_point = self.point_at(x as isize + 1, y as isize);
        if near_point > point && near_point != 9 && !visited[y * self.width + x + 1] {
            size += self.calculate_basin_size(visited, x + 1, y);
        }

        let near_point = self.point_at(x as isize, y as isize + 1);
        if near_point > point && near_point != 9 && !visited[(y + 1) * self.width + x] {
            size += self.calculate_basin_size(visited, x, y + 1);
        }

        let near_point = self.point_at(x as isize - 1, y as isize);
        if near_point > point && near_point != 9 && !visited[y * self.width + x - 1] {
            size += self.calculate_basin_size(visited, x - 1, y);
        }

        // println!(") basin at {}, {} = {}", x, y, size);

        size
    }
}

pub fn main() -> Result<()> {
    let file = File::open("input/input-9-2.txt")?;
    let reader = BufReader::new(file);

    let lines = reader.lines().into_iter().map(|l| l.unwrap());
    let heightmap = Heightmap::from_input(lines.collect::<Vec<String>>());

    let mut result_part_1: u32 = 0;
    let mut basins = Vec::new();
    let mut visited = vec![false; heightmap.width * heightmap.height];

    let mut basin_sum = 0;

    for y in (0..heightmap.height).map(|i| i as isize) {
        for x in (0..heightmap.width).map(|i| i as isize) {
            if heightmap.is_low_point(x, y) {
                println!("Low point at {}, {}", x, y);
                result_part_1 += heightmap.point_at(x, y) as u32 + 1;

                let basin_size =
                    heightmap.calculate_basin_size(&mut visited, x as usize, y as usize);
                basins.push(basin_size);
                basin_sum += basin_size;
            }
        }
    }

    heightmap.print_out_with_visited(&visited);

    basins.sort_by(|a, b| b.cmp(a));
    let result_part_2: u32 = basins.iter().take(3).product();

    println!("The result for part 1 is: {}", result_part_1);
    println!("The result for part 2 is: {}", result_part_2);

    println!("Basins: {:?}", basins);
    println!("Basin sum: {}", basin_sum);

    let heightmap_sum = heightmap
        .values
        .iter()
        .fold(0 as u32, |a, b| a as u32 + *b as u32);
    let nines_sum = heightmap
        .values
        .iter()
        .filter(|v| **v == 9)
        .fold(0 as u32, |a, b| a as u32 + *b as u32);
    let other_sum = heightmap_sum - nines_sum;

    println!("Heightmap sum: {}", heightmap_sum);
    println!("Nines sum: {}", nines_sum);
    println!("Other sum: {}", other_sum);

    Ok(())
}
