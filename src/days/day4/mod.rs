pub mod parser;

fn solve_one(input: &PassportQueue) -> u32 {
    input.iter().fold(0, |count, pass| count + pass.is_present() as u32)
}

fn solve_two(input: &PassportQueue) -> u32 {
    input.iter().fold(0, |count, pass| count + pass.is_valid() as u32)
}

#[derive(Debug, PartialEq)]
pub struct Passport {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>
}

impl Passport {
    pub fn is_present(&self) -> bool {
        self.byr.is_some() 
        && self.iyr.is_some()
        && self.eyr.is_some()
        && self.hgt.is_some()
        && self.hcl.is_some()
        && self.ecl.is_some()
        && self.pid.is_some()
    }
    pub fn is_valid(&self) -> bool {
        if !self.is_present() {
            return false
        }
        let byr = self.byr.as_ref().unwrap().parse::<u32>().unwrap_or(0);
        let iyr = self.iyr.as_ref().unwrap().parse::<u32>().unwrap_or(0);
        let eyr = self.eyr.as_ref().unwrap().parse::<u32>().unwrap_or(0); 
        let hgt = self.hgt.as_ref().unwrap();
        let hcl = self.hcl.as_ref().unwrap();
        let ecl = self.ecl.as_ref().unwrap();
        let pid = self.pid.as_ref().unwrap();

        let valid_byr = byr >= 1920 && byr <= 2002;
        let valid_iyr = iyr >= 2010 && iyr <= 2020;
        let valid_eyr = eyr >= 2020 && eyr <= 2030;
        let valid_hgt = valid_hgt(hgt);
        let valid_hcl = hcl.len() == 7 && hcl.chars().next() == Some('#') && hcl.chars().skip(1).all(|c| c.is_digit(16));
        let valid_ecl = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&ecl.as_str());
        let valid_pid = pid.len() == 9 && pid.chars().all(|c| c.is_digit(10));
        
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
