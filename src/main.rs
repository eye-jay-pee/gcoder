fn main() {
    print::avalible_ports_report();

    let my_port = serial::connect_by_id(1, 115200);

    print::connected_port_report(my_port.as_ref());
}

mod serial {
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
}

mod print {
    use serialport::SerialPort;
    pub fn connected_port_report(port: &dyn SerialPort) {
        println!("\x1b[1m{:^28}\x1b[0m", "PORT REPORT");
        println!("{:>12} = {}", "name", port.name().unwrap());
        println!("{:>12} = {}", "baud rate", port.baud_rate().unwrap());
        println!("{:>12} = {}", "data bits", port.data_bits().unwrap());
        println!("{:>12} = {}", "flow control", port.flow_control().unwrap());
        println!("{:>12} = {}", "parity", port.parity().unwrap());
        println!("{:>12} = {}", "stop bits", port.stop_bits().unwrap());
        println!("{:>12} = {:#?}", "timeout", port.timeout());
    }
    pub fn avalible_ports_report() {
        use serialport::{available_ports, SerialPortType::*};
        println!("\x1b[1m{:^28}\x1b[0m", "AVAILABLE PORTS");
        println!("{:<4}{:^16}{:>8}", "ID", "Name", "Type");

        for (i, port) in available_ports().unwrap().iter().enumerate() {
            println!(
                "{:<4}{:^16}{:>8}",
                i,
                port.port_name,
                match port.port_type {
                    UsbPort(_) => "usb",
                    PciPort => "pci",
                    BluetoothPort => "bluetooth",
                    Unknown => "unknown",
                }
            );
        }
    }
}
