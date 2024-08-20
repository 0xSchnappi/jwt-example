/*
 * @Author: 0xSchnappi 952768182@qq.com
 * @Date: 2024-08-20 16:34:01
 * @LastEditors: 0xSchnappi 952768182@qq.com
 * @LastEditTime: 2024-08-20 17:44:56
 * @FilePath: /jwt-example/src/client.rs
 * @Description: jwt 客户端示例
 *
 * Copyright (c) 2024 by github.com/0xSchnappi, All Rights Reserved.
 */
use std::io::{Read, Write};
use std::net::TcpStream;

fn main() {
    // 请求 JWT
    let mut stream = TcpStream::connect("127.0.0.1:8080").expect("Could not connect to server");
    let request = "GET /token HTTP/1.1\r\nHost: localhost\r\n\r\n";
    stream
        .write(request.as_bytes())
        .expect("Failed to write to server");

    let mut buffer = [0; 512];
    stream
        .read(&mut buffer)
        .expect("Failed to read from server");
    let response = String::from_utf8_lossy(&buffer);
    let token = response.split("\r\n\r\n").nth(1).unwrap().trim();

    println!("Received JWT: {}", token);

    // 使用 JWT 访问受保护资源
    let mut stream = TcpStream::connect("127.0.0.1:8080").expect("Could not connect to server");
    let request = format!(
        "GET /protected HTTP/1.1\r\nHost: localhost\r\nAuthorization: Bearer {}\r\n\r\n",
        token
    );
    stream
        .write(request.as_bytes())
        .expect("Failed to write to server");

    let mut buffer = [0; 512];
    stream
        .read(&mut buffer)
        .expect("Failed to read from server");
    let response = String::from_utf8_lossy(&buffer);

    println!("Server response: {}", response);
}
