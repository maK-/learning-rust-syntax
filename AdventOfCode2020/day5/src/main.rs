use std::path::Path;
use std::env;
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashSet;

fn main(){
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let data = return_data_vec(filename);

    println!("Highest ID: {:?}", one(&data));
    println!("Your Seat: {:?}", two(&data));
}

struct BoardingPass{
    seat_id: u32
}

impl BoardingPass{
    fn parse(data: String) -> BoardingPass{
        let row_id = data.chars()
                       .take(7)
                       .fold(0, |index, c| index * 2 + "B".contains(c) as u32);
        let col_id = data[data.len() - 3..]
                     .chars()
                     .fold(0, |index, c| index * 2 + "R".contains(c) as u32);
        BoardingPass{
            seat_id: row_id * 8 + col_id
        }
    }
}

fn return_data_vec(filename: impl AsRef<Path>) -> Vec<BoardingPass>{
    
    let mut file = File::open(filename).expect("No file specified!");
    let mut data = String::new();
    file.read_to_string(&mut data).expect("Unable to read the file");
    
    data.lines().map(|i| BoardingPass::parse(i.to_string())).collect()
}

fn one(data: &Vec<BoardingPass>) -> u32{
    let mut highest_id: u32 = 0;
    for i in data{
        if i.seat_id > highest_id{
            highest_id = i.seat_id
        }
    }
    highest_id
}

fn two(data: &Vec<BoardingPass>) -> u32{
    let filled_seats: HashSet<u32> = data.iter().map(|i| i.seat_id).collect();
    let min: u32 = *filled_seats.iter().min().unwrap();
    let max: u32 = *filled_seats.iter().max().unwrap();
    
    let all_seats: HashSet<u32> = (min..max+1).collect();
    let mut remaining = all_seats.difference(&filled_seats);
    
    *remaining.next().unwrap()
}
