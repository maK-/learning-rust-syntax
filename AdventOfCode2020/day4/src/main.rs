use std::path::Path;
use std::env;
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashSet;

fn main(){
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let data = return_data_vec(filename);

    println!("Total valid Passports: {:?}", validate(&data));
}

struct Passport{
    is_valid: bool
}

impl Passport{
    fn parse(data: String) -> Passport{
        let required_keys: HashSet<String> = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
                                             .iter()
                                             .copied()
                                             .map(String::from)
                                             .collect();
        
        let kv_fields: HashSet<(String, String)> = data
                                        .split_whitespace()
                                        .map(|i| Passport::return_kv(i.to_string()))
                                        .collect();
        
        let key_fields: HashSet<String> = kv_fields
                                        .iter()
                                        .map(|i| i.0.to_string())
                                        .collect();    
    
        if required_keys.is_subset(&key_fields){
            // Comment between the lines for part1
            //----------------------------------------
         
            if Passport::validation(&kv_fields){
            //----------------------------------------
                Passport{
                    is_valid: true
                }
            
            //----------------------------------------
            }
            else{
                Passport{
                    is_valid: false
                }
            }
            //----------------------------------------
        }
        else{
            Passport {
                is_valid: false
            }
        }
    }
    fn return_kv(token: String) -> (String, String){
        let key = token.split(":").next().unwrap();
        let value = token.split(":").last().unwrap();
        
        (key.to_string(), value.to_string())
    }

    // Return true if it passes all validation steps
    fn validation(kv: &HashSet<(String, String)>) -> bool{
        for (k, v) in kv{
            if k == "byr"{
                if (1920..=2002).contains(&v.parse::<i32>().unwrap()){
                    continue;
                }
                else{ return false; }
            }
            else if k == "iyr"{
                if (2010..=2020).contains(&v.parse::<i32>().unwrap()){
                    continue;
                }
                else{ return false; }
            }
            else if k == "eyr"{
                if (2020..=2030).contains(&v.parse::<i32>().unwrap()){
                    continue;
                }
                else{ return false; }
            }
            else if k == "hgt"{
                let height = &v[0..v.len() - 2].parse().unwrap_or(0);
                let unit = &v[v.len()-2..];
                match unit {
                    "cm" => { 
                                if (150..=193).contains(height){
                                    continue;
                                }
                                else{ return false; }
                    },
                    "in" => {
                                if (59..=76).contains(height){
                                    continue;
                                }
                                else{ return false; }
                    },
                    _ => return false
                }
            }
            else if k == "hcl"{
                if v.len() == 7 && v.starts_with("#") && v[1..v.len()].chars().all(|i| i.is_ascii_hexdigit()){
                    continue;
                }
                else{ return false; }
            }
            else if k == "ecl"{
                if ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&v.as_str()){
                    continue;
                }
                else{ return false; }
            }
            else if k == "pid"{
                if v.len() == 9 && v.chars().all(char::is_numeric){
                    continue;
                }
                else{ return false; }
            }
            else{
                continue;
            }
        }
        return true;
    }
}

fn return_data_vec(filename: impl AsRef<Path>) -> Vec<Passport>{
    
    let mut file = File::open(filename).expect("No file specified!");
    let mut data = String::new();
    file.read_to_string(&mut data).expect("Unable to read the file");
    
    data.split("\n\n")
        .map(|i| Passport::parse(i.to_string()))
        .collect()
}

//Return count based on valid entries
fn validate(passport: &Vec<Passport>) -> i32{
    let mut count = 0;

    for p in passport{
        if p.is_valid == true{
            count = count + 1
        }
    }    
    count
}

