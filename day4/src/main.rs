#![feature(str_split_once)]
use std::io::{BufReader};
use std::fs::File;
use std::io::prelude::*;
use std::vec::Vec;
use std::error::Error;
use regex::Regex;

#[derive(Debug, Clone)]
struct PassportData {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}

impl<'a> PassportData {
    /// Returns an empty PassportData
    fn new() -> PassportData {
        PassportData{
            byr: None,
            iyr: None,
            eyr: None,
            hgt: None,
            hcl: None,
            ecl: None,
            pid: None,
            cid: None,
        }
    }

    fn valid_fields(&self) -> Vec<&'a str> {
        vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid", "cid"]
    }

    fn valid_ecl(&self) -> Vec<&'a str> {
        vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
    }

    fn add_field(&mut self, kv_pair: String) -> Result<(), Box<dyn Error>> {
        // Problem
        let (k, v) = kv_pair.split_once(":").unwrap();

        if k == "byr" {
            self.byr = Some(v.to_string())
        } else if k == "iyr" {
            self.iyr = Some(v.to_string())
        } else if k == "eyr" {
            self.eyr = Some(v.to_string())
        } else if k == "hgt" {
            self.hgt = Some(v.to_string())
        } else if k == "hcl" {
            self.hcl = Some(v.to_string())
        } else if k == "ecl" {
            self.ecl = Some(v.to_string())
        } else if k == "pid" {
            self.pid = Some(v.to_string())
        } else if k == "cid" {
            self.cid = Some(v.to_string())
        }

        Ok(())
    }

    fn valid(&self) -> bool {
        let hgt_re: Regex = Regex::new(r"^\d{2,3}(cm|in)$").unwrap();
        let hcl_re: Regex = Regex::new(r"^\#[0-9a-z]{6}$").unwrap();
        let pid_re: Regex = Regex::new(r"[0-9]{9}").unwrap();

        if self.byr.is_none() {
            return false;
        } else {
            println!("BYR: {}", self.byr.clone().unwrap());
            let year: i32 = self.byr.clone().unwrap().parse().unwrap();
            if year < 1920 || year > 2002 {
                println!("BYR not valid");
                return false;
            }
        }
        if self.iyr.is_none() {
            return false;
        } else {
            let year: i32 = self.byr.clone().unwrap().parse().unwrap();
            println!("IYR: {}", self.byr.clone().unwrap());
            if year < 2010 || year > 2020 {
                println!("IYR not valid");
                return false;
            }
        }
        if self.eyr.is_none() {
            return false;
        } else {
            println!("EYR: {}", self.eyr.clone().unwrap());
            let year: i32 = self.byr.clone().unwrap().parse().unwrap();
            if year < 2020 || year > 2030 {
                println!("EYR not valid");
                return false;
            }
        }
        if self.hgt.is_none() {
            return false;
        } else {
            match hgt_re.captures(self.hgt.clone().unwrap().as_str()) {
                Some(mat) => {
                    let height: u32 = mat.get(0).unwrap().as_str().parse().unwrap();
                    let unit = mat.get(1).unwrap();

                    println!("Unit: {}, height: {}", unit.as_str(), height);

                    if unit.as_str() == "cm" {
                        if height < 149 || height > 193 {
                            return false;
                        }
                    } else {
                        if height < 59 || height > 76 {
                            return false;
                        }
                    }
                },
                None => {
                    return false;
                }
            }
        }
        if self.hcl.is_none() {
            return false;
        } else {
            if !hcl_re.is_match(self.hcl.clone().unwrap().as_str()) {
                return false;
            }
        }
        if self.ecl.is_none() {
            return false;
        } else {
            let color = self.hcl.clone().unwrap();
            if !self.valid_ecl().contains(&color.as_str()) {
                return false;
            }
        }
        if self.pid.is_none() {
            return false;
        } else {
            if !pid_re.is_match(self.pid.clone().unwrap().as_str()) {
                return false
            }
        }
        // if self.cid.is_none() {
        //     return false;
        // }

        return true;
    }
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let buf = BufReader::new(file);


    let mut datas = vec![PassportData::new()];

    for line in buf.lines() {
        let unwrapped = line.unwrap();
        println!("Line: {}", unwrapped);

        if unwrapped.trim().is_empty() {
            // We know that this is the end of a passport builder
            println!("This is an empty line!");
            datas.push(PassportData::new());
        } else {
            for kv in unwrapped.split(" ") {
                println!("Pushing kv: {}", kv);
                datas.last_mut().unwrap().add_field(kv.to_string()).unwrap();
            }
        }
    }
    let mut valid = 0;
    for data in datas {
        println!("Data: {:?}", data);
        if data.valid() {
            valid += 1;
        }
    }
    println!("Valid count: {}", valid);
}
