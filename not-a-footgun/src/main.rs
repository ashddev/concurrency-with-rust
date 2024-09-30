use std::{sync::atomic::AtomicI32, thread};

static COUNTER: AtomicI32 = AtomicI32::new(0);

fn main() {
    let mut handlers = Vec::new();
    for _ in 0..50_000 {
        handlers.push(thread::spawn(|| {
            COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        }));
    }

    handlers.into_iter().for_each(|h| h.join().unwrap());

    println!("{}", COUNTER.load(std::sync::atomic::Ordering::Relaxed));
}
