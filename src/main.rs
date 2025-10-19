use serde_bencode;
use serde_bytes::ByteBuf;
use serde_json;
use std::env;

#[allow(dead_code)]
fn decode_bencoded_value(encode: Vec<u8>) -> serde_json::value::Value {
    let decode: serde_bencode::value::Value = serde_bencode::from_bytes(&encode).unwrap();
    // println!("{:?}", decode);
    let json_value = convert(decode);
    // println!("{:?}", json_value);
    let json_string = serde_json::to_value(&json_value).unwrap();
    json_string
}
fn convert(el: serde_bencode::value::Value) -> serde_json::value::Value {
    match el {
        serde_bencode::value::Value::Bytes(bytes) => {
            serde_json::value::Value::String(String::from_utf8_lossy(&bytes).into_owned())
        }
        serde_bencode::value::Value::List(list) => {
            let json_list: Vec<serde_json::value::Value> = list.into_iter().map(convert).collect();
            serde_json::value::Value::Array(json_list)
        }
        serde_bencode::value::Value::Int(i) => {
            serde_json::value::Value::Number(serde_json::Number::from(i))
        }
        serde_bencode::value::Value::Dict(map) => {
            let mut json_map = serde_json::Map::new();
            for (key, value) in map {
                let key_str = String::from_utf8_lossy(&key).into_owned();
                json_map.insert(key_str, convert(value));
            }
            serde_json::value::Value::Object(json_map)
        }
        _ => serde_json::value::Value::Null,
    }
}

#[derive(serde::Deserialize)]
struct MetaInfo {
    announce: String,
    info: InfoDict,
}

#[derive(serde::Deserialize)]
struct InfoDict {
    pieces: ByteBuf, // <- binary SHA1 hashes
    name: String,
    length: i64,
    #[serde(rename = "piece length")]
    piece_length: i64,
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
        let result = decode_bencoded_value(encoded_value.as_bytes().to_vec());
        println!("{}", result);

        // let decoded_value = serde_bencode::from_str::<serde_json::Value>(&encoded_value.to_string()).unwrap();
    } else if command == "info" {
        let torrent_url = &args[2];
        let data = std::fs::read(torrent_url).expect("Unable to read file");
        let bencode_str: Vec<u8> = data;
        // println!("{:?}", bencode_str);
        let new_result = serde_bencode::from_bytes::<MetaInfo>(&bencode_str)
            .expect("Failed to deserialize MetaInfo");

        // let result = decode_bencoded_value(bencode_str);
        // // println!("{}", result);

        // let new_result: MetaInfo = serde_json::from_value(result).expect("Failed to deserialize MetaInfo");

        println!("Tracker URL: {}", new_result.announce);

        println!("Length: {}", new_result.info.length);
    } else {
        println!("unknown command: {}", args[1]);
    }
}
