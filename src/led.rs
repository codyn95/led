// use rppal;
use console::style;
pub fn init_led() {
    // Turn on the led
    println!("{}", style("LED is Blue").blue());
}

pub fn led_adjust(mut r_value: u32, mut g_value: u32, mut b_value: u32) {
    // Cap min/max value
    if r_value > 255 {r_value = 255};
    if g_value > 255 {g_value = 255};
    if b_value > 255 {b_value = 255};
    if r_value < 0 {r_value = 0};
    if g_value < 0 {g_value = 0};
    if b_value < 0 {b_value = 0};
    
    // Adjust pwm according to value
    let r_hex = format!("{r_value:#04X}");
    let g_hex = format!("{g_value:#04X}");
    let b_hex = format!("{b_value:#04X}");
    println!("Color being adjusted to: {} {} {}", style(r_hex).red(), style(g_hex).green(), style(b_hex).blue());

    // TODO Output to pins to adjust LED color
}

