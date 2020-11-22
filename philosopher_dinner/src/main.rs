use std::thread;
use std::time;
use std::sync::Mutex;
use std::sync::Arc;

struct Philosopher {
    name: String,
    left: usize,
    right: usize,
}

struct Table {
    forks: Vec<Mutex <()>>,
}

impl Philosopher {
    fn new (name: & str, left: usize, right: usize) -> Philosopher {
        Philosopher {
            name: name.to_string(),
            left: left,
            right: right,
        }
    }

    fn eat (& self, table: & Table){
        let _left = table.forks[self.left].lock().unwrap();
        let _right = table.forks[self.right].lock().unwrap();
        println!("{} is eating!", self.name);
        let second_1 = time::Duration::from_millis(1000);
        thread::sleep(second_1);
        println!("{} has finished eating!", self.name);
    }
}

fn main() {

    let table = Arc::new(Table {forks: vec! [
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
    ]});


    let philosophers = vec! [
        Philosopher::new("maK", 0, 1),
        Philosopher::new("Karl Marx", 1, 2),
        Philosopher::new("Ciaran McNally", 2, 3),
        Philosopher::new("Eimear Tyrell", 3, 4),
        Philosopher::new("beimear", 0, 4),
    ];

    let handles: Vec<_> = philosophers.into_iter().map(| f | {
            
            let table = table.clone();
            thread::spawn(move || {
                f.eat(& table);
            })
    }).collect();

    for h in handles {
        h.join().unwrap();
    }
}

