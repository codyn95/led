mod led;

// use rppal;
use std::{env, time, thread};
use text_io::read;
fn main() {
    let mut start = false;
    println!("{}", env::consts::OS);

    led::init_led();

    // Run loop check for button press to start
    while !start {
        println!("Waiting for start press");
        thread::sleep(time::Duration::from_millis(250));
        // TODO Check for button press and set start to true
        let input: String = read!("{}\n");
        let trimmed_input = input.trim();
        if trimmed_input == "start" {
            start = true;
        }
        else {
            println!("We got this {input}");
        }
    }

    // Button pressed start 1 hour timer
    println!("Timer has started...");
    // Start LED at blue
    led::led_adjust(0, 0, 255);

    // TODO LED start turning to red every hour
    
    // One hour limit reached
    start = false;

}
