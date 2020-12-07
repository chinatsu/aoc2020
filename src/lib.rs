#![feature(str_split_once)]
#[macro_use] pub mod days;
use days::{day1, day2, day3, day4, day5, day6, day7};

pub fn main() -> String {
    let mut input_1 = day1::parser::parse(day1::INPUT);
    let mut res = format!("{}\n", day1::one(&mut input_1));
    res = format!("{}{}\n", res, day1::two(&mut input_1));
    let input_2 = day2::parser::parse(day2::INPUT);
    res = format!("{}{}\n", res, day2::one(&input_2));
    res = format!("{}{}\n", res, day2::two(&input_2));
    let input_3 = day3::parser::parse(day3::INPUT);
    res = format!("{}{}\n", res, day3::one(&input_3));
    res = format!("{}{}\n", res, day3::two(&input_3));
    let input_4 = day4::parser::parse(day4::INPUT);
    res = format!("{}{}\n", res, day4::one(&input_4));
    res = format!("{}{}\n", res, day4::two(&input_4));
    let input_5 = day5::parser::parse(day5::INPUT);
    res = format!("{}{}\n", res, day5::one(&input_5));
    res = format!("{}{}\n", res, day5::two(&input_5));
    let input_6 = day6::parser::parse(day6::INPUT);
    res = format!("{}{}\n", res, day6::one(&input_6));
    res = format!("{}{}\n", res, day6::two(&input_6));

    let input_7 = day7::parser::parse(day7::INPUT);
    res = format!("{}{}\n", res, day7::one(&input_7));
    res = format!("{}{}\n", res, day7::two(&input_7));
    res
}
