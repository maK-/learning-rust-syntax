use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;
use std::env;

fn main(){
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let data = return_data_vec(filename);
    
    println!("Part 1: {:?}", traverse(&data, [(3,1)].to_vec()));
    println!("Part 2: {:?}", traverse(&data, [(1,1),(3,1),(5,1),(7,1),(1,2)].to_vec()));
}

fn return_data_vec(filename: impl AsRef<Path>) -> Vec<String>{
    
    let file = File::open(filename).expect("No file specified!");
    let buf = BufReader::new(file);
    
    buf.lines()
        .map(|i| i.expect("line parse failed!"))
        .collect()
} 

fn traverse(forest: &Vec<String>, directions: Vec<(usize, usize)>) -> i64{
    let max_x = forest[0].len();
    let max_y = forest.len();

    let mut total_value: i64 = 1;

    let mut curr_x = 0;
    let mut curr_y = 0;

    let mut tree_count = 0;

    for i in directions {

        println!("\nDirections: {:?}", i);
        for j in 0..max_y {
            let chars: Vec<char> = forest[j].chars().collect();

            if j == curr_y {
                if chars[curr_x] == '#' {
                    tree_count = tree_count + 1;
                }
            }
            else{
                continue;
            }

            curr_x = (curr_x + i.0) % max_x;
            curr_y = curr_y + i.1;
            if curr_y >= max_y{
                break;
            }
        }
        
        if tree_count != 0{
            total_value = total_value * tree_count;
        }
        
        tree_count = 0;
        curr_x = 0;
        curr_y = 0;
        println!("Total: {:?}\n", total_value);
    }
    total_value
}



