use rust_base58::{ToBase58, FromBase58};
use hex;
use solana_sdk::bs58;

fn main() {
    let address = String::from("9axNqFRy75YxUfebcFNdZfw5a2LGEMyNfWcnTG6Ekqgb");
    let address1 = String::from("25zBh6DbBPQmQUGeLMaEfNAASfvxrm1GvVg3bfTZ6NkG");
    //let num = String::from("1112j");

    //let x:i32 = 100;
    //let x_bytes = x.to_be_bytes();

    let hash = address.from_base58().unwrap();
    let hash1 = address1.from_base58().unwrap();


    // println!("{}",hex::encode(&hash));
    println!("{:02x?}", &hash);
    println!("{:02x?}", &hash1);
    //println!("{:02x?}", &num);


    let decoded = bs58::decode("9axNqFRy75YxUfebcFNdZfw5a2LGEMyNfWcnTG6Ekqgb").into_vec().unwrap();
    let encoded = bs58::encode(&decoded).into_string();

    println!("{:02x?}", &decoded);
    println!("{}", &encoded);//9axNqFRy75YxUfebcFNdZfw5a2LGEMyNfWcnTG6Ekqgb


    // let input = "090A0B0C";

    // let decoded = hex::decode(input).expect("Decoding failed");

    // println!("{:?}", decoded);

    //let original_x = i32::from_be_bytes(x_bytes);
}
