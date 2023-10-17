#![feature(let_chains)]

mod exploit;
mod structure;
mod stprotocol;

use exploit::*;
use structure::*;
use stprotocol::{ Session, Client };

fn main() {
    let mut client = Session::new("127.0.0.1:6974".to_string());

    match client.recv_stone() {
        Ok(ssh) => {
            // Access ssh.stone_status, ssh.stone_type, and ssh.stone_size here
            println!("받은거 : {:?}", ssh);
        },
        Err(_) => {
            // Handle the error case
            println!("Error receiving stone.");
        }
    }
}
