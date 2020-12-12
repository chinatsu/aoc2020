#![feature(str_split_once)]
#[macro_use] pub mod days;
use days::{day1, day2, day3, day4, day5, day6, day7, day8, day9, day10, day11, day12};

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
    let input_8 = day8::parser::parse(day8::INPUT);
    res = format!("{}{}\n", res, day8::one(&input_8));
    res = format!("{}{}\n", res, day8::two(&input_8));
    let input_9 = day9::parser::parse(day9::INPUT);
    let answer_day9_1 = day9::solve_one(&input_9, 25);
    let answer_day9_2 = day9::solve_two(&input_9, answer_day9_1);
    res = format!("{}{}\n", res, day9::one(answer_day9_1));
    res = format!("{}{}\n", res, day9::two(answer_day9_2));
    let input_10 = day10::parser::parse(day10::INPUT);
    res = format!("{}{}\n", res, day10::one(&input_10));
    res = format!("{}{}\n", res, day10::two(&input_10));
    let input_11 = day11::parser::parse(day11::INPUT);
    res = format!("{}{}\n", res, day11::one(&input_11));
    res = format!("{}{}\n", res, day11::two(&input_11));
    let input_12 = day12::parser::parse(day12::INPUT);
    res = format!("{}{}\n", res, day12::day(&input_12));
    
    res
}
