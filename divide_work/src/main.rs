fn main() {
    const NUM_THREADS: usize = 8;

    let to_add: Vec<u32> = (0..10000).collect();
    let mut thread_handlers = Vec::new();
    let chunks = to_add.chunks(NUM_THREADS);

    for chunk in chunks {
        let current_chunk = chunk.to_owned();
        let thread = std::thread::spawn(move || current_chunk.into_iter().sum::<u32>());
        thread_handlers.push(thread);
    }

    let mut sum = 0;
    thread_handlers.into_iter().for_each(|th| {
        sum += th.join().unwrap();
    });

    println!("Sum is {sum}")
}
