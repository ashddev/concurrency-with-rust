use std::sync::Mutex;

static SHARED: Mutex<i32> = Mutex::new(0);

fn main() {
    // Exiting the scope implicitly unlocks the lock
    // {
    //     let lock = SHARED.lock().unwrap();
    // }

    let lock = SHARED.lock().unwrap();
    
    // Or explicitly unlock the lock by dropping it
    // std::mem::drop(lock);

    if let Ok(_lock) = SHARED.try_lock() {
        println!("Got the lock")
    } else {
        println!("Not got the lock")
    }
}
