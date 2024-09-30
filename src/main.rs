fn hello_thread(i: i16) {
    println!("Hello from thread {i}");
}

fn do_math(i: i16) -> i16 {
    let mut i = i+1;
    for _ in 0..10 {
        i+=10;
    }
    i
}

fn main() {
    println!("Hello from main thread");

    let mut thread_handles = Vec::new();

    for i in 0..8 {
        let thread_handle = std::thread::spawn(move || do_math(i));
        thread_handles.push(thread_handle);
    }

    thread_handles.into_iter().for_each(|th| {
        println!("{}", th.join().unwrap())
    });
}
