use crate::utils::thats_weird;
use std::collections::HashSet;

pub enum Sorting {
    Unsorted,
    Descending,
    Ascending,
}

fn solve_day01_1(entries: &mut Vec<i32>) -> Option<i32> {
    let mut seen: HashSet<i32> = HashSet::new();
    for x in entries.iter() {
        let y = 2020 - x;
        if seen.contains(&y) {
            return Some(x * y)
        }
        seen.insert(*x);
    }
    None
}

fn solve_day01_2(entries: &mut Vec<i32>) -> Option<i32> {
    for i1 in 0..entries.len() {
        let mut seen: HashSet<i32> = HashSet::new();
        for i2 in i1+1..entries.len() {
            let x = entries[i1];
            let y = entries[i2];
            let z = 2020 - x - y;
            if seen.contains(&z) {
                return Some(x * y * z)
            }
            seen.insert(y);
        }
    }
    None
}

pub fn day01_1(mut entries: &mut Vec<i32>, sorting: Sorting) -> String {
    let answer = match sorting {
        Sorting::Unsorted => match solve_day01_1(&mut entries) {
            Some(solution) => format!("{}", solution),
            None => thats_weird()
        },
        Sorting::Ascending => {
            entries.sort_unstable();
            match solve_day01_1(&mut entries) {
                Some(solution) => format!("{}", solution),
               None => thats_weird()
            }
        },
        Sorting::Descending => {
            entries.sort_unstable_by(|a, b| b.cmp(a));
            match solve_day01_1(&mut entries) {
                Some(solution) => format!("{}", solution),
                None => thats_weird()
            }
        }
    };
    format!("Day 1-1:  {}", answer)
}

pub fn day01_2(mut entries: &mut Vec<i32>, sorting: Sorting) -> String {
    let answer = match sorting {
        Sorting::Unsorted => match solve_day01_2(&mut entries) {
            Some(solution) => format!("{}", solution),
            None => thats_weird()
        },
        Sorting::Ascending => {
            entries.sort_unstable();
            match solve_day01_2(&mut entries) {
                Some(solution) => format!("{}", solution),
                None => thats_weird()
            }
        },
        Sorting::Descending => {
            entries.sort_unstable_by(|a, b| b.cmp(a));
            match solve_day01_2(&mut entries) {
                Some(solution) => format!("{}", solution),
                None => thats_weird()
            }
        }
    };
    format!("Day 1-2:  {}", answer)

}

#[test]
pub fn solve_day01_test() {
    let mut entries: Vec<i32> = vec![0, 0];
    assert_eq!(None, solve_day01_1(&mut entries));

    entries = vec![1011, 1010, 313, 312];
    assert_eq!(None, solve_day01_1(&mut entries));

    entries = vec![1010, 1010, 999, 333];
    assert_eq!(Some(1020100), solve_day01_1(&mut entries));
}

#[test]
pub fn solve_day01_part2_test() {
    let mut entries: Vec<i32> = vec![0, 0, 0];
    assert_eq!(None, solve_day01_2(&mut entries));

    entries = vec![1008, 4, 1337, 333];
    assert_eq!(None, solve_day01_2(&mut entries));

    entries = vec![1008, 4, 1008, 333];
    assert_eq!(Some(4064256), solve_day01_2(&mut entries));

    entries = vec![382, 300, 1337, 333];
    assert_eq!(None, solve_day01_2(&mut entries));

    entries = vec![383, 300, 1337, 333];
    assert_eq!(Some(153621300), solve_day01_2(&mut entries));
}

#[test]
pub fn regression() {
    let mut entries = lines_from!("01", i32);
    assert_eq!(Some(326211), solve_day01_1(&mut entries));
    assert_eq!(Some(131347190), solve_day01_2(&mut entries));
}
