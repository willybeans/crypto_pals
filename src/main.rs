extern crate base64;
use std::u8;
use base64::{Engine as _, engine::general_purpose};

pub fn hex_to_base64(hex: String) -> String {

    // Make vector of bytes from octets
    // u8 0 -> 255, smalled integer type
    // what is an octet? 
    let mut bytes = Vec::new();
    println!("hex: {:?}", hex);

    for i in 0..(hex.len()/2) {
        //from_str_radix: Converts a string slice in a given base to an integer.
        let res = u8::from_str_radix(&hex[2*i .. 2*i+2], 16);
        println!("--i:{:?}--of--{:?}-", i, (hex.len()/2));
        println!("test op: {} {} {}", i, 2*i, 2*i+2);
        println!("hex[]: {:?}", &hex[2*i .. 2*i+2]);
        println!("str_radix: {:#?}", u8::from_str_radix(&hex[2*i .. 2*i+2], 16));
        println!("res: {:?}", res);
        println!("()()()()()()()");


        match res {
            Ok(v) => bytes.push(v),
            Err(e) => println!("Problem with hex: {}", e),
        };
    };

    general_purpose::STANDARD.encode(&bytes) // now convert from Vec<u8> to b64-encoded String
}

fn main() {
    let our_string: String = String::from("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");
    let base64 = hex_to_base64(our_string);
    println!("converted: {:?}", base64);
}