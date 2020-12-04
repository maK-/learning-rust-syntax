use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;
use std::env;

fn main(){
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let data = return_data_vec(filename);
    let pw_policies: Vec<Password> = data.into_iter()
                                     .map(|i| Password::parse(i))
                                     .collect();

    println!("{:?}", one(&pw_policies));
    println!("{:?}", two(&pw_policies));
}

struct Password{
    min: usize,
    max: usize,
    chr: char,
    pass: String
}

impl Password{
    fn parse(line: String) -> Password{
        let data: Vec<String> = line.split(|i| i == ' '||i == '-')
                                .map(|i| i.to_string())
                                .collect();
        
        Password{
            min: data[0].parse().unwrap(),
            max: data[1].parse().unwrap(),
            chr: data[2].chars().next().unwrap(),
            pass: data[3].to_string()
        }
    }
}


fn return_data_vec(filename: impl AsRef<Path>) -> Vec<String>{
    
    let file = File::open(filename).expect("No file specified!");
    let buf = BufReader::new(file);
    
    buf.lines()
        .map(|i| i.expect("line parse failed!"))
        .collect()
}

//Return count based on valid entries
fn one(password: &Vec<Password>) -> i32{
    let mut count = 0;
    for p in password{
        let chars: String = p.pass
                            .chars()
                            .filter(|i| *i == p.chr)
                            .collect();
        if chars.len() >= p.min && chars.len() <= p.max {
            count = count + 1
        }
    }    
    count
}

//Return count based on exactly one positional char
fn two(password: &Vec<Password>) -> i32{
    let mut count = 0;
    for p in password{
        let mut p_count = 0;
        if p.pass.chars().nth(p.min - 1).unwrap() == p.chr{
            p_count = p_count + 1
        }
        if p.pass.chars().nth(p.max - 1).unwrap() == p.chr{
            p_count = p_count + 1
        }
        
        if p_count == 1{
            count = count + 1
        }
    }
    count
}
