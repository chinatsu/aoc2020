use aoc2020::days::{day1, day2, day3, day4, day5};

pub fn main() {
    let mut input_1 = day1::parser::parse(day1::INPUT);
    println!("{}", day1::one(&mut input_1));
    println!("{}", day1::two(&mut input_1));
    let input_2 = day2::parser::parse(day2::INPUT);
    println!("{}", day2::one(&input_2));
    println!("{}", day2::two(&input_2));
    let input_3 = day3::parser::parse(day3::INPUT);
    println!("{}", day3::one(&input_3));
    println!("{}", day3::two(&input_3));
    let input_4 = day4::parser::parse(day4::INPUT);
    println!("{}", day4::one(&input_4));
    println!("{}", day4::two(&input_4));
    let input_5 = day5::parser::parse(day5::INPUT);
    println!("{}", day5::one(&input_5));
    println!("{}", day5::two(&input_5));
}
