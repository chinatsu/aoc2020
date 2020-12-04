pub mod parser;

fn solve_one(input: &OuterType) -> u32 {
    input.iter().fold(0, |count, pass| count + pass.is_present() as u32)
}

fn solve_two(input: &OuterType) -> u32 {
    input.iter().fold(0, |count, pass| count + pass.is_valid() as u32)
}

#[derive(Debug, PartialEq)]
pub struct InnerType {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>
}

impl InnerType {
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
        // maybe i should invite regex over for tea some time
        if self.byr.is_some() {
            let byr = self.byr.as_ref().unwrap();
            if byr.len() != 4 {
                
                return false
            }
            match byr.parse::<u32>() {
                Ok(v) => {
                    if v < 1920 || v > 2002 {
                        return false
                    }
                },
                Err(_) => return false
            }
        } else {
            return false
        }
        if self.iyr.is_some() {
            let iyr = self.iyr.as_ref().unwrap();
            if iyr.len() != 4 {
                return false
            }
            match iyr.parse::<u32>() {
                Ok(v) => {
                    if v < 2010 || v > 2020 {
                        return false         
                    }
                },
                Err(_) => return false
            }
        } else {
            return false
        }
        if self.eyr.is_some() {
            let eyr = self.eyr.as_ref().unwrap();
            if eyr.len() != 4 {
            }
            match eyr.parse::<u32>() {
                Ok(v) => {
                    if v < 2020 || v > 2030 {
                        return false
                    }
                },
                Err(_) => return false
            }
        } else {
            return false
        }
        if self.hgt.is_some() {
            let hgt = self.hgt.as_ref().unwrap();
            if hgt.contains("in") {
                let end = hgt.find("in").unwrap();
                match &hgt[0..end].parse::<u32>() {
                    Ok(v) => {
                        if v < &59 || v > &76 {
                            return false
                        }
                    }
                    Err(_) => return false
                }
            } else if hgt.contains("cm") {
                let end = hgt.find("cm").unwrap();
                match &hgt[0..end].parse::<u32>() {
                    Ok(v) => {
                        if v < &150 || v > &193 {
                            return false
                        }
                    }
                    Err(_) => return false
                }
            } else {
                return false
            }
        } else {
            return false
        }
        if self.hcl.is_some() {
            let hcl = self.hcl.as_ref().unwrap();
            if hcl.len() != 7 {
                return false
            }
            if hcl.chars().next() != Some('#') {
                return false
            } else if hcl.chars().skip(1).any(|c| !c.is_digit(16)) {
                return false
            }
        } else {
            return false
        }
        if self.ecl.is_some() {
            if !vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&self.ecl.as_ref().unwrap().as_str()) {
                return false
            }
        } else {
            return false
        }
        if self.pid.is_some() {
            let pid = self.pid.as_ref().unwrap();
            if pid.len() != 9 {
                return false
            }
            if pid.chars().any(|c| !c.is_digit(10)) {
                return false
            }
        } else {
            return false
        }
        true
    }
}

pub type OuterType = Vec<InnerType>;

pub fn one(input: &OuterType) -> String {
    let answer = solve_one(input);
    format!("Day 4-1:  {}", answer)
}


pub fn two(input: &OuterType) -> String {
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
