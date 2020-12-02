use aoc2020::days::{day1, day2};

pub fn main() {
    let mut input_1 = day1::parser::parse("input");
    println!("{}", day1::one(&mut input_1));
    println!("{}", day1::two(&mut input_1));
    let input_2 = day2::parser::parse("input");
    println!("{}", day2::one(&input_2));
    println!("{}", day2::two(&input_2));
}
