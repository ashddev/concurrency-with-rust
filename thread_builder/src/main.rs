use std::thread;

fn greeting() {
    println!("Hello from thread named {}", thread::current().name().unwrap())
}

fn main() {
    thread::Builder::new()
        .name("Test thread".to_string())
        .stack_size(std::mem::size_of::<usize>() * 4)
        .spawn(greeting)
        .unwrap();
}
