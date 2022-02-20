
use quick_xml::Reader;
use quick_xml::events::Event;

use std::io::{stdout, Write};

use curl::easy::Easy;

fn download_mod_list(url: String) -> Result<String, String> {
    let mut dst = Vec::new();
    let mut easy = Easy::new();
    easy.url(&url).unwrap();
    let mut list = curl::easy::List::new();
    list.append("Content-Type: application/json").unwrap();
    easy.http_headers(list).unwrap();

    {
        let mut transfer = easy.transfer();
        transfer.write_function(|data| {
            dst.extend_from_slice(data);
            Ok(data.len())
        }).unwrap();
        transfer.perform().unwrap();
    }

    let result = match std::str::from_utf8(&dst[..]) {
        Ok(r) => r,
        Err(e) => {
            return Err(format!("Could not understand what server is saying, {}", e.to_string()));
        }
    };

    Ok(String::from(result))
}


fn indent(size: usize) -> String {
    const INDENT: &'static str = "    ";
    (0..size).map(|_| INDENT)
             .fold(String::with_capacity(size*INDENT.len()), |r, s| r + s)
}


fn parse_arma_mod_list(xml: String) -> Vec<String> {
    let mut reader = Reader::from_str(&xml);
    reader.trim_text(true);
    reader.check_end_names(false);

    let mut count = 0;
    //let mut txt = Vec::new();
    let mut buf = Vec::new();

    let mut result: Vec<String> = Vec::new();

    // The `Reader` does not implement `Iterator` because it outputs borrowed data (`Cow`s)
    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => {
                let mut is_item = false;
                let mut item: Option<String> = None;

                for val in e.html_attributes() {
                    if String::from_utf8_lossy(val.as_ref().unwrap().key) == String::from("data-type") {
                        if String::from_utf8_lossy(&val.as_ref().unwrap().value) == String::from("Link") {
                            //println!("{}", String::from_utf8_lossy(&val.unwrap().value));
                            is_item = true;
                            if item != None {
                                //println!("{}", item.as_ref().unwrap());
                                result.push(item.clone().unwrap());
                                break;
                            }
                        }
                    } else if String::from_utf8_lossy(val.as_ref().unwrap().key) == String::from("href") {
                        //println!("{}", String::from_utf8_lossy(&val.unwrap().value));
                        let s = String::from_utf8_lossy(&val.as_ref().unwrap().value).to_string();
                        s.split("_").last().unwrap();
                        item = Some(String::from(s.split("=").last().unwrap()));
                        if is_item {
                            //println!("{}", item.as_ref().unwrap());
                            result.push(item.clone().unwrap());
                            break;
                        }
                    }
                }
            },
            Ok(Event::Text(e)) => {},//txt.push(e.unescape_and_decode(&reader).unwrap_or(String::from(""))),
            Ok(Event::Eof) => break, // exits the loop when reaching end of file
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (), // There are several other `Event`s we do not consider here
        }

        // if we don't keep a borrow elsewhere, we can clear the buffer to keep memory usage low
        buf.clear();
    }

    return result;
}

