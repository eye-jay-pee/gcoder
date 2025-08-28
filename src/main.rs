use std::time::Duration;

fn main() {
    let port_name = "/dev/ttyACM0";
    let baud_rate = 115200;
    let timeout = Duration::from_secs(1);
    let cmd = "M115\n";

    let mut port = serial::initalize(port_name, baud_rate, timeout).unwrap();

    println!("{}:\n{}", "host", cmd);
    let reply = serial::transfer(&mut *port, cmd).unwrap();
    println!("{}:\n{}", "printer", reply);
}

mod serial {
    use serialport::{Result, SerialPort, SerialPortBuilder};
    use std::time::Duration;

    pub struct _Printer {
        builder: Box<SerialPortBuilder>,
        port: Option<Box<dyn SerialPort>>,
        _log_file: String,
    }
    impl _Printer {
        pub fn _new(builder: Box<SerialPortBuilder>) -> Self {
            _Printer {
                builder: builder,
                port: None,
                _log_file: String::from("~/.gcoder/log"),
            }
        }
        pub fn _send(&mut self, _transmission: &str) -> Result<String> {
            if self.port.is_none() {
                match self.connect() {}
            }

            Ok(String::from("ok"))
        }
        fn connect(&mut self) -> Result<()> {}
    }

    pub fn transfer(port: &mut dyn SerialPort, data: &str) -> Result<String> {
        let mut buf = [0u8; 256];

        port.write_all(data.as_bytes())?;
        let n = port.read(&mut buf)?;

        Ok(String::from_utf8_lossy(&buf[..n]).into_owned())
    }
    pub fn initalize(
        name: &str,
        baud: u32,
        timeout: Duration,
    ) -> Result<Box<dyn SerialPort>> {
        rebind()?;
        Ok(serialport::new(name, baud).timeout(timeout).open()?)
    }

    fn rebind() -> std::io::Result<()> {
        use std::time::Duration;
        use std::{fs, thread};

        let iface = "1-1.3:1.0"; // your interface from dmesg/udev
        fs::write("/sys/bus/usb/drivers/cdc_acm/unbind", iface)?;
        thread::sleep(Duration::from_millis(200));
        fs::write("/sys/bus/usb/drivers/cdc_acm/bind", iface)?;
        Ok(())
    }
}
