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
            .arg("curl -fsSL https://pgp.mongodb.com/server-7.0.asc | sudo gpg -o /usr/share/keyrings/mongodb-server-7.0.gpg --dearmor")
            .output()
            .expect("failed to execute process");
            println!("{}", format!("{:?}", gpg));

            let deb = Command::new("sh")
                    .arg("-c")
                    .arg("echo \"deb [ arch=amd64,arm64 signed-by=/usr/share/keyrings/mongodb-server-7.0.gpg ] https://repo.mongodb.org/apt/ubuntu jammy/mongodb-org/7.0 multiverse\" | sudo tee /etc/apt/sources.list.d/mongodb-org-7.0.list")
                    .output().expect("command can't run!");
            println!("{}", format!("{:?}", deb));

            let update = Command::new("sudo")
                .arg("apt-get")
                .arg("update")
                .output()
                .expect("update command problem!");
            println!("{}", format!("{:?}", update));

            let install_mng = Command::new("sudo")
                .arg("apt-get")
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
