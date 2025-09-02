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
    use std::path::Path;
    use std::time::Duration;

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
        rebind(name)?;
        Ok(serialport::new(name, baud).timeout(timeout).open()?)
    }

    fn rebind(device: &str) -> std::io::Result<()> {
        use std::time::Duration;
        use std::{fs, thread};

        let dev_struct = Path::new(device);
        let addr = helpers::device_address(dev_struct)?;
        println!("addr:{:?}", addr);

        fs::write("/sys/bus/usb/drivers/cdc_acm/unbind", &addr)?;
        thread::sleep(Duration::from_millis(200));
        fs::write("/sys/bus/usb/drivers/cdc_acm/bind", &addr)?;
        Ok(())
    }

    mod helpers {
        use std::fs::canonicalize;
        use std::io::{Error, ErrorKind::InvalidInput, Result};
        use std::path::{Path, PathBuf};

        pub fn device_address(device: &Path) -> Result<String> {
            physical_address(device).ok_or_else(|| {
                Error::new(
                    InvalidInput,
                    format!("cannot find physical address for {:?}", device),
                )
            })
        }

        fn physical_address(device: &Path) -> Option<String> {
            let symbolic_link = driver_symlink(device)?;
            let physical_addr = follow_link(symbolic_link.as_path())?;

            Some(String::from(format!("{}", physical_addr)))
        }
        fn driver_symlink(device: &Path) -> Option<PathBuf> {
            if device.parent()?.to_str()? != "/dev" {
                return None;
            }
            let file_n = device.file_name()?.to_str()?;
            Some(PathBuf::from(format!("/sys/class/tty/{}/device", file_n,)))
        }
        fn follow_link(symlink: &Path) -> Option<String> {
            let canonical = canonicalize(symlink).ok()?;
            let physical = canonical.file_name()?.to_str()?;
            Some(String::from(physical))
        }
    }
}
