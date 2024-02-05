mod led;

// use rppal;
use std::{env, time, thread};

fn main() {
    let mut start = false;
    println!("{}", env::consts::OS);

    led::init_LED();
    // Run loop check for button press to start
    while !start {
        println!("Waiting for start press");
        thread::sleep(time::Duration::from_millis(250));
    }
}
