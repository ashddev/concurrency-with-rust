use dashmap::DashMap;
use std::sync::LazyLock;

static CHARCOUNT: LazyLock<DashMap<char, u32>> = LazyLock::new(DashMap::new);

const NUM_THREADS: usize = 8;

fn main() {
    let mut thread_handlers = Vec::new();

    let contents = std::fs::read_to_string("poem.txt")
        .expect("Could not read the file");

    let binding = contents
    .chars()
    .collect::<Vec<char>>();

    let chunk_size = contents.len() / NUM_THREADS;

    let chunks = binding
    .chunks(chunk_size);

    for chunk in chunks {
        let current_chunk = chunk.to_owned();
        let thread = std::thread::spawn(move || {
            for c in current_chunk {
                CHARCOUNT.entry(c).and_modify(|count| *count += 1).or_insert(1);
            }
        });
        
        thread_handlers.push(thread);
    }

    for thread in thread_handlers {
        thread.join().unwrap();
    }
    println!("{CHARCOUNT:#?}");
}
