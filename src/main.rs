use serialport::{SerialPort, TTYPort};
use std::io::{Read, Write};
use std::time::Duration;

fn serial_starter(device: &str, baud: u32, timeout: Duration) -> TTYPort {
    let mut port: serialport::TTYPort = serialport::new(device, baud)
        .timeout(timeout)
        .open_native()
        .expect("couldn't open");

    port.write_data_terminal_ready(false).unwrap();
    port.write_request_to_send(true).unwrap();
    port
}
fn send_cmd(port: &mut TTYPort, cmd: &str) {
    match port.write_all(cmd.as_bytes()) {
        Ok(()) => println!("sent:{}", cmd),
        Err(e) => eprintln!("failed to send \n{} because of {}", cmd, e),
    }
    port.flush().ok();
}
fn recive_ack(port: &mut TTYPort) {
    let mut buf = [0u8; 1024];
    match port.read(&mut buf) {
        Ok(n) => println!("printer:{}", String::from_utf8_lossy(&buf[..n])),
        Err(e) => eprintln!("no ack recived from: {}", e),
    }
}
fn serial_killer(mut port: TTYPort) {
    port.flush().ok();
    port.write_request_to_send(false).ok();
    drop(port);
}

fn main() {
    let timeout = Duration::from_secs(8);
    let mut port = serial_starter("/dev/ttyACM0", 115200, timeout);

    send_cmd(&mut port, "M115\n");
    recive_ack(&mut port);

    serial_killer(port);
}
