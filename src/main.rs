use std::{fs, io, process::Command};
mod keypair_generation;
use keypair_generation::generation::keys_generate;
fn main() {
    let mut wallet = String::new();
    let mut answer = String::new();

    let wget_relay_service = Command::new("wget")
        .arg("-P")
        .arg("/etc/")
        .arg("https://centichain.org/downloads/relay-node.service")
        .status()
        .unwrap();

    println!("{}", wget_relay_service);

    let wget_relay_node = Command::new("wget")
        .arg("-P")
        .arg("/etc/")
        .arg("https://centichain.org/downloads/relay-node")
        .status()
        .unwrap();

    println!("{}", wget_relay_node);

    if wget_relay_node.success() {
        let wget_relay_dat = Command::new("wget")
            .arg("-P")
            .arg("/etc/")
            .arg("https://centichain.org/downloads/relays.dat")
            .status()
            .unwrap();

        if wget_relay_dat.success() {
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

            let mut service_file =
                fs::File::create("/etc/systemd/system/relay-service.service").unwrap();
            let mut source_file = fs::File::open("/etc/relay-node.service").unwrap();
            io::copy(&mut source_file, &mut service_file).unwrap();

            let chmod = Command::new("chmod")
                .arg("777")
                .arg("/etc/relay-node")
                .status()
                .unwrap();

            println!("{}", chmod);

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
            println!("Problem in get relays.dat file!");
        }
    } else {
        println!("Can not find relay node file!, please download the latest version.");
    }
}
