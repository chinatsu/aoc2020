use aoc2020::lines_from;
use aoc2020::days::*;

pub fn main() {
    let mut entries = lines_from!("01", i32);
    println!("{}", day01_1(&mut entries, Sorting::Ascending));
    println!("{}", day01_2(&mut entries, Sorting::Ascending));
}
