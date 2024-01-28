use std::process::Command;

pub fn linux_mongo_install() {
    Command::new("sudo")
        .arg("apt")
        .arg("autoremove")
        .arg("-y")
        .output()
        .expect("error from autoremove");
    let check_mongo = Command::new("mongosh").output();

    match check_mongo {
        Ok(_) => println!("monogodb is installed"),
        Err(_) => {
            println!("installing mongodb...");
            let curl = Command::new("sudo")
                .arg("apt-get")
                .arg("install")
                .arg("gnupg")
                .arg("curl")
                .output()
                .expect("curl error!");
            println!("{}", format!("{:?}", curl));

            let gpg = Command::new("sh")
            .arg("-c")
            .arg("wget -qO - https://www.mongodb.org/static/pgp/server-6.0.asc |  gpg --dearmor | sudo tee /usr/share/keyrings/mongodb.gpg > /dev/null")
            .output()
            .expect("wget error!");
            println!("{}", format!("{:?}", gpg));

            let deb = Command::new("sh")
                    .arg("-c")
                    .arg("echo \"deb [ arch=amd64,arm64 signed-by=/usr/share/keyrings/mongodb.gpg ] https://repo.mongodb.org/apt/ubuntu jammy/mongodb-org/6.0 multiverse\" | sudo tee /etc/apt/sources.list.d/mongodb-org-6.0.list")
                    .output().expect("command can't run!");
            println!("{}", format!("{:?}", deb));

            let update = Command::new("sudo")
                .arg("apt")
                .arg("update")
                .output()
                .expect("update command problem!");
            println!("{}", format!("{:?}", update));

            let install_mng = Command::new("sudo")
                .arg("apt")
                .arg("install")
                .arg("-y")
                .arg("mongodb-org")
                .output()
                .expect("install mongod error!");
            println!("{}", format!("{:?}", install_mng));

            Command::new("sudo")
                .arg("systemctl")
                .arg("start")
                .arg("mongod")
                .output()
                .expect("start mongod problem!");

            let daemon_reload = Command::new("sudo")
                .arg("systemctl")
                .arg("daemon-reload")
                .output()
                .expect("daemon-reload command problem!");
            println!("{}", format!("{:?}", daemon_reload));

            let mongod_enable = Command::new("sudo")
                .arg("systemctl")
                .arg("enable")
                .arg("mongod")
                .output();

            match mongod_enable {
                Ok(_) => {
                    println!("mongo db is installed");
                }
                Err(e) => println!("{}", format!("{:?}", e)),
            }
        }
    }
}
