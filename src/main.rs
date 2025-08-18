use std::io::{Read, Write};
use std::time::Duration;

fn main() {
    let port_name = "/dev/ttyACM0";
    let baud_rate = 115200;
    let timeout = Duration::from_secs(1);
    let cmd = "M115\n";

    let mut buf = [0u8; 1024];
    let mut port = serialport::new(port_name, baud_rate)
        .timeout(timeout)
        .open()
        .unwrap();

    port.write_all(cmd.as_bytes()).unwrap();
    println!("sent {} to {}", cmd, port_name);

    let n = port.read(&mut buf).unwrap();
    println!("Received {} bytes:", n);
    println!("{}", String::from_utf8_lossy(&buf[..n]));
}
