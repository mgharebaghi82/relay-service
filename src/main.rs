use std::{fs, io, process::Command};
mod keypair_generation;
use keypair_generation::generation::keys_generate;
fn main() {
    let mut wallet = String::new();
    let mut answer = String::new();
    loop {
        answer.clear();
        wallet.clear();
        println!("Enter phrases key or enter 'N' to generate one:");
        std::io::stdin().read_line(&mut answer).unwrap();
        keys_generate(answer.clone().trim().to_string(), &mut wallet);
        if wallet != "emptey".to_string() {
            break;
        }
    }
    let mut file = fs::File::create("/etc/systemd/system/relay-service.service").unwrap();
    let mut source = fs::File::open("relay-node.service").unwrap();
    io::copy(&mut source, &mut file).unwrap();

    let command = Command::new("systemctl")
        .arg("daemon-reload")
        .status()
        .unwrap();
    println!("{}", command);

    let start = Command::new("systemctl")
        .arg("start")
        .arg("relay-service")
        .status()
        .unwrap();
    println!("start status: {}", start);
    let status = Command::new("systemctl")
        .arg("status")
        .arg("relay-service")
        .status()
        .unwrap();
    println!("status code: {}", status);
}
