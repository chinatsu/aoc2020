pub mod parser;

fn solve_one(entries: &mut Vec<i32>) -> Option<i32> {
    entries.sort_unstable();
    let mut l = 0;
    let mut r = entries.len() - 1;
    while l < r {
        let sum = entries[l] + entries[r];
        if sum > 2020 {
            r -= 1;
        } else if sum < 2020 {
            l += 1;
        } else {
            return Some(entries[l] * entries[r])
        }
    }
    None
}

fn solve_two(entries: &mut Vec<i32>) -> Option<i32> {
    entries.sort_unstable();
    for (i, &x) in entries.iter().enumerate() {
        let target = 2020 - x;
        let mut l = i + 1;
        let mut r = entries.len() - 1;
        while l < r {
            let sum = entries[l] + entries[r];
            if sum > target {
                r -= 1;
            } else if sum < target {
                l += 1;
            } else {
                return Some(x * entries[l] * entries[r])
            }
        }
    }
    None
}

pub fn one(mut entries: &mut Vec<i32>) -> String {
    let answer = match solve_one(&mut entries) {
        Some(solution) => format!("{}", solution),
        None => String::from("That's weird, something didn't add up")
    };
    format!("Day 1-1:  {}", answer)
}

pub fn two(mut entries: &mut Vec<i32>) -> String {
    let answer = match solve_two(&mut entries) {
        Some(solution) => format!("{}", solution),
        None => String::from("That's weird, something didn't add up")
    };    
    format!("Day 1-2:  {}", answer)

}

#[test]
pub fn solve_one_test() {
    let mut entries: Vec<i32> = vec![0, 0];
    assert_eq!(None, solve_one(&mut entries));

    entries = vec![1011, 1010, 313, 312];
    assert_eq!(None, solve_one(&mut entries));

    entries = vec![1010, 1010, 999, 333];
    assert_eq!(Some(1020100), solve_one(&mut entries));
}

#[test]
pub fn solve_two_test() {
    let mut entries: Vec<i32> = vec![0, 0, 0];
    assert_eq!(None, solve_two(&mut entries));

    entries = vec![1008, 4, 1337, 333];
    assert_eq!(None, solve_two(&mut entries));

    entries = vec![1008, 4, 1008, 333];
    assert_eq!(Some(4064256), solve_two(&mut entries));

    entries = vec![382, 300, 1337, 333];
    assert_eq!(None, solve_two(&mut entries));

    entries = vec![383, 300, 1337, 333];
    assert_eq!(Some(153621300), solve_two(&mut entries));
}

#[test]
pub fn regression() {
    let mut entries = parser::parse("input");
    assert_eq!(Some(326211), solve_one(&mut entries));
    assert_eq!(Some(131347190), solve_two(&mut entries));
}
