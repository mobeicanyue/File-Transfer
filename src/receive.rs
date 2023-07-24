use blake3::{Hash, Hasher};
use std::fs::File;
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener};

const MAX_PACKET_SIZE: usize = 4 * 1024;

pub fn receive_file(ip: &SocketAddr) -> Result<(), Box<dyn std::error::Error>> {
    println!("Server listening on {}", ip);
    // 创建套接字 let listener: TcpListener = TcpListener::bind(ip);
    let listener: TcpListener = match TcpListener::bind(ip) {
        Ok(listener) => listener,
        Err(e) => {
            panic!("Error binding to socket, please check port usage:\n {}", e);
        }
    };
    println!("Waiting for incoming connections...");

    // 接收连接
    let (mut stream, socket) = listener.accept()?;
    // 获取客户端的IP地址
    let client_ip = socket.ip();
    println!("Remote Client IP address: {}", client_ip);

    // 1.接收文件名的长度
    let mut file_name_length_buffer = [0; 1]; // Assuming the length can be represented in 4 bytes (u32)
    stream.read_exact(&mut file_name_length_buffer)?;
    let file_name_length = file_name_length_buffer[0] as usize;

    // 2.接收文件名
    let mut file_name_buffer = vec![0; file_name_length];
    stream.read_exact(&mut file_name_buffer)?;

    let file_name = String::from_utf8(file_name_buffer)?;
    println!("Receiving file: {}", file_name);

    // 创建文件
    let mut file = File::create(file_name.trim())?;
    let mut buffer = [0; MAX_PACKET_SIZE];

    // 3.接收文件的长度
    let mut file_length_buffer = [0; 8]; // Assuming the length can be represented in 8 bytes (u64)
    stream.read_exact(&mut file_length_buffer)?;
    let file_length = u64::from_be_bytes(file_length_buffer);

    println!("File size: {} bytes", file_length);

    // 4.接收哈希值
    let mut hash_value_buffer = [0; 32]; // 默认32字节长度
                                         // 接收哈希值
    stream.read_exact(&mut hash_value_buffer)?;

    // 将哈希结果转换为十六进制字符串并打印
    let hash_string_buffer = Hash::from(hash_value_buffer).to_hex();
    println!("BLAKE3 Hash Received: {}", hash_string_buffer);

    // 计算文件的blake3
    let mut blake3 = Hasher::new();
    // 5.接收文件字节
    loop {
        // let bytes_read = stream.read(&mut buffer)
        let bytes_read = match stream.read(&mut buffer) {
            Ok(bytes_read) => bytes_read,
            Err(e) => {
                panic!("Error reading stream: {}", e);
            }
        };

        if bytes_read == 0 {
            // Connection closed, file receiving completed
            break;
        }

        // 更新 BLAKE3 哈希值
        blake3.update(&buffer[..bytes_read]);

        // file.write_all(&buffer[..bytes_read])
        if let Err(e) = file.write_all(&buffer[..bytes_read]) {
            panic!("Error writing to file: {}", e);
        }
    }

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
        println!("File integrity verification failed. File may be corrupted.");
    }

    println!("File successfully received.");
    Ok(())
}
