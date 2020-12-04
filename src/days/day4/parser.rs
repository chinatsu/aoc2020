use super::{PassportQueue, Passport};
use std::io::Read;

pub fn parse(filename: &str) -> PassportQueue {
    let mut file = std::fs::File::open(format!("src/days/day4/resources/{}.txt", filename)).unwrap();
    let mut buffer = Vec::new();
    let mut stringbuffer: Vec<String> = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    let mut cur_string: String = String::new();
    for line in buffer.split(|b| b == &10) {
        let string_line = std::str::from_utf8(&line).unwrap();
        if line.len() == 0 {
            stringbuffer.push(cur_string);
            cur_string = String::new();
        }
        cur_string = format!("{} {}", cur_string, string_line);
    }
    stringbuffer.push(cur_string);
    stringbuffer.iter().map(|val| {
        parse_entry(val)
    }).collect::<PassportQueue>()
}

fn parse_entry(entries: &String) -> Passport {
    let entries = entries.split(' ').map(|v| v.split(':').collect()).collect::<Vec<Vec<&str>>>();
    let mut ret = Passport {
        byr: None,
        iyr: None,
        eyr: None,
        hgt: None,
        hcl: None,
        ecl: None,
        pid: None,
        cid: None
    };
    for entry in entries {
        match entry[0] {
            "byr" => { ret.byr = Some(entry[1].to_string()) },
            "iyr" => { ret.iyr = Some(entry[1].to_string()) },
            "eyr" => { ret.eyr = Some(entry[1].to_string()) },
            "hgt" => { ret.hgt = Some(entry[1].to_string()) },
            "hcl" => { ret.hcl = Some(entry[1].to_string()) },
            "ecl" => { ret.ecl = Some(entry[1].to_string()) },
            "pid" => { ret.pid = Some(entry[1].to_string()) },
            "cid" => { ret.cid = Some(entry[1].to_string()) },
            _ => {}
        }
    }
    ret
}

#[test]
fn parse_test() {
    let input = parse("test");
    assert_eq!(Some("1937".to_string()), input[0].byr);
    assert_eq!(Some("166559648".to_string()), input[3].pid);
}

