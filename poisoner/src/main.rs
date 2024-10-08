use std::sync::Mutex;


static SHARED: Mutex<i32> = Mutex::new(0);

fn poisoned() {
    let mut lock = SHARED.lock().unwrap();
    *lock += 1;
    panic!("Get poisoned")
}

fn main() {
    
    let handle = std::thread::spawn(poisoned);
    println!("Trying to return from the thread");
    println!("{:?}", handle.join());

    let lock = SHARED.lock();
    println!("{lock:?}");

    // How to recover data from a poisoned mutex
    let recovered_data = lock.unwrap_or_else(|poisoned| {
        println!("Mutex was poisoned, recovering data...");
        poisoned.into_inner()
    });
    println!("{recovered_data:?}");
    
}
