use std::{fs, io, process::Command};
mod keypair_generation;
use keypair_generation::generation::keys_generate;
fn main() {
    let mut wallet = String::new();
    let mut answer = String::new();

    let wget_relay_service = Command::new("wget")
        .arg("https://centichain.org/downloads/relay-node.service")
        .status()
        .unwrap();

    println!("{}", wget_relay_service);

    let wget_relay_node = Command::new("wget")
        .arg("https://centichain.org/downloads/relay-node")
        .status()
        .unwrap();

    println!("{}", wget_relay_node);

    if wget_relay_node.success() {

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

        let cp_command = Command::new("cp")
            .arg("-r")
            .arg("relay-node")
            .arg("/etc/")
            .status()
            .unwrap();

        if cp_command.success() {
            let mut file = fs::File::create("/etc/systemd/system/relay-service.service").unwrap();
            let mut source = fs::File::open("relay-node.service").unwrap();
            io::copy(&mut source, &mut file).unwrap();

            let command = Command::new("systemctl")
                .arg("daemon-reload")
                .status()
                .unwrap();
            println!("{}", command);

            let enable = Command::new("systemctl")
                .arg("enable")
                .arg("relay-service")
                .status()
                .unwrap();
            println!("start status: {}", enable);

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
        } else {
            println!("Can not find relay node for copy!");
        }
    } else {
        println!("Can not find relay node file!, please download the latest version.");
    }
}
