use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main(){
    println!("Guess the number!");
    let secret_n = rand::thread_rng().gen_range(1,101);
    println!("The secret nuymber is: {}", secret_n);

    loop{
        println!("Please enter your hunch.");
        let mut hunch = String::new();
        io::stdin().read_line(& mut hunch).ok().expect("Failed to read line!");

        let hunch: u32 = match hunch.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("Hunch: {}", hunch);
        
        match hunch.cmp(& secret_n){
            Ordering::Less=> println!("Too low"),
            Ordering::Greater=> println!("Too big"),
            Ordering::Equal=> {
                                println!("Guessed right!");
                                break;
            }
        }
    }
}
