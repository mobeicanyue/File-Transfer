# **FileTransfer in `rust`**

- 使用rust编写的文件传输程序
- TCP 传输文件
- 下载上传的进度条，可视化命令行
- 对输入校验，防止非法输入
- 多线程接收文件  
- 哈希校验 使用更快更安全的 blake3 代替 sha2  
  介绍如下 [blake3-crates.io](https://crates.io/crates/blake3)

## 使用说明
- Linux/MacOS 执行程序
  - ./file-transfer
  - 选择服务端/客户端
  - 服务端输入要绑定的网卡地址
  - 客户端输入服务端地址和要传输的文件路径 
- Windows 执行程序
  - 双击 file-transfer.exe
  - 选择服务端/客户端
  - 服务端输入要绑定的网卡地址
  - 客户端输入服务端地址和要传输的文件路径  
  
  等待发送/接收完成

**`请遵守开源协议，不要用于非法用途`**