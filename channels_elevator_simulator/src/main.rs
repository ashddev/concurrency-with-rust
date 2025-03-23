use std::sync::mpsc;

struct ElevatorRequest {
    current: i8,
    destination: i8,
}

fn main() {
    let (transmitter, reciever) = mpsc::channel::<ElevatorRequest>();

     let elevator_controller_handle = std::thread::spawn(move || {
        while let Ok(request) = reciever.recv() {
            println!("Current floor: {}", request.current);
            println!("Moving to floor: {}", request.destination)
        }
     });

     for i in 1..8 {
        let _ = transmitter.send(ElevatorRequest { current: i, destination: i+1 });
     }
     elevator_controller_handle.join().unwrap();
}
