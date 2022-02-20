// For interacting with steam api
//

use curl::easy::Easy;
use regex::Regex;

pub struct WorkshopItemInfo {
    pub id: String,
    pub name: String,
    pub fmt_name: String
}

pub fn fmt_name(mut item: &mut WorkshopItemInfo) {
    item.fmt_name = item.fmt_name.replace(":", "");
    item.fmt_name = item.fmt_name.replace("-", "");
    item.fmt_name = item.fmt_name.replace("'", "");

    let mut regex = Regex::new(r"\(([^)]+)\)|\.[0-9a-zA-Z]+$").unwrap();
    item.fmt_name = String::from(regex.replace_all(&item.fmt_name, ""));

    regex = Regex::new(r"[ \t]{2,}").unwrap();
    item.fmt_name = String::from(regex.replace_all(&item.fmt_name, " "));

    item.fmt_name = String::from(item.fmt_name.trim());
    item.fmt_name = item.fmt_name.replace(" ", "_");
}

pub fn get_items_info(items: Vec<String>) -> Result<Vec<WorkshopItemInfo>, String> {
    let mut dst = Vec::new();
    let mut easy = Easy::new();

    easy.url("http://api.steampowered.com/ISteamRemoteStorage/GetPublishedFileDetails/v1/").unwrap();

    let mut items_fields: String = String::new();
    items_fields.push_str(&format!("itemcount={}", items.len()));
    let mut i = 0;
    for item in items {
        items_fields.push_str(&format!("&publishedfileids[{}]={}", i, item));
        i += 1;
    }
    items_fields.push_str(&format!("&format=json"));
    //println!("{}", items_fields);

    //let mut list = curl::easy::List::new();
    //list.append("Content-Type: application/json").unwrap();
    //easy.http_headers(list).unwrap();
    easy.post(true);
    easy.post_fields_copy(items_fields.as_bytes());

    {
        let mut transfer = easy.transfer();
        transfer.write_function(|data| {
            dst.extend_from_slice(data);
            Ok(data.len())
        }).unwrap();
        transfer.perform().unwrap();
    }

    let txt = match std::str::from_utf8(&dst[..]) {
        Ok(r) => r,
        Err(e) => {
            return Err(format!("Could not understand what server is saying, {}", e.to_string()));
        }
    };


    let mut result: Vec<WorkshopItemInfo> = Vec::new();

    let json: serde_json::Value = match serde_json::from_str(txt) {
        Ok(v) => v,
        Err(e) => {
            return Err(format!("Could not understand what server is saying, {}", e.to_string()));
        }
    };
    //println!("{}", serde_json::to_string_pretty(&json).unwrap());

    let file_details = json.get("response").unwrap().get("publishedfiledetails").unwrap().as_array().unwrap();

    for file in file_details {
        let mut item = WorkshopItemInfo {
            id: String::from(file.get("publishedfileid").unwrap().as_str().unwrap()),
            name: String::from(file.get("title").unwrap().as_str().unwrap()),
            fmt_name: String::from(file.get("title").unwrap().as_str().unwrap())
        };

        fmt_name(&mut item);

        result.push(item);
    }

    Ok(result)
}
