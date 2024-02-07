mod led;

// use rppal;
use std::{env, time::Duration, thread, process};
use text_io::read;

use crate::led::led_adjust;
fn main() {
    let mut start = false;
    let mut counter = 0;
    println!("{}", env::consts::OS);

    led::init_led();

    // Run loop check for button press to start
    loop {
        wait_loop();
        counter = 0;
        
        // Button pressed start 1 hour timer
        println!("Timer has started...");
        
        // Start LED at Green
        led::led_adjust(0, 255, 0);

        // LED start turning to red every hour
        let mut r_value = 0.0;
        let mut g_value = 255.0;
        let max_time = 3600;
        while counter < max_time {
            // At halfway time, LED is fully yellow
            // At one hour, LED is fully red
            if counter > max_time/2 {
                g_value -= 255.0/max_time as f32;
            }
            
            r_value += 255.0/(max_time as f32/2.0);
            println!("{}", r_value.to_string());
            led_adjust(r_value as u32, g_value as u32, 0);

            thread::sleep(Duration::from_secs(1));
            counter += 1;
        }

        // One hour limit reached
        led::led_adjust(255, 0, 0)
    }

}

fn wait_loop(){
    println!("Waiting for start press");
    thread::sleep(Duration::from_millis(250));
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
