pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;

use anyhow::Result;

fn main() -> Result<()> {
    day01::main()?;
    day02::main()?;
    day03::main()?;
    day04::main()?;
    day05::main()?;
    day06::main()?;
    day07::main()
}
