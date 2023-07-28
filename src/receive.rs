use blake3::{Hash, Hasher};
use std::fs::File;
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use threadpool::ThreadPool;

use crate::utils;

const MAX_PACKET_SIZE: usize = utils::MAX_RECEIVE_SIZE;

pub fn run_server(socket_server: &SocketAddr) {
    println!("Server listening on {}", socket_server);
    // 创建套接字
    let listener: TcpListener = TcpListener::bind(socket_server).unwrap();
    println!("Waiting for incoming connections...");

    // 创建线程池，设置最小线程数和最大线程数，这里我们设置最小为 4，最大为 8，你可以根据需要调整
    let thread_pool = ThreadPool::new(4);

    loop {
        // 接收连接
        let (mut stream, socket) = listener.accept().unwrap();
        // 获取客户端的IP地址
        let client_ip = socket.ip();
        println!("Remote Client IP address: {}", client_ip);
        // 处理每个客户端的连接
        thread_pool.execute(move || {
            receive_file(&mut stream);
        });
    }
}

/// function: receive_file
/// args:
///   stream: &TcpStream, 客户端连接流
pub fn receive_file(stream: &mut TcpStream) {
    // 1.接收文件名的长度
    let mut file_name_length_buffer = [0; 1]; // Assuming the length can be represented in 4 bytes (u32)
    stream.read_exact(&mut file_name_length_buffer).unwrap();
    let file_name_length = file_name_length_buffer[0] as usize;

    // 2.接收文件名
    let mut file_name_buffer = vec![0; file_name_length];
    stream.read_exact(&mut file_name_buffer).unwrap();

    let file_name = String::from_utf8(file_name_buffer).unwrap();
    println!("Receiving file: {}", file_name);

    // 创建文件
    let mut file = File::create(file_name.trim()).unwrap();
    let mut buffer = [0; MAX_PACKET_SIZE];

    // 3.接收文件的长度
    let mut file_length_buffer = [0; 8]; // Assuming the length can be represented in 8 bytes (u64)
    stream.read_exact(&mut file_length_buffer).unwrap();
    let file_length = u64::from_be_bytes(file_length_buffer);
    utils::print_file_size(file_length);

    // 4.接收哈希值
    let mut hash_value_buffer = [0; 32]; // 默认32字节长度
    stream.read_exact(&mut hash_value_buffer).unwrap(); // 接收哈希值

    // 将哈希结果转换为十六进制字符串并打印
    let hash_string_buffer = Hash::from(hash_value_buffer).to_hex();
    println!("BLAKE3 Hash Received: {}", hash_string_buffer);

    // 计算文件的blake3
    let mut blake3 = Hasher::new();
    // 5.接收文件字节
    let progress_bar = utils::create_progress_bar(file_length); // 创建进度条
    loop {
        let bytes_read = match stream.read(&mut buffer) {
            Ok(0) => break, // Connection closed, file receiving completed
            Ok(n) => n,
            Err(err) => panic!(
                "An error occurred while receiving data: \x1b[0;37;41m{}\x1b[0m",
                err
            ),
        };

        blake3.update(&buffer[..bytes_read]); // 更新 BLAKE3 哈希值
        progress_bar.inc(bytes_read as u64); // 更新进度条
        file.write_all(&buffer[..bytes_read]).unwrap();
    }

    progress_bar.finish(); // 完成进度条

    println!("File received. Verifying file integrity...");

    let hash_result = blake3.finalize(); // 计算哈希值

    let hash_value_calculate = hash_result.as_bytes(); // 将hash值转换为字节数组

    let hash_string_received = hash_result.to_hex();

    // 将哈希结果转换为十六进制字符串并打印
    println!("BLAKE3 Hash Result: {}", hash_string_received);

    // 比较接收到的哈希值与发送端计算得到的哈希值
    if &hash_value_buffer == hash_value_calculate {
        println!("File integrity verified. File successfully received.");
    } else {
        println!(
            "\x1b[0;37;41m File integrity verification failed. File may be corrupted. \x1b[0m "
        );
    }

    println!("File successfully received.");
}
