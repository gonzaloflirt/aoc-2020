use anyhow::Result;
use regex::Regex;
use std::collections::HashMap;

static INPUT_FILE_NAME: &str = "input";

fn main() -> Result<()> {
    let input = read_input()?;
    println!("first: {}", input.iter().filter(|d| is_passport(d)).count());
    println!("second: {}", input.iter().filter(|d| is_valid(d)).count());

    Ok(())
}

fn is_passport(data: &HashMap<String, String>) -> bool {
    data.contains_key("byr")
        && data.contains_key("iyr")
        && data.contains_key("eyr")
        && data.contains_key("hgt")
        && data.contains_key("hcl")
        && data.contains_key("ecl")
        && data.contains_key("pid")
}

fn is_valid(data: &HashMap<String, String>) -> bool {
    check_byr(data)
        && check_iyr(data)
        && check_eyr(data)
        && check_hgt(data)
        && check_hcl(data)
        && check_ecl(data)
        && check_pid(data)
}

fn check_byr(data: &HashMap<String, String>) -> bool {
    check_number(data, "byr", 1920, 2002)
}

fn check_iyr(data: &HashMap<String, String>) -> bool {
    check_number(data, "iyr", 2010, 2020)
}

fn check_eyr(data: &HashMap<String, String>) -> bool {
    check_number(data, "eyr", 2020, 2030)
}

fn check_number(data: &HashMap<String, String>, key: &str, min: u64, max: u64) -> bool {
    if let Some(d) = data.get(key) {
        if d.len() == 4 {
            if let Ok(d) = d.parse::<u64>() {
                return (min..max + 1).contains(&d);
            }
        }
    }
    false
}

fn check_hgt(data: &HashMap<String, String>) -> bool {
    if let Some(d) = data.get("hgt") {
        let cm: &[_] = &['c', 'm'];
        if let Ok(d) = d.trim_end_matches(cm).parse::<u64>() {
            return (150..194).contains(&d);
        }
        let inch: &[_] = &['i', 'n'];
        if let Ok(d) = d.trim_end_matches(inch).parse::<u64>() {
            return (59..77).contains(&d);
        }
    }
    false
}

fn check_hcl(data: &HashMap<String, String>) -> bool {
    if let Some(d) = data.get("hcl") {
        return Regex::new("^#[\\d|a-f]{6}$").unwrap().is_match(d.as_str());
    }
    false
}

fn check_ecl(data: &HashMap<String, String>) -> bool {
    if let Some(d) = data.get("ecl") {
        return Regex::new("^amb|blu|brn|gry|grn|hzl|oth$")
            .unwrap()
            .is_match(d);
    }
    false
}

fn check_pid(data: &HashMap<String, String>) -> bool {
    if let Some(d) = data.get("pid") {
        return Regex::new("^\\d{9}$").unwrap().is_match(d.as_str());
    }
    false
}

fn read_input() -> Result<Vec<HashMap<String, String>>> {
    use std::fs::File;
    use std::io::prelude::*;
    use std::io::BufReader;
    let mut credentials = vec![];
    let mut passport = HashMap::new();

    let file = File::open(INPUT_FILE_NAME)?;
    for line in BufReader::new(file).lines().filter_map(|l| l.ok()) {
        if line.is_empty() {
            credentials.push(passport);
            passport = HashMap::new();
        } else {
            for item in line.split(' ') {
                let mut data = item.split(':');
                passport.insert(data.next().unwrap().into(), data.next().unwrap().into());
            }
        }
    }
    Ok(credentials)
}
