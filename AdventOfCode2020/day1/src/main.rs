use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let data = return_data_vec(filename);
    println!("{:?}", one(&data));
    println!("{:?}", two(&data));

}

fn return_data_vec(filename: impl AsRef<Path>) -> Vec<u32> {
    
    let mut data = Vec::<u32>::new();

    let file = File::open(filename).expect("No file specified!");
    let buf = BufReader::new(file);
    for line in buf.lines(){
        let line = &line.unwrap();
        let int = match line.parse::<u32>(){
            Ok(e) => e,
            Err(_) => continue,
        };
        data.push(int);
    }
    return data;
}

fn one(data: &Vec<u32>) -> u32{
    let mut result = 0;
    for i in data{
        if let Some(j) = data.iter().find(|&j| i + j == 2020){
            result = i * j;
            break;
        }

    }
    result
}

fn two(data: &Vec<u32>) -> u32{
    let mut result2 = 0;
    for (i, a) in data.iter().enumerate() {
        for (j, b) in data[i..].iter().enumerate() {
            for c in data[i + j..].iter() {
                if a + b + c == 2020 {
                    result2 = a * b * c
                }
            }
        }
    }
    result2
}

