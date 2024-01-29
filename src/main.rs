use std::{
    fs, io,
    process::{Command, ExitStatus},
};
mod keypair_generation;
use keypair_generation::generation::keys_generate;

fn main() {
    let mut wallet = String::new();
    let mut answer = String::new();

    let relay_service_exist = fs::metadata("/etc/relay-node.service").is_ok();

    if !relay_service_exist {
        let wget_relay_service = Command::new("wget")
            .arg("-P")
            .arg("/etc/")
            .arg("https://centichain.org/downloads/relay-node.service")
            .status()
            .unwrap();
        println!("{}", wget_relay_service);
    } else {
        let rm_r_service = Command::new("rm")
            .arg("/etc/relay-node.service")
            .status()
            .unwrap();
        println!("{}", rm_r_service);
        let wget_relay_service = Command::new("wget")
            .arg("-P")
            .arg("/etc/")
            .arg("https://centichain.org/downloads/relay-node.service")
            .status()
            .unwrap();
        println!("{}", wget_relay_service);
    }

    let relay_node_exist = fs::metadata("/etc/relay-node").is_ok();
    if !relay_node_exist {
        let wget_relay_node = Command::new("wget")
            .arg("-P")
            .arg("/etc/")
            .arg("https://centichain.org/downloads/relay-node")
            .status()
            .unwrap();

        println!("{}", wget_relay_node);
        handle_relay_node(wget_relay_node, &mut answer, &mut wallet);
    } else {
        let rm_r_service = Command::new("rm").arg("/etc/relay-node").status().unwrap();
        println!("{}", rm_r_service);
        let wget_relay_node = Command::new("wget")
            .arg("-P")
            .arg("/etc/")
            .arg("https://centichain.org/downloads/relay-node")
            .status()
            .unwrap();

        println!("{}", wget_relay_node);
        handle_relay_node(wget_relay_node, &mut answer, &mut wallet);
    }
}

fn handle_relay_node(wget_relay_node: ExitStatus, answer: &mut String, wallet: &mut String) {
    if wget_relay_node.success() {
        let relays_file_exist = fs::metadata("/etc/relays.dat").is_ok();

        if !relays_file_exist {
            let wget_relays_dat = Command::new("wget")
                .arg("-P")
                .arg("/etc/")
                .arg("https://centichain.org/downloads/relays.dat")
                .status()
                .unwrap();
            get_relays_datfile(wget_relays_dat, answer, wallet)
        } else {
            let rm_r_service = Command::new("rm").arg("/etc/relays.dat").status().unwrap();
            println!("{}", rm_r_service);
            let wget_relays_dat = Command::new("wget")
                .arg("-P")
                .arg("/etc/")
                .arg("https://centichain.org/downloads/relays.dat")
                .status()
                .unwrap();
            get_relays_datfile(wget_relays_dat, answer, wallet)
        }
    } else {
        println!("Can not find relay node file!, please download the latest version.");
    }
}

fn get_relays_datfile(wget_relays_dat: ExitStatus, answer: &mut String, wallet: &mut String) {
    if wget_relays_dat.success() {
        loop {
            answer.clear();
            wallet.clear();
            println!("Enter phrases key or enter 'N' to generate one:");
            std::io::stdin().read_line(answer).unwrap();
            keys_generate(answer.trim().to_string(), wallet);
            if wallet.clone() != "emptey".to_string() {
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
}
