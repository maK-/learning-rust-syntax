use std::path::Path;
use std::env;
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashSet;

fn main(){
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    
    let data = return_data_vec(filename);

    println!("Part 1 - Total valid Passports: {:?}", one(&data));
    //println!("{:?}", two(&pw_policies));
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
        
        let key_fields: HashSet<String> = data
                                        .split_whitespace()
                                        .map(|i| Passport::return_key(i.to_string()))
                                        .collect();
        
        
        if required_keys.is_subset(&key_fields){
            Passport{
                is_valid: true
            }
        }
        else{
            Passport{
                is_valid: false
            }
        }
    }
    fn return_key(token: String) -> String{
        let key = token.split(":").next().unwrap();
        key.to_string()
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
fn one(passport: &Vec<Passport>) -> i32{
    let mut count = 0;

    for p in passport{
        if p.is_valid == true{
            count = count + 1
        }
    }    
    count
}

