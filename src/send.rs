use blake3::Hasher;
use std::fs::File;
use std::io::{copy, Read, Seek, SeekFrom, Write};
use std::net::{Shutdown, SocketAddr, TcpStream};
use std::path::Path;

use crate::utils;

const MAX_PACKET_SIZE: usize = utils::MAX_PACKET_SIZE;

/// function: send_file
/// args:
///    socket: &SocketAddr, 服务器地址
///    file_path: &str, 文件路径
pub fn send_file(socket: &SocketAddr, file_path: &str) {
    println!("Connecting to server at {}", socket);

    // 创建套接字
    let mut stream = TcpStream::connect(socket).unwrap();

    // 打开文件
    let mut file = File::open(file_path).unwrap();

    // 计算文件的blake3
    let mut blake3 = Hasher::new();
    copy(&mut file, &mut blake3).unwrap(); // std::io::copy 将文件的内容复制到 blake3 中
    let hash_result = blake3.finalize(); // 计算哈希值

    let hash_value: &[u8; 32] = hash_result.as_bytes(); // 将hash值转换为字节数组,默认32字节长度
    let hex_string = hash_result.to_hex(); // 将哈希结果转换为十六进制字符串

    // 1.发送文件名的长度
    let file_name = Path::new(file_path).file_name().unwrap().to_str().unwrap();
    let file_name_length = file_name.len() as u8;
    stream.write_all(&file_name_length.to_be_bytes()).unwrap();
    println!("File name length: {}", file_name_length);

    // 2.发送文件名
    stream.write_all(file_name.as_bytes()).unwrap();
    println!("File name: {}", file_name);

    // 3.发送文件字节的长度
    let file_length = file.metadata().unwrap().len();
    stream.write_all(&file_length.to_be_bytes()).unwrap();
    utils::print_file_size(file_length);

    // 4.发送blake3值
    stream.write_all(hash_value).unwrap();
    // 将哈希结果转换为十六进制字符串并打印
    println!("BLAKE3 Hash Result: {}", hex_string);

    // 5.发送文件字节
    let mut buffer = [0; MAX_PACKET_SIZE];
    file.seek(SeekFrom::Start(0)).unwrap(); // 重置文件指针
    let progress_bar = utils::create_progress_bar(file_length); // 创建进度条
    loop {
        let bytes_read = match file.read(&mut buffer) {
            Ok(0) => break, // Reached EOF, file sending completed
            Ok(n) => n,
            Err(e) => panic!("An error occurred while reading the file: {}", e),
        };

        progress_bar.inc(bytes_read as u64); // 更新进度条
        stream.write_all(&buffer[..bytes_read]).unwrap(); // 将字节写入文件
    }

    progress_bar.finish(); // 完成进度条

    // 关闭写入流
    stream.shutdown(Shutdown::Write).unwrap();
    println!("File successfully sent.");
}
