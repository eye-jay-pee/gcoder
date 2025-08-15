mod print;

fn main() {
    print::avalible_ports_report();

    let mut my_port = serial::connect_by_id(1, 115200);
    print::connected_port_report(my_port.as_ref());
    print::port_client_report(my_port.as_mut());

    let outmssg = serial::_Transmission::_send(&mut *my_port, "M115\n").unwrap();
    let inmssg = serial::_Transmission::_recive(&mut *my_port).unwrap();

    println!("sent\n{}\n\ngot\n{}", outmssg, inmssg);

    // serial::write(my_port.as_mut(), b"M115\n");
    // let ack = serial::read(my_port.as_mut());
    //println!("ack: {}", ack);
    print::connected_port_report(my_port.as_ref());
    print::port_client_report(my_port.as_mut());
}

mod serial {
    use chrono::{DateTime, Local};
    use serialport::SerialPort;

    pub struct _Transmission {
        data: Vec<u8>,
        timestamp: DateTime<Local>,
        outbound: bool,
    }
    impl _Transmission {
        fn _validate(self) -> Self {
            let _now = Local::now(); // TODO: check for unreasonable timestamps
                                     // IE the future, or before the publish
                                     // date of the current package
            self
        }
        pub fn _send(port: &mut dyn SerialPort, text: &str) -> std::io::Result<Self> {
            let x = _Transmission {
                data: text.as_bytes().to_vec(),
                timestamp: Local::now(),
                outbound: true,
            };
            port.write_all(&x.data)?;
            Ok(x._validate())
        }
        pub fn _recive(port: &mut dyn SerialPort) -> std::io::Result<Self> {
            let mut x = _Transmission {
                data: Vec::new(),
                timestamp: Local::now(),
                outbound: false,
            };
            port.read_to_end(&mut x.data)?;
            Ok(x._validate())
        }
    }
    use std::fmt;

    impl fmt::Display for _Transmission {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "Timestamp: {}\nOutbound: {}\nData: {:?}",
                self.timestamp, self.outbound, self.data
            )
        }
    }

    pub fn connect(port: &str, baud: u32) -> Box<dyn SerialPort> {
        use std::time::Duration;
        let timeout = Duration::from_secs(1);
        serialport::new(port, baud).timeout(timeout).open().unwrap()
    }
    pub fn connect_by_id(id: usize, baud: u32) -> Box<dyn SerialPort> {
        use serialport::available_ports;
        connect(&available_ports().unwrap()[id].port_name, baud)
    }
    // pub fn write(port: &mut dyn SerialPort, data: &[u8]) {
    //     let _ = port.write_all(data);
    // }
    // pub fn read(port: &mut dyn SerialPort) -> String {
    //     let mut buf = [0u8; 128];
    //     let n = port.read(&mut buf).unwrap();
    //     String::from_utf8_lossy(&buf[..n]).into_owned()
    // }
}
