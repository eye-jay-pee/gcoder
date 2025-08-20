use std::io::{Read, Write};
use std::time::Duration;

fn main() {
    let port_name = "/dev/ttyACM0";
    let baud_rate = 115200;
    let timeout = Duration::from_secs(1);
    let cmd = "M115\n";

    let mut buf = [0u8; 1024];

    let _ = rebind_driver(port_name);

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

use std::{
    io,
    process::{Command, Stdio},
};

fn call_script(script: &str, args: &str) -> io::Result<()> {
    let status = Command::new(&script)
        .arg(args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()?;

    if status.success() {
        Ok(())
    } else {
        Err(io::Error::new(
            io::ErrorKind::Other,
            "script returned non-zero",
        ))
    }
}

/// This simulates unplugging the device for 0.1 seconds. It is to be used as
/// a last-resort reset when the printer stops responding.
fn rebind_driver(dev: &str) {
    let _ = call_script("scripts/rebind_driver.sh", dev);
}
