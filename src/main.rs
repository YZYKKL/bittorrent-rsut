use serde_json;
use std::env;
// Available if you need it!
// use serde_bencode
#[allow(dead_code)]
fn decode_bencoded_value(encoded_value: &str) -> (serde_json::Value, &str) {

    match encoded_value.chars().next() {
        Some('i') => {
            if let Some((n,rest)) = encoded_value
                                                .split_at(1)
                                                .1
                                                .split_once('e')
                                                .and_then(|(digits, rest)|{
                                                    let n = digits.parse::<i64>().ok()?;
                                                    Some((n,rest))
                                                })
            {
                return (n.into(),rest);
            }
        }
        Some('l')=> {
            let mut values = Vec::new();
            let mut rest = encoded_value.split_at(1).1;
            while !rest.is_empty() && !rest.starts_with('e') {
                let (v, reamainder) = decode_bencoded_value(rest);
                values.push(v);
                rest = reamainder;
            }
            return (values.into(), &rest[1..]);
        }
        Some('0'..='9') => {
            if let Some((len,rest)) =encoded_value.split_once(":"){
                eprintln!("len is {len}, rest is {rest}");
                if let Ok(len) = len.parse::<usize>() {
                    return (rest[..len].to_string().into(), &rest[len..]);
                }
            }
        }  
        _ => {}
    }
    panic!("Unhandled encoded value: {}", encoded_value);
}
// Usage: your_bittorrent.sh decode "<encoded_value>"
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        //eprintln!("Usage: your_bittorrent.sh decode \"<encoded_value>\"");
    } else {
        let command = &args[1];

        if command == "decode" {
            if args.len() < 3 {
                //eprintln!("Usage: your_bittorrent.sh decode \"<encoded_value>\"");
            } else {
                let encoded_value = &args[2];
                let decoded_value = decode_bencoded_value(encoded_value);
                println!("{}", decoded_value.0.to_string());
            }
        } else {
            eprintln!("unknown command: {}", command)
        }
    }
}