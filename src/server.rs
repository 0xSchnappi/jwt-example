/*
 * @Author: 0xSchnappi 952768182@qq.com
 * @Date: 2024-08-20 16:31:33
 * @LastEditors: 0xSchnappi 952768182@qq.com
 * @LastEditTime: 2024-08-20 17:51:24
 * @FilePath: /jwt-example/src/server.rs
 * @Description: jwt 服务端示例
 *
 * Copyright (c) 2024 by github.com/0xSchnappi, All Rights Reserved.
 */
use std::io::{Read, Write};
use std::net::TcpListener;
use std::str;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Could not bind");

    for stream in listener.incoming() {
        let mut stream = stream.expect("Failed to accept connection");
        let mut buffer = [0; 512];
        stream
            .read(&mut buffer)
            .expect("Failed to read from connection");

        // 解析请求
        let request = str::from_utf8(&buffer).unwrap();
        if request.contains("GET /token") {
            // 创建 JWT
            let header = r#"{"alg":"HS256","typ":"JWT"}"#;
            let payload = r#"{"sub":"1234567890","name":"John Doe","iat":1516239022}"#;
            let secret = "your-256-bit-secret";

            let token = create_jwt(header, payload, secret);

            // 返回 JWT
            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\n\r\n{}",
                token
            );
            stream
                .write(response.as_bytes())
                .expect("Failed to write to connection");
        } else if request.contains("GET /protected") {
            // 解析 JWT
            if let Some(auth_header) = request
                .lines()
                .find(|&line| line.starts_with("Authorization: Bearer"))
            {
                let token = auth_header.split_whitespace().nth(2).unwrap();
                let secret = "your-256-bit-secret";

                if verify_jwt(token, secret) {
                    let response =
                        "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\n\r\nProtected content";
                    stream
                        .write(response.as_bytes())
                        .expect("Failed to write to connection");
                } else {
                    let response = "HTTP/1.1 401 Unauthorized\r\nContent-Type: text/plain\r\n\r\nInvalid token";
                    stream
                        .write(response.as_bytes())
                        .expect("Failed to write to connection");
                }
            } else {
                let response =
                    "HTTP/1.1 401 Unauthorized\r\nContent-Type: text/plain\r\n\r\nMissing token";
                stream
                    .write(response.as_bytes())
                    .expect("Failed to write to connection");
            }
        }
    }
}

use base64::{self, Engine};
fn base64_encode(input: &str) -> String {
    general_purpose::STANDARD.encode(input)
    // base64::encode(input)
}

fn base64_decode(input: &str) -> String {
    // let decoded = base64::decode(input).expect("Invalid Base64 input");
    let decoded = general_purpose::STANDARD.decode(input).unwrap();
    String::from_utf8(decoded).expect("Invalid UTF-8 sequence")
}

use base64::engine::general_purpose;
use hmac::{Hmac, Mac};
use sha2::{Digest, Sha256}; // HMAC 实现

// 创建 HMAC-SHA256 的签名
fn hmac_sha256(secret: &str, data: &str) -> Vec<u8> {
    let mut mac =
        Hmac::<Sha256>::new_from_slice(secret.as_bytes()).expect("HMAC can take key of any size");
    mac.update(data.as_bytes());
    mac.finalize().into_bytes().to_vec()
}

fn create_jwt(header: &str, payload: &str, secret: &str) -> String {
    let encoded_header = base64_encode(header);
    let encoded_payload = base64_encode(payload);

    // 拼接 header 和 payload
    let data = format!("{}.{}", encoded_header, encoded_payload);

    // 生成签名
    let signature = hmac_sha256(secret, &data);

    let encoded_signature = base64_encode(&hex::encode(signature));

    // 拼接最终的 JWT
    format!(
        "{}.{}.{}",
        encoded_header, encoded_payload, encoded_signature
    )
}

fn verify_jwt(token: &str, secret: &str) -> bool {
    let parts: Vec<&str> = token.split('.').collect();
    if parts.len() != 3 {
        return false;
    }

    let encoded_header = parts[0];
    let encoded_payload = parts[1];
    let provided_signature = parts[2].trim_matches('\0');

    // 重新计算签名
    let data = format!("{}.{}", encoded_header, encoded_payload);
    let expected_signature = hmac_sha256(secret, &data);
    let encoded_expected_signature = base64_encode(&hex::encode(expected_signature));

    // 比较提供的签名和计算出的签名
    provided_signature == encoded_expected_signature
}
