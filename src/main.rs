use serialport::SerialPort;
use std::io::{Read, Write};
use std::time::Duration;

fn reset_serial_connection(port: &mut dyn SerialPort) {
    port.write_data_terminal_ready(true).unwrap();
    port.write_data_terminal_ready(false).unwrap();
}

fn main() {
    let port_name = "/dev/ttyACM0";
    let baud_rate = 115200;
    let timeout = Duration::from_secs(1);

    let mut port = serialport::new(port_name, baud_rate)
        .timeout(timeout)
        .open()
        .unwrap();

    port.write_request_to_send(true).unwrap();
    reset_serial_connection(&mut *port);

    let cmd = "M115\n";
    if let Err(e) = port.write_all(cmd.as_bytes()) {
        eprintln!("failed to send \n{} because of {}", cmd, e);
        return;
    } else {
        println!("sent {} to {}", cmd, port_name);
    }

    let mut buf = [0u8; 1024];
    match port.read(&mut buf) {
        Ok(n) => {
            println!("Received {} bytes:", n);
            println!("{}", String::from_utf8_lossy(&buf[..n]));
        }
        Err(e) => {
            eprintln!("Response not recived from {} due to {}", port_name, e);
        }
    }

    port.flush().ok();
    port.write_request_to_send(false).ok(); // master is no longer listening
    drop(port);
}
