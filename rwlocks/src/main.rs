use std::sync::{RwLock, LazyLock};

static USERS: LazyLock<RwLock<Vec<String>>> = LazyLock::new(|| RwLock::new(build_users()));

fn build_users() -> Vec<String> {
    vec!["Alice".to_string(), "Bob".to_string()]
}

fn read_line() -> String {
    let mut input: String = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn main() {
    std::thread::spawn(|| {
        loop {
            println!("Current users (in a thread)");
            let users = USERS.read().unwrap();
            println!("{users:?}");
            std::thread::sleep(std::time::Duration::from_secs(3));
        }
    });

    loop {
        println!("Enter name of new user (or q to quit)");
        let input = read_line();
        if input == "q" {
            break;
        } else {
            let mut lock = USERS.write().unwrap();
            lock.push(input);
        }
    }
}
