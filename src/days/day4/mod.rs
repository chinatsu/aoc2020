pub mod parser;

const ECLS: [&str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

fn solve_one(input: &PassportQueue) -> u32 {
    input.iter().fold(0, |count, pass| count + pass.is_present() as u32)
}

fn solve_two(input: &PassportQueue) -> u32 {
    input.iter().fold(0, |count, pass| count + pass.is_valid() as u32)
}

#[derive(Debug, PartialEq)]
pub struct Passport {
    byr: String,
    iyr: String,
    eyr: String,
    hgt: String,
    hcl: String,
    ecl: String,
    pid: String
}

impl Passport {
    pub fn is_present(&self) -> bool {
        self.byr.len() > 0
        && self.iyr.len() > 0
        && self.eyr.len() > 0
        && self.hgt.len() > 0
        && self.hcl.len() > 0
        && self.ecl.len() > 0
        && self.pid.len() > 0
    }

    pub fn is_valid(&self) -> bool {
        let byr = self.byr.parse::<u32>().unwrap_or(0);
        let iyr = self.iyr.parse::<u32>().unwrap_or(0);
        let eyr = self.eyr.parse::<u32>().unwrap_or(0); 

        let valid_byr = byr >= 1920 && byr <= 2002;
        let valid_iyr = iyr >= 2010 && iyr <= 2020;
        let valid_eyr = eyr >= 2020 && eyr <= 2030;
        let valid_hgt = valid_hgt(&self.hgt);
        
        let mut hcl_chars = self.hcl.chars();
        let valid_hcl = self.hcl.len() == 7 && hcl_chars.next() == Some('#') && hcl_chars.all(|c| c.is_digit(16));
        
        let valid_ecl = ECLS.contains(&self.ecl.as_str());
        let valid_pid = self.pid.len() == 9 && self.pid.chars().all(|c| c.is_digit(10));
        
        valid_byr && valid_iyr && valid_eyr && valid_hgt && valid_hcl && valid_ecl && valid_pid
    }
}

fn valid_hgt(hgt: &String) -> bool {
    let mut valid = false;
    if hgt.contains("in") {
        let end = hgt.find("in").unwrap();
        let height = hgt[0..end].parse::<u32>().unwrap_or(0);
        valid = height >= 59 && height <= 76;
    } else if hgt.contains("cm") {
        let end = hgt.find("cm").unwrap();
        let height = hgt[0..end].parse::<u32>().unwrap_or(0);
        valid = height >= 150 && height <= 193;
    }
    valid
}

pub type PassportQueue = Vec<Passport>;

pub fn one(input: &PassportQueue) -> String {
    let answer = solve_one(input);
    format!("Day 4-1:  {}", answer)
}


pub fn two(input: &PassportQueue) -> String {
    let answer = solve_two(input);
    format!("Day 4-2:  {}", answer)
}

#[test]
pub fn solve_one_test() {
    assert_eq!(2, solve_one(&parser::parse("test")))
}


#[test]
pub fn solve_two_test() {
    assert_eq!(2, solve_two(&parser::parse("test")))
}

#[test]
pub fn regression() {
    let input = parser::parse("input");
    assert_eq!(256, solve_one(&input));
    assert_eq!(198, solve_two(&input));
}
