use std::thread;

# [no_mangle]
pub extern fn process() {
    let handles: Vec<_> = (0..10).map(| _ | {
        thread::spawn(|| {
            let mut _x = 0;
            for _ in (0..5_000_000){
                _x = 1
            }
        })
    }).collect();

    for h in handles {
        println!("thread done");
        h.join().ok().expect("A thread could not be joined");
    }
}
