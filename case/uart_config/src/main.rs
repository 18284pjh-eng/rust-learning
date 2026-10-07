// uart config selector
// set baud frame class

fn classify_baud(baud: u32) -> &'static str {
    if baud <= 9600 {
        "slow"
    } else if baud <= 115200 {
        "normal"
    } else {
        "fast"
    }
}

fn is_supported_baud(baud: u32) -> bool {
    [9600, 19200, 57600, 115200].contains(&baud)
}

fn main() {
    println!("Hello, this is uart_config");

    let baud: u32 = 115_200;
    let data_bits: u8 = 8;
    let stop_bits: u8 = 1;
    let parity: &str = "N";

    let class: &str = classify_baud(baud);

    let frame = if data_bits == 8 && parity == "N" && stop_bits == 1 {
        "8N1"
    } else {
        "custom"
    };

    println!("baud = {baud}");

    println!("frame = {frame}");

    println!("class = {class}");

    for baud in [9600, 19200, 57600, 115200, 10000] {
        println!("{baud}: {}", is_supported_baud(baud));
    }
}