fn parse_mod_list(xml: String) -> Vec<String> {
    let mut reader = Reader::from_str(&xml);
    reader.trim_text(true);
    reader.check_end_names(false);

    let mut count = 0;
    //let mut txt = Vec::new();
    let mut buf = Vec::new();

    let mut result: Vec<String> = Vec::new();

    // The `Reader` does not implement `Iterator` because it outputs borrowed data (`Cow`s)
    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => {
                let mut is_item = false;
                let mut item: Option<String> = None;

                for val in e.html_attributes() {
                    if String::from_utf8_lossy(val.as_ref().unwrap().key) == String::from("class") {
                        if String::from_utf8_lossy(&val.as_ref().unwrap().value) == String::from("collectionItem") {
                            //println!("{}", String::from_utf8_lossy(&val.unwrap().value));
                            is_item = true;
                            if item != None {
                                //println!("{}", item.as_ref().unwrap());
                                result.push(item.clone().unwrap());
                                break;
                            }
                        }
                    } else if String::from_utf8_lossy(val.as_ref().unwrap().key) == String::from("id") {
                        //println!("{}", String::from_utf8_lossy(&val.unwrap().value));
                        let s = String::from_utf8_lossy(&val.as_ref().unwrap().value).to_string();
                        s.split("_").last().unwrap();
                        item = Some(String::from(s.split("_").last().unwrap()));
                        if is_item {
                            //println!("{}", item.as_ref().unwrap());
                            result.push(item.clone().unwrap());
                            break;
                        }
                    }
                }
            },
            Ok(Event::Text(e)) => {},//txt.push(e.unescape_and_decode(&reader).unwrap_or(String::from(""))),
            Ok(Event::Eof) => break, // exits the loop when reaching end of file
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (), // There are several other `Event`s we do not consider here
        }

        // if we don't keep a borrow elsewhere, we can clear the buffer to keep memory usage low
        buf.clear();
    }

    return result;
}

extern crate chrono;

use chrono::prelude::*;

#[derive(serde::Serialize, serde::Deserialize)]
struct ModsPresetWorkshopItem {
    id: String,
    enabled: bool
}

fn make_mod_list_unix(items :Vec<api::steam::WorkshopItemInfo>) -> String  {
    let mut mods = Vec::new();

    for item in items {
        let mod_item = ModsPresetWorkshopItem {
            id: item.id,
            enabled: true
        };

        mods.push(mod_item);
    }

    let preset = serde_json::json!({
        "mods": {
            "custom": [],
            "workshop": serde_json::json!(mods)
        }
    });

    let mut dir: std::path::PathBuf = std::path::PathBuf::new();
    dir.push(std::env::temp_dir());

    dir.push(format!("temppreset_{}.a3ulml", chrono::Local::now().format("%d_%m_%Y-%H_%M_%S")));
    //let path_str = dir.display();
    //println!("Path {}", path_str);

    let mut file = std::fs::File::create(&dir).unwrap();
    writeln!(file, "{}", serde_json::to_string_pretty(&preset).unwrap()).unwrap();

    // println!("{}", serde_json::to_string_pretty(&preset).unwrap());
    // output.push(format!("file:///{}", path_str));
    //
    return String::from(dir.to_str().unwrap());

    //return serde_json::to_string_pretty(&preset).unwrap();
}

fn make_mod_list_win(items: Vec<api::steam::WorkshopItemInfo>) -> String {
    // makes a path for the output
    let mut dir: std::path::PathBuf = std::path::PathBuf::new();
    dir.push(std::env::temp_dir());

    dir.push(format!("temppreset_{}.a3ulml", chrono::Local::now().format("%d_%m_%Y-%H_%M_%S")));

    // creates file to write to
    let mut file = std::fs::File::create(&dir).unwrap();

    // open prefix and insert it
    let prefix = String::from_utf8_lossy(include_bytes!("../../templates/prefix.html"));
    writeln!(file, "{}", prefix).unwrap();

    // lets insert all the mods
    for item in items {
        writeln!(file,
"            <tr data-type=\"ModContainer\">
                 <td data-type=\"DisplayName\">{}</td>
                 <td>
                     <span class=\"from-steam\">Steam</span>
                 </td>
                 <td>
                     <a href=\"http://steamcommunity.com/sharedfiles/filedetails/?id={1}\" data-type=\"Link\">http://steamcommunity.com/sharedfiles/filedetails/?id={1}</a>
                 </td>
             </tr>"
            , item.name, item.id).unwrap();
    }
    let suffix = String::from_utf8_lossy(include_bytes!("../../templates/suffix.html"));
    writeln!(file, "{}", suffix).unwrap();

    String::from(dir.to_str().unwrap())
}
