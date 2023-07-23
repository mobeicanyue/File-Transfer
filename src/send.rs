use blake3::Hasher;
use std::io::{Read, Seek, SeekFrom, Write};
use std::net::{SocketAddr, TcpStream};
use std::path::Path;

const MAX_PACKET_SIZE: usize = 1024;

pub fn send_file(socket: &SocketAddr, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Connecting to server at {}", socket);

    let mut stream: TcpStream = TcpStream::connect(socket)?; // 创建套接字

    let mut file = std::fs::File::open(file_path)?; // 创建文件

    // 计算文件的blake3
    let mut blake3 = Hasher::new();
    std::io::copy(&mut file, &mut blake3)?; // std::io::copy 将文件的内容复制到 blake3 中
    let hash_result = blake3.finalize(); // 计算哈希值

    let hash_value: &[u8; 32] = hash_result.as_bytes(); // 将hash值转换为字节数组,默认32字节长度
    let hex_string = hash_result.to_hex(); // 将哈希结果转换为十六进制字符串

    // 1.发送文件名的长度
    let file_name = Path::new(file_path).file_name().unwrap().to_str().unwrap();
    let file_name_length = file_name.len() as u32;
    stream.write_all(&file_name_length.to_be_bytes())?;
    println!("File name length: {}", file_name_length);

    // 2.发送文件名
    stream.write_all(file_name.as_bytes())?;
    println!("File name: {}", file_name);

    // 3.发送文件字节的长度
    let file_length = file.metadata()?.len() as u64;
    stream.write_all(&file_length.to_be_bytes())?;
    println!("File size: {} bytes", file_length);

    // 4.发送blake3值
    stream.write_all(hash_value)?;
    // 将哈希结果转换为十六进制字符串并打印
    println!("BLAKE3 Hash Result: {}", hex_string);

    // 5.发送文件字节
    let mut buffer = [0; MAX_PACKET_SIZE];
    file.seek(SeekFrom::Start(0))?; // 重置文件指针
    loop {
        let bytes_read = file.read(&mut buffer)?;
        if bytes_read == 0 {
            // Reached EOF, file sending completed
            break;
        }
        stream.write_all(&buffer[..bytes_read])?;
    }

    // 关闭写入流
    stream.shutdown(std::net::Shutdown::Write)?;
    println!("File successfully sent.");
    Ok(())
}
