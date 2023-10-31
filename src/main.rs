use std::{fs::{self, File}, io,io::Write, process::Command, env};
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

    let mut path_of_cfg = File::create("/etc/relay_source.conf").unwrap();
    let path = format!("PATH={}\\relay-service", env::current_dir().unwrap().display().to_string());
    let mut path_file = File::create("./SERVICE_PATH.conf").unwrap();
    write!(path_file, "{}", path).unwrap();
    io::copy(&mut path_file, &mut path_of_cfg).unwrap();

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
