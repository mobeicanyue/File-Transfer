mod network_utils;
mod receive;
mod send;

use std::{
    net::{Ipv4Addr, SocketAddr, SocketAddrV4},
    str::FromStr,
};

fn main() {
    let num_op: u8 = network_utils::select_operation(); // 选择操作，发送或者接收

    match num_op {
        1 => {
            println!("Please enter the server ip (default '127.0.0.1'):");
            let mut ip = String::new();
            std::io::stdin().read_line(&mut ip).unwrap();

            // 如果ip为空，则默认为 127.0.0.1
            let ip = if ip.trim().is_empty() {
                println!("Using default ip (127.0.0.1):");
                "127.0.0.1"
            } else {
                &ip
            };

            let ip = ip.trim();
            let ip: Ipv4Addr = Ipv4Addr::from_str(ip).unwrap();

            let server_addr = SocketAddr::V4(SocketAddrV4::new(ip, 6666)); // 创建socket

            println!("Please enter the file path:");
            let mut file_path = String::new();
            std::io::stdin().read_line(&mut file_path).unwrap();
            let file_path = file_path.trim();

            send::send_file(&server_addr, file_path);
        }
        2 => {
            let nics = network_utils::get_nics(); // 获取所有的ip

            let socket: Ipv4Addr = network_utils::select_nic(nics); // 选择ip

            let server_addr = SocketAddr::V4(SocketAddrV4::new(socket, 6666)); // 创建socket

            receive::receive_file(&server_addr);
        }
        _ => {
            println!("Invalid operation.");
        }
    }
}
