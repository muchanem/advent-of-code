use crate::utils::*;
pub mod utils;
pub mod day1;

fn main() {
    day_one()
}

fn day_one() {
    use crate::day1::*;

    let input: String = read_file("src/day1/day1.input");
    let sum: u32 = input.split("\n").map(|x| line_to_cal(x)).sum();
    println!("{}",sum)
}