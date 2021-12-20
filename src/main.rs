use rust_base58::{ToBase58, FromBase58};
use solana_sdk::bs58;
use hex;

fn main() {
    let address1 = String::from("9axNqFRy75YxUfebcFNdZfw5a2LGEMyNfWcnTG6Ekqgb");
    let address2 = String::from("25zBh6DbBPQmQUGeLMaEfNAASfvxrm1GvVg3bfTZ6NkG");

    let bytes1 = address1.from_base58().unwrap();
    let bytes2 = address2.from_base58().unwrap();

    println!("9axNqFRy75YxUfebcFNdZfw5a2LGEMyNfWcnTG6Ekqgb\n{:02x?}", &bytes1);
    println!("25zBh6DbBPQmQUGeLMaEfNAASfvxrm1GvVg3bfTZ6NkG\n{:02x?}", &bytes2);

    let decoded = bs58::decode("9axNqFRy75YxUfebcFNdZfw5a2LGEMyNfWcnTG6Ekqgb").into_vec().unwrap();
    let encoded = bs58::encode(&decoded).into_string();

    println!("Base58 decoded 9axNqFRy75YxUfebcFNdZfw5a2LGEMyNfWcnTG6Ekqgb\n{:02x?}", &decoded);
    println!("Base58 encoded 9axNqFRy75YxUfebcFNdZfw5a2LGEMyNfWcnTG6Ekqgb\n{}", &encoded);
}
