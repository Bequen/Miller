use rusqlite::{Connection, Result, params};

struct ArmaDB {
    path: String,
    conn: Option<Connection>
}

impl ArmaDB {
    fn from_file(path: String) -> Self {
        let mut res: ArmaDB = ArmaDB {
            path: String::from(""),
            conn: None
        };
        res.path = path;

        res
    }

    fn connect(&mut self) {
        match Connection::open(self.path.clone()) {
            Ok(c) => {
                self.conn = Some(c);
            }, Err(e) => {
                println!("Failed opening SQLite connection because\n{}", e);
            }
        }
    }

    fn validate(&mut self) {
        self.conn.as_ref().unwrap().execute(
            "CREATE TABLE IF NOT EXISTS workshop_items (
item_id INTEGER PRIMARY KEY,
item_workshop_id TEXT NOT NULL UNIQUE,
item_name TEXT NOT NULL,
item_fmt_name TEXT
)", []).unwrap();
    }

    fn push_ws_item(&mut self, item: ArmaWorkshopItem) {
        match self.conn.as_ref().unwrap().execute(
            "INSERT INTO workshop_items (item_workshop_id, item_name, item_fmt_name) VALUES (?1, ?2, ?3)",
            params![item.item_id, item.item_name, item.item_name]
        ) {
            Ok(_o) => {},
            Err(_e) => println!("Failed pushing workshop item into DB")
        };
    }

    fn push_ws_items(&mut self, items: Vec<api::steam::WorkshopItemInfo>) {
        let mut query = String::from("INSERT OR REPLACE INTO workshop_items (item_workshop_id, item_name, item_fmt_name) VALUES");

        let mut is_first = true;

        for item in items {
            if(is_first) {
                is_first = false;
            } else {
                query.push_str(",");
            }

            query.push_str(&format!("('{}','{}','{}')", item.id, item.name.replace("'", ""), item.fmt_name));
        }

        println!("query: {}", query);

        match self.conn.as_ref().unwrap().execute(
            &query,
            []
        ) {
            Ok(_o) => {},
            Err(_e) => println!("Failed pushing workshop items into DB, because\n{}", _e)
        };
    }

    fn get_ws_items(&mut self) -> Vec<api::steam::WorkshopItemInfo> {
        let mut stmt = self.conn.as_ref().unwrap()
            .prepare("SELECT * FROM workshop_items")
            .unwrap();
        let ws_iter = stmt
            .query_map(rusqlite::NO_PARAMS, |row| Ok(api::steam::WorkshopItemInfo {
                id: row.get(1).unwrap(),
                name: row.get(2).unwrap(),
                fmt_name: row.get(3).unwrap(),
            }))
            .unwrap();

        let mut result: Vec<api::steam::WorkshopItemInfo> = Vec::new();
        for iter in ws_iter {
            result.push(iter.unwrap());
        }

        return result;
    }
}
