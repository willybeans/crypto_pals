// use crate::set_1::hex_to_base::calculate;
mod set_1;

fn main() {
    let our_string: String = String::from("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");
    let base64 =  set_1::hex_to_base::calculate(our_string);
    println!("converted hex_to_base: {:?}", base64);

    let first = String::from("1c0111001f010100061a024b53535009181c");
    let second: String = String::from("686974207468652062756c6c277320657965");
    let fixed_xor = set_1::xor::convert(first, second);
    println!("converted to xor: {:?}", fixed_xor);


}