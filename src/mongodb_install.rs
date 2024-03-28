use std::{fs::OpenOptions, io::Write, process::Command, time::Duration};

use mongodb::{bson::doc, Client};

pub async fn linux_mongo_install() -> Result<(), String> {
    // Build the configuration for the replica set
    let rs_config = doc! {
        "_id": "myReplSet",  // Replace with your desired replica set name
        "members": [
            { "_id": 0, "host": "localhost:27017" }, // Member 1
        ]
    };

    Command::new("sudo")
        .arg("apt")
        .arg("autoremove")
        .arg("-y")
        .output()
        .expect("error from autoremove");
    let check_mongo = Command::new("mongosh").output();

    match check_mongo {
        Ok(_) => {
            let client = Client::with_uri_str("mongodb://127.0.0.1:27017").await;
            let admin_db = client.unwrap().database("admin");
            let config = admin_db
                .run_command(doc! {"replSetGetConfig": 1}, None)
                .await;

            match config {
                Ok(_) => {
                    println!("monogodb is installed and replica set is configured");
                    Ok(())
                }
                Err(_) => {
                    //set replica set configuration
                    let mongod_conf_file = OpenOptions::new()
                        .append(true)
                        .write(true)
                        .open("/etc/mongod.conf");
                    match mongod_conf_file {
                        Ok(mut file) => {
                            // Write the replica set configuration to the file
                            let config_content = "replication:\n  replSetName: \"rs0\"";
                            writeln!(file, "{}", config_content).unwrap();
                        }
                        Err(e) => {
                            println!("{}", format!("{:?}", e));
                            return Err("error from create mongod.conf file".to_string());
                        }
                    }

                    //restart mongod for set replica
                    match Command::new("sudo")
                        .arg("systemctl")
                        .arg("restart")
                        .arg("mongod")
                        .output()
                    {
                        Ok(_) => {
                            println!("sleep for 10 seconds to restart mongod");
                            tokio::time::sleep(Duration::from_secs(10)).await;
                            println!("start to configure replica set");
                            //final set replica set configuration
                            admin_db
                                .run_command(doc! {"replSetInitiate": rs_config}, None)
                                .await
                                .unwrap();
                            println!("monogodb is installed");
                            Ok(())
                        }
                        Err(e) => {
                            println!("{}", e);
                            return Err("".to_string());
                        }
                    }
                }
            }
        }
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

            //set replica set configuration
            let mongod_conf_file = OpenOptions::new()
                .append(true)
                .write(true)
                .open("/etc/mongod.conf");
            match mongod_conf_file {
                Ok(mut file) => {
                    // Write the replica set configuration to the file
                    let config_content = "replication:\n  replSetName: \"rs0\"";
                    writeln!(file, "{}", config_content).unwrap();
                }
                Err(e) => {
                    println!("{}", format!("{:?}", e));
                    return Err("error from create mongod.conf file".to_string());
                }
            }

            //restart mongod for set replica
            match Command::new("sudo")
                .arg("systemctl")
                .arg("restart")
                .arg("mongod")
                .output()
            {
                Ok(_) => {
                    println!("sleep for 10 seconds to restart mongod");
                    tokio::time::sleep(Duration::from_secs(10)).await;
                    println!("start to configure replica set");
                    let client = Client::with_uri_str("mongodb://127.0.0.1:27017").await;
                    let admin_db = client.unwrap().database("admin");

                    //final set replica set configuration
                    admin_db
                        .run_command(doc! {"replSetInitiate": rs_config}, None)
                        .await
                        .unwrap();
                    println!("monogodb is installed");
                    Ok(())
                }
                Err(e) => {
                    println!("{}", e);
                    return Err("".to_string());
                }
            }
        }
    }
}
