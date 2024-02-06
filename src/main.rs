use std::{sync::Mutex, thread};

fn main() {
    let m = Mutex::new(0);
    thread::scope(|s| {
        for _ in 0..10 {
            s.spawn(|| {
                //println!("  spawned: {:?}",thread::current().id());
                let mut guard = m.lock().unwrap();
                for _ in 0..10000 {
                    *guard +=1;
                    
                }
                println!("{:?} {}", thread::current().id(), *guard );
                
            }); // guard is droped here
        }
        println!("Done spawning");
    })
    
}
