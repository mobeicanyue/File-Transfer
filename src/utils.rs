use indicatif::{HumanBytes, ProgressBar, ProgressState, ProgressStyle};
use local_ip_address::list_afinet_netifas;
use std::fmt::Write;
use std::net::{IpAddr, Ipv4Addr};

// pub const MAX_PACKET_SIZE: usize = 1024 * 1024; // 最大包大小
pub const MAX_SEND_SIZE: usize = 2 * 1024 * 1024; // 最大发送大小 2M
pub const MAX_RECEIVE_SIZE: usize = 1024 * 1024; // 最大接收大小 1M

/// Get all network interfaces with IPv4 private address except loopback
/// Returns a vector of tuples (index, name, ip)
/// index: usize - index of the network interface
/// name: String - name of the network interface
/// ip: Ipv4Addr - IPv4 address of the network interface
pub fn get_nics() -> Vec<(usize, String, Ipv4Addr)> {
    let network_interfaces: Vec<(String, IpAddr)> = list_afinet_netifas().unwrap();

    let mut private_nics: Vec<(usize, String, Ipv4Addr)> = Vec::new();
    let mut count: usize = 1;

    for (name, ip) in network_interfaces.iter() {
        if let IpAddr::V4(ipv4) = ip {
            private_nics.push((count, name.to_string(), *ipv4));
            count += 1;
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

// 文件检验是否存在，如果不存在则重新输入
pub fn check_file_exist(file_path: &str) -> String {
    let mut file_path = file_path.to_string();
    loop {
        if std::path::Path::new(&file_path).exists() {
            break;
        } else {
            println!("File not found: {}", file_path);
            file_path.clear();
            println!("Please enter the file path again:");
            std::io::stdin().read_line(&mut file_path).unwrap();
            file_path = file_path.trim().to_string();
        }
    }
    file_path
}

pub fn select_nic(nics: Vec<(usize, String, Ipv4Addr)>) -> Ipv4Addr {
    println!("\nPlease select the network interface you want to use (default 1):");
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

pub fn print_file_size(file_length: u64) {
    println!("File size: {} ({})", file_length, HumanBytes(file_length));
}

pub fn create_progress_bar(file_length: u64) -> ProgressBar {
    let progress_bar = ProgressBar::new(file_length); // 创建进度条
    progress_bar.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] {bar:50} {bytes}/{total_bytes} ({eta})").unwrap()
        .with_key("eta", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap()));
    progress_bar
}
