use serialport::SerialPort;

/// Everything about a serialport that can be learned without interacting
/// with the slave device, which requires mutability.
pub fn connected_port_report(port: &dyn SerialPort) {
    println!("\x1b[1m{:^28}\x1b[0m", "PORT REPORT");
    println!("{:>12} = {}", "name", port.name().unwrap());
    println!("{:>12} = {}", "baud rate", port.baud_rate().unwrap());
    println!("{:>12} = {}", "data bits", port.data_bits().unwrap());
    println!("{:>12} = {}", "flow control", port.flow_control().unwrap());
    println!("{:>12} = {}", "parity", port.parity().unwrap());
    println!("{:>12} = {}", "stop bits", port.stop_bits().unwrap());
    println!("{:>12} = {:#?}", "timeout", port.timeout());
    println!(
        "{:>12} = {}",
        "bytes to read",
        port.bytes_to_read().unwrap()
    );
    println!(
        "{:>12} = {}",
        "bytes to write",
        port.bytes_to_write().unwrap()
    );
}

pub fn port_client_report(port: &mut dyn SerialPort) {
    println!("\x1b[1m{:^28}\x1b[0m", "CLIENT REPORT");
    println!(
        "{:>12} = {}",
        "data set ready",
        port.read_data_set_ready().unwrap()
    );
    println!(
        "{:>12} = {}",
        "ring indicator",
        port.read_ring_indicator().unwrap()
    );
    println!(
        "{:>12} = {}",
        "carrier detect",
        port.read_carrier_detect().unwrap()
    );
}

/// List of avalible ports
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
