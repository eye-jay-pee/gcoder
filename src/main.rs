mod print;

fn main() {
    print::avalible_ports_report();

    let mut my_port = serial::connect_by_id(1, 115200);
    print::connected_port_report(my_port.as_ref());
    print::port_client_report(my_port.as_mut());

    serial::write(my_port.as_mut(), b"M115\n");
    let ack = serial::read(my_port.as_mut());
    println!("ack: {}", ack);
    print::connected_port_report(my_port.as_ref());
    print::port_client_report(my_port.as_mut());
}

mod serial {
    use chrono::DateTime;

    pub struct packet {
        data: String,
        sent: DateTime,
        ack: String,
        acked: DateTime,
    }

    use serialport::SerialPort;
    pub fn connect(port: &str, baud: u32) -> Box<dyn SerialPort> {
        use std::time::Duration;
        let timeout = Duration::from_secs(1);
        serialport::new(port, baud).timeout(timeout).open().unwrap()
    }
    pub fn connect_by_id(id: usize, baud: u32) -> Box<dyn SerialPort> {
        use serialport::available_ports;
        connect(&available_ports().unwrap()[id].port_name, baud)
    }
    pub fn write(port: &mut dyn SerialPort, data: &[u8]) {
        let _ = port.write_all(data);
    }
    pub fn read(port: &mut dyn SerialPort) -> String {
        let mut buf = [0u8; 128];
        let n = port.read(&mut buf).unwrap();
        String::from_utf8_lossy(&buf[..n]).into_owned()
    }
}
