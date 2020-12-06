#![feature(str_split_once)]
use std::io::{BufReader};
use std::fs::File;
use std::io::prelude::*;
use std::vec::Vec;
use std::error::Error;
use std::borrow::Cow;

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
        if self.byr.is_none() {
            return false;
        }
        if self.iyr.is_none() {
            return false;
        }
        if self.eyr.is_none() {
            return false;
        }
        if self.hgt.is_none() {
            return false;
        }
        if self.hcl.is_none() {
            return false;
        }
        if self.ecl.is_none() {
            return false;
        }
        if self.pid.is_none() {
            return false;
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
        if data.valid() {
            valid += 1;
        }
    }
    println!("Valid count: {}", valid);
}
