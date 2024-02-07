mod led;

// use rppal;
use std::{env, time, thread, process};
use text_io::read;
fn main() {
    let mut start = false;
    println!("{}", env::consts::OS);

    led::init_led();

    // Run loop check for button press to start
    loop {
        wait_loop();

        // Button pressed start 1 hour timer
        println!("Timer has started...");
        // Start LED at blue
        led::led_adjust(0, 0, 255);

        // TODO LED start turning to red every hour

        // One hour limit reached
        led::led_adjust(255, 0, 0)
    }

}

fn wait_loop(){
    println!("Waiting for start press");
    thread::sleep(time::Duration::from_millis(250));
    // TODO Check for button press
    loop {
        let input: String = read!("{}\n");
        let trimmed_input = input.trim();
        match trimmed_input {
            "start" => return,
            "exit" => process::exit(1),
            (_) => println!("We got this: {input}")
        }
    }
}
