use serde_json;
use std::env;
use serde_bencode;

#[allow(dead_code)]
fn decode_bencoded_value(encoded_value: &str) {
    // If encoded_value starts with a digit, it's a number
    let first_no = encoded_value.as_bytes()[0];
     if first_no=='i' as u8{
        let decoded_value: i64 = serde_bencode::from_bytes(encoded_value.as_bytes()).unwrap();
        println!("{}", decoded_value);
     } else if first_no=='l' as u8 {
        let decoded_value: Vec<String> = serde_bencode::from_bytes(encoded_value.as_bytes()).unwrap();
        println!("{:?}", decoded_value);
     } else if first_no=='d' as u8 {
        let decoded_value: std::collections::HashMap<String, String> = serde_bencode::from_bytes(encoded_value.as_bytes()).unwrap();
        println!("{:?}", decoded_value);
          
     } 
     else if first_no.is_ascii_digit() {
        let decoded_value: String = serde_bencode::from_bytes(encoded_value.as_bytes()).unwrap();
        println!("\"{}\"", decoded_value);
     }else {
        panic!("Unhandled encoded value: {}", encoded_value)
    }
}

// Usage: your_program.sh decode "<encoded_value>"
fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];

    if command == "decode" {
        // You can use print statements as follows for debugging, they'll be visible when running tests.
        eprintln!("Logs from your program will appear here!");

        // Uncomment this block to pass the first stage
        let encoded_value = &args[2];
    
        decode_bencoded_value(encoded_value);
        // let decoded_value = serde_bencode::from_str::<serde_json::Value>(&encoded_value.to_string()).unwrap();
    

        // println!("{}", decoded_value.to_string());
    } else {
        println!("unknown command: {}", args[1])
    }
} 