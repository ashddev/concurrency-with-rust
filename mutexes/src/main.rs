use std::{sync::Mutex, thread};


static NUMBERS: Mutex<Vec<i16>> = Mutex::new(Vec::new());

fn main() {
    let mut handlers = Vec::new();
    for i in 0..10 {
        let handler = thread::spawn(move || {
            let mut lock = NUMBERS.lock().unwrap();
            lock.push(i);
        });
        handlers.push(handler);
    }

    handlers.into_iter().for_each(|h| h.join().unwrap());

    let lock = NUMBERS.lock().unwrap();
    println!("{:#?}", lock)
}
