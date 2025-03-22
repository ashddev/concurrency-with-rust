use std::sync::{Arc, Mutex};

fn process_queue(queue_consumer: Arc<Mutex<Vec<String>>>) {
    loop {
        std::thread::park();
        let mut q = queue_consumer.lock().unwrap();
        
        while let Some(latest) = q.pop() {
            println!("{:?}", latest)
        }
    }
}

fn read_line() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn main() {
    let queue: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::<String>::new()));
    let queue_consumer: Arc<Mutex<Vec<String>>> = Arc::clone(&queue);

    let thread_handler = std::thread::spawn(move || {
        process_queue(queue_consumer);
    });

    println!("Add messages to the queue!");
    loop {
        let message = read_line();
        if message == "q" {
            break
        }

        let mut q = queue.lock().unwrap();
        q.push(message);
        println!("{:?}", q);

        if q.len() > 3 {
            thread_handler.thread().unpark();
        }
    }  
}
