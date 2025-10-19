use serde_json;
use std::env;
use serde_bencode;

#[allow(dead_code)]
fn decode_bencoded_value(encode:&str) {
    let decode : serde_bencode::value::Value = serde_bencode::from_str(encode).unwrap();
    // println!("{:?}", decode);
    let json_value = convert(decode);
    // println!("{:?}", json_value);
    let json_string = serde_json::to_value(&json_value).unwrap();
    println!("{}", json_string);


}
fn convert(el: serde_bencode::value::Value) -> serde_json::value::Value {
    match el {
        serde_bencode::value::Value::Bytes(bytes) => {
            serde_json::value::Value::String(String::from_utf8_lossy(&bytes).into_owned())
        },
        serde_bencode::value::Value::List(list) => {
            let json_list: Vec<serde_json::value::Value> = list.into_iter().map(convert).collect();
            serde_json::value::Value::Array(json_list)
        },
        serde_bencode::value::Value::Int(i) => {
            serde_json::value::Value::Number(serde_json::Number::from(i))
        },
        serde_bencode::value::Value::Dict(map) => {
            let mut json_map = serde_json::Map::new();
            for (key, value) in map {
                let key_str = String::from_utf8_lossy(&key).into_owned();
                json_map.insert(key_str, convert(value));
            }
            serde_json::value::Value::Object(json_map)
        },
        _ => serde_json::value::Value::Null,
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
    

     
    } else {
        println!("unknown command: {}", args[1])
    }
}