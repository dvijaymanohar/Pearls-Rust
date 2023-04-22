#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused)]
use serialport::{DataBits, StopBits};
use std::io::{self, Write};
use std::time::Duration;

fn main() {
    let port_name = "/dev/ttyUSB0";
    let baud_rate = 115_200;

    let stop_bits = StopBits::One;
    let data_bits = DataBits::Eight;

    let builder = serialport::new(port_name, baud_rate)
        .stop_bits(stop_bits)
        .data_bits(data_bits);

    println!("{:?}", &builder);

    let mut port = builder.open().unwrap_or_else(|e| {
        eprintln!("Failed to open \"{}\". Error: {}", port_name, e);
        ::std::process::exit(1);
    });

    let str = "Version\r\n".to_string();

    match port.write(str.as_bytes()) {
        Ok(_) => {
            print!("{}", &str);
            std::io::stdout().flush().unwrap();
        }
        Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
        Err(e) => eprintln!("{:?}", e),
    }

    // Duration::from_millis only returns a Duration and does not sleep
    std::thread::sleep(Duration::from_millis(100));

    let mut serial_buf: Vec<u8> = vec![0; 1000];

    match port.read(serial_buf.as_mut_slice()) {
        Ok(t) => io::stdout().write_all(&serial_buf[..t]).unwrap(),
        Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
        Err(e) => eprintln!("{:?}", e),
    }
}
