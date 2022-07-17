pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;

use anyhow::Result;

fn main() -> Result<()> {
    println!("===== Day 01 =====");
    day01::main()?;
    println!("\n===== Day 02 =====");
    day02::main()?;
    println!("\n===== Day 03 =====");
    day03::main()?;
    println!("\n===== Day 04 =====");
    day04::main()?;
    println!("\n===== Day 05 =====");
    day05::main()?;
    println!("\n===== Day 06 =====");
    day06::main()?;
    println!("\n===== Day 07 =====");
    day07::main_first();
    println!("\n===== Day 08 =====");
    day08::main();
    println!("\n===== Day 09 =====");
    day09::main()
}
