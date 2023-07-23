use local_ip_address::list_afinet_netifas;
use std::net::{IpAddr, Ipv4Addr};

/// Get all network interfaces with IPv4 private address except loopback
/// Returns a vector of tuples (index, name, ip)
/// index: usize - index of the network interface
/// name: String - name of the network interface
/// ip: Ipv4Addr - IPv4 address of the network interface
pub fn get_nics() -> Vec<(usize, String, Ipv4Addr)> {
    let network_interfaces: Vec<(String, IpAddr)> = list_afinet_netifas().unwrap();

    let mut private_nics: Vec<(usize, String, Ipv4Addr)> = Vec::new();
    let mut count: usize = 0;

    for (name, ip) in network_interfaces.iter() {
        if let IpAddr::V4(ipv4) = ip {
            // if ipv4.is_private() {
                private_nics.push((count, name.to_string(), *ipv4));
                count += 1;
            // }
        }
    }

    let private_nics = private_nics; // 重新将 private_nics 声明为不可变的

    // 打印出所有的网卡
    println!("Your local IP address is:");
    for (count, name, ip) in private_nics.iter() {
        println!("{}: {} ({})", count, name, ip);
    }

    private_nics
}

pub fn select_nic(nics: Vec<(usize, String, Ipv4Addr)>) -> Ipv4Addr {
    println!("\nPlease select the network interface you want to use (default 0):");
    let mut num_net = String::new();
    std::io::stdin().read_line(&mut num_net).unwrap();
    let num_net = num_net.trim();
    let nic_index: usize = if num_net.is_empty() {
        0
    } else {
        num_net.parse().unwrap()
    };

    let nic = nics.get(nic_index).unwrap(); // 获取选择的网卡
    println!("You selected: ({}: {} ({}))", nic.0, nic.1, nic.2);

    nic.2 // 返回选择的网卡的ip
}

pub fn select_operation() -> u8 {
    println!("Please select the operation: \n1. Send file \n2. Receive file");
    let mut num_op = String::new();
    std::io::stdin().read_line(&mut num_op).unwrap();
    let num_op = num_op.trim().parse().unwrap();
    num_op
}
