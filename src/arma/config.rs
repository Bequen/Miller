include!("../config.rs");

pub const ARMA_DEFAULT_DB_PATH: &str = "arma.db";

// use dirs;
// use serde_json;
use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize)]
pub struct ArmaConfig {
    instance_id: Option<String>,
    arma_api_port: Option<i32>,

    arma_db_path: Option<String>,
    bot_token: Option<String>
}

pub struct ArmaWorkshopItem {
    item_name: String,
    item_id: String
}

impl ArmaWorkshopItem {
    fn new(name: String, id: String) -> Self {
        ArmaWorkshopItem {
            item_name: name,
            item_id: id
        }
    }
}

fn save<T>(value: &T, path: String) -> Result<(), std::io::Error> where T: Serialize {
    let json = match serde_json::to_string_pretty(value) {
        Ok(o) => o,
        Err(e) => { return Err(std::io::Error::new(std::io::ErrorKind::InvalidData,
             format!("Could not convert config to JSON, because\n{}", e))) }
    };

    match std::fs::write(path, json) {
        Ok(_) => Ok(()),
        Err(e) => Err(e)
    }
}


impl Config for ArmaConfig {
    fn new() -> Self {
        ArmaConfig {
            instance_id: None,
            arma_api_port: None,
            arma_db_path: Some(String::from(dirs::home_dir().unwrap().to_str().unwrap()) + "/" + (ARMA_DEFAULT_DB_PATH)),
            bot_token: None
        }
    }

    fn from_file() -> Result<Self, String> where Self: Sized {
        let result: ArmaConfig;
        let mut config_path = std::path::PathBuf::new();

        match dirs::config_dir() {
            Some(p) => config_path = p,
            None => {
                Err(String::from("Could not find config path")).unwrap()
            }
        }

        config_path.push(CONFIG_PATH);
        config_path.push(ARMA_CONFIG_PATH);

        let contents = match std::fs::read_to_string(&*config_path) {
            Ok(c) => c,
            Err(e) => {
                println!("Error while reading request config file because\n{}", e);

                match std::fs::File::create(config_path.clone()) {
                    Ok(_) => println!("Request config file created"),
                    Err(e) => {
                        println!("Sorry, but request config file could not be created because\n{}", e);
                        return Err(format!("Sorry, but request config file could not be created because\n{}", e))
                    }
                }

                let file = std::fs::read_to_string(config_path.clone());
                if file.is_ok() {
                    match file {
                        Ok(f) => f,
                        Err(_e) => {
                            return Err(String::from("Could not read to string"));
                        }
                    }
                } else {
                    return Err(String::from("Could not open new config file, that's weird :("))
                }
            }
        };

        result = match serde_json::from_str(&*contents) {
            Ok(c) => {
                c
            },
            Err(e) => {
                println!("Cannot read json file because {}", e);
                return Ok(ArmaConfig::new());
            }
        };

        Ok(result)
    }

    fn save(&self) -> Result<(), std::io::Error> {
        let mut config_path = std::path::PathBuf::new();

        match dirs::config_dir() {
            Some(p) => config_path = p,
            None => {
                println!("Could not find config path");
                Err(String::from("Could not find config path")).unwrap()
            }
        }

        config_path.push(CONFIG_PATH);
        config_path.push(ARMA_CONFIG_PATH);

        return save(self, String::from(config_path.to_str().unwrap()));
    }

    fn validate() -> Result<(), std::io::Error> {
        // First test if the file can be opened/created
        let mut arma_config = match ArmaConfig::from_file() {
            Ok(c) => c,
            Err(e) => {
                println!("Could not create config file: {}", e);
                return Err(std::io::Error::new(
                    std::io::ErrorKind::PermissionDenied,
                    format!("Could not create config file: {}", e))
                );
            }
        };

        match arma_config.instance_id {
            Some(_v) => { arma_config.instance_id = Some(_v) },
            None => {
                println!("Arma API Server Instance ID is not specified");
                print!("Instance ID:");
                use std::io::{stdin,stdout,Write};
                let mut s = String::new();
                let _ = stdout().flush();
                stdin().read_line(&mut s).expect("Invalid");
                if let Some('\n') = s.chars().next_back() {
                    s.pop();
                }

                if let Some('\r') = s.chars().next_back() {
                    s.pop();
                }
                println!("Arma API Server Instance Id: {}", s);
                arma_config.instance_id = Some(s);

            }
        }

        match arma_config.bot_token {
            Some(_v) => { arma_config.bot_token = Some(_v) },
            None => {
                println!("Bot Token is not specified");
                print!("Token:");
                use std::io::{stdin,stdout,Write};
                let mut s = String::new();
                let _ = stdout().flush();
                stdin().read_line(&mut s).expect("Invalid");
                if let Some('\n') = s.chars().next_back() {
                    s.pop();
                }

                if let Some('\r') = s.chars().next_back() {
                    s.pop();
                }
                println!("Bot Token is: {}", s);
                arma_config.bot_token = Some(s.parse().unwrap());
            }
        }

        match arma_config.arma_api_port {
            Some(_v) => { },
            None => {
                println!("Arma API Server Port is not specified");
                print!("Port:");
                use std::io::{stdin,stdout,Write};
                let mut s = String::new();
                let _ = stdout().flush();
                stdin().read_line(&mut s).expect("Invalid");
                if let Some('\n') = s.chars().next_back() {
                    s.pop();
                }

                if let Some('\r') = s.chars().next_back() {
                    s.pop();
                }
                println!("Arma API Server Instance Id: {}", s);
                arma_config.arma_api_port = Some(s.parse().unwrap());

            }
        }

        match arma_config.save() {
            Ok(()) => {
                println!("Config successfully validated and saved");
            }, Err(e) => {
                println!("Failed to save validated config");
                return Err(e);
            }
        }

        Ok(())
    }
}

// Outputs mods.sh content
fn make_mods_list(mut db: ArmaDB) -> String {
    let all_items = db.get_ws_items();

    let mut result = String::new();

    result.push_str(&format!("MODS=("));

    let mut is_first = true;
    for mut item in &all_items {
        if !is_first {
            result.push(' ');
        } else {
            is_first = false;
        }

        result.push_str(&format!("\"{}\"", item.id));
    }
    result.push_str(&format!(")\n"));
    result.push_str(&format!("NAMES=("));

    let mut is_first = true;
    for mut item in &all_items {
        if !is_first {
            result.push(' ');
        } else {
            is_first = false;
        }

        result.push_str(&format!("\"{}\"", item.fmt_name));
    }
    result.push_str(&format!(")"));

    return result;
}
