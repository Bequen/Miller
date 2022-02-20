include!("arma/config.rs");
include!("arma/db.rs");
include!("arma/workshop.rs");

mod api;

mod commands;
use crate::commands::*;
use clap::Parser;

use api::steam;

use std::str::FromStr;

#[derive(Parser)]
struct Cli {
    /// Mod to append into database
    #[clap(short, long)]
    append: Option<String>,
    
    /// Prints out the preset
    #[clap(short, long)]
    print: Option<String>,

    /// Output path
    #[clap(short, long)]
    output: Option<String>
}

#[tokio::main]
async fn main() {
    println!("Starting up!");

    let args = Cli::parse();

    /* validates and opens Arma config */
    match ArmaConfig::validate() {
        Ok(_ok) => {},
        Err(e) => {
            println!("failed validating arma config because {},\nexitting", e.to_string());
            return;
        }
    }

    let arma_config:ArmaConfig = ArmaConfig::from_file().unwrap();

    /* opens Arma DB on path specified in Arma Config */
    let mut arma_db: ArmaDB = ArmaDB::from_file(arma_config.arma_db_path.unwrap());
    arma_db.connect();
    arma_db.validate();

    if let Some(ws) = args.append {
        if let Ok(ws_info) = steam::get_items_info(vec!{ws}) {
            println!("appending {}", ws_info[0].fmt_name);
            arma_db.push_ws_items(ws_info);
        }
    }

    if let Some(print) = args.print {
        let mut preset_path = String::new();
        if print.starts_with("win") {
            preset_path = make_mod_list_win(arma_db.get_ws_items());
        } else if print.starts_with("unix") {
            preset_path = make_mod_list_unix(arma_db.get_ws_items());
        }

        if let Some(output) = args.output {
            std::fs::copy(preset_path, output);
        } else {
            println!("{}", std::fs::read_to_string(preset_path).unwrap());
        }
    }

    return; 

    let ws_items = arma_db.get_ws_items();
    let preset = make_mod_list_win(ws_items); 
    let preset_read = std::fs::read_to_string(preset)
        .expect("Something went wrong reading the prefix file");

    println!("{}", preset_read);

    let mods_list = make_mods_list(arma_db);
    println!("Mods: {}", mods_list);

    return;
}
