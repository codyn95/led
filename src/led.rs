// use rppal;
use console::style;
pub fn init_led() {
    // Turn on the led
    println!("{}", style("LED is RED").red());
}

pub fn led_adjust(r_value: u32, g_value: u32, b_value: u32) {
    // Adjust pwm according to value
    let r_hex = format!("{r_value:#04X}");
    let g_hex = format!("{g_value:#04X}");
    let b_hex = format!("{b_value:#04X}");
    println!("Color being adjusted to: {} {} {}", style(r_hex).red(), style(g_hex).green(), style(b_hex).blue());

    // TODO Output to pins to adjust LED color
}

