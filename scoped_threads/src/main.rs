use std::thread;

fn main() {
    const NUM_THREADS: usize = 8;
    let to_add: Vec<i32> = (0..10000).collect();
    let chunks = to_add.chunks(NUM_THREADS);

    let sum = thread::scope(|s| {
        let mut thread_handlers = Vec::new();

        for chunk in chunks {
            thread_handlers.push(s.spawn(move || chunk.iter().sum::<i32>()));
        }

        thread_handlers.into_iter().map(|h| h.join().unwrap()).sum::<i32>()
    });

    println!("Sum is {sum}")
}
