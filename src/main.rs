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
    use serialport::{Result, SerialPort};
    use std::time::Duration;

    pub struct _Printer {
        builder: serialport::SerialPortBuilder,

        port: Option<Box<dyn SerialPort>>,
        _verbose: bool,
        _log_file: String,
    }
    impl _Printer {
        pub fn _new(builder: serialport::SerialPortBuilder) -> Self {
            _Printer {
                builder: builder,
                port: None,
                _verbose: false,
                _log_file: String::from("~/.gcoder/log"),
            }
        }
        pub fn _exchange(&mut self, _transmission: &str) -> Result<String> {
            //self.connect()? {}

            Ok(String::from("ok"))
        }
        fn _connect(&mut self) -> Result<()> {
            self.port = Some(self.builder.clone().open()?);
            Ok(())
        }
        fn _connect_if_not_already(&mut self) -> Result<()> {
            match self.port {
                Some(_) => Ok(()),
                None => self._connect(),
            }
        }
        fn _rebind(&mut self) -> Result<()> {
            use std::time::Duration;
            use std::{fs, thread};

            let iface = "1-1.3:1.0"; // your interface from dmesg/udev
            fs::write("/sys/bus/usb/drivers/cdc_acm/unbind", iface)?;
            thread::sleep(Duration::from_millis(200));
            fs::write("/sys/bus/usb/drivers/cdc_acm/bind", iface)?;
            Ok(())
        }
    }

    pub fn transfer(port: &mut dyn SerialPort, data: &str) -> Result<String> {
        let mut buf = [0u8; 256];

        port.write_all(data.as_bytes())?;
        let n = port.read(&mut buf)?;

        Ok(String::from_utf8_lossy(&buf[..n]).into_owned())
    }
    pub fn initalize(name: &str, baud: u32, timeout: Duration) -> Result<Box<dyn SerialPort>> {
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
