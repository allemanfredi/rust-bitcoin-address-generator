
extern crate secp256k1;
extern crate crypto;
extern crate rand;
extern crate rust_base58;
extern crate bech32;

use secp256k1::Secp256k1;
use rand::rngs::OsRng;
use std::env;
use hex::FromHex;


mod address;

fn main(){

    let args: Vec<String> = env::args().collect();
    
    //generste private and public keys
    let secp256k1 = Secp256k1::new();
    let mut rng = OsRng::new().expect("OsRng");
    let (_secret_key, public_key) = secp256k1.generate_keypair(&mut rng);
    let serialized_public_key = public_key.serialize();

    match args.len() {
        3 => {
            let cmd = &args[1];
            let what = &args[2];
            match &cmd[..] {
                "--type" => {
                    match &what[..] {
                        "p2pkh" => {
                            let address = address::BitcoinAddress::p2pkh(&serialized_public_key);
                            print!("Private Key : {}\n", _secret_key);
                            print!("Public Key : {}\n", public_key);
                            print!("Address : {}\n", address);
                        },
                        "p2wpkh" => {
                            let address = address::BitcoinAddress::p2wpkh(&serialized_public_key);
                            print!("Private Key : {}\n", _secret_key);
                            print!("Public Key : {}\n", public_key);
                            print!("{}\n", address);
                        }
                         _ => {
                            eprintln!("error: invalid address type");
                        },  
                    }
                }
                _ => {
                    eprintln!("error: invalid command");
                    help();
                },
            }
        },
        5 => {
            let cmd = &args[1];
            let what = &args[2];
            let cmds = &args[3];
            let script = &args[4];

            let pscript = parse_script(String::from(script.to_string()));

            match &cmds[..] {
                "--script" => {
                    match &cmd[..] {
                        "--type" => {
                            match &what[..] {
                                "p2sh" => {
                                    let address = address::BitcoinAddress::p2sh(&pscript);
                                    print!("Address : {}\n", address);
                                },
                                "p2wsh" => {
                                    let address = address::BitcoinAddress::p2wsh(&pscript);
                                    print!("Address : {}\n", address);
                                }
                                _ => {
                                    eprintln!("error: invalid address type");
                                },  
                            }
                        }
                        _ => {
                            eprintln!("error: invalid command");
                            help();
                        },
                    }
                }
                _ => {
                    eprintln!("error: invalid command");
                    help();
                },
            }
        },
        _ => {
            // show a help message
            eprintln!("error: invalid command");
            help()
        }
    }

}   

fn parse_script(script :String) -> Vec<u8>{
    let split_script = script.split(",");
    let vec_script: Vec<&str> = split_script.collect();
    let mut res : Vec<u8>= Vec::new(); 
    for el in vec_script {
        let elh = Vec::from_hex(el).unwrap();
        res.extend(elh);
    }
    return res;
}

fn help(){
    print!("USAGE:\n --type [p2pkh,p2wpkh] \n --type [p2sh,p2wsh] --script [script bytes] \n ex: --type p2sh --script '00,14...'")
}


