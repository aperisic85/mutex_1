use std::{sync::Mutex, thread, time::Duration};

fn main() {
    let m = Mutex::new(0);
    thread::scope(|s| {
        for _ in 0..10 {
            s.spawn(|| {
                //println!("  spawned: {:?}",thread::current().id());
                let mut guard = m.lock().unwrap();
                for _ in 0..10000 {
                    *guard += 1;
                }
                drop(guard); //with explicit drop of guard, all is done in 1 sec.
                thread::sleep(Duration::from_secs(1));
            }); // guard is droped here
        }
    });
    println!("m: {}", &m.into_inner().unwrap());
}
