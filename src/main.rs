use std::{fs, process::Command, io};

fn main() {
    let mut file = fs::File::create("/etc/systemd/system/relay-service.service").unwrap();
    let mut source = fs::File::open("relay-node.service").unwrap();
    io::copy(&mut source, &mut file).unwrap();

    let command = Command::new("systemctl")
        .arg("daemon-reload")
        .status()
        .unwrap();
    println!("{}", command);

    Command::new("systemctl").arg("start").arg("relay-service");
    Command::new("systemctl").arg("status").arg("relay-service");
}
