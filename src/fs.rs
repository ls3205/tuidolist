use crate::AppState;
use crate::TodoItem;
use serde::Deserialize;
use serde::Serialize;
use std::env::home_dir;
use std::fs;
use std::fs::OpenOptions;
use std::io::Read;
use std::io::Write;

#[derive(Deserialize, Serialize)]
struct TodoJSON {
    items: Vec<JSONItem>,
}

#[derive(Deserialize, Serialize)]
struct JSONItem {
    is_done: bool,
    name: String,
    description: String,
}

pub fn read() -> Vec<TodoItem> {
    let mut out: Vec<TodoItem> = Vec::new();

    let path = home_dir().unwrap().join(".tuidolist/items.json");

    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("Failed to create directory");
    }

    let mut file = OpenOptions::new()
        .create(true)
        .truncate(false)
        .write(true)
        .read(true)
        .open(&path)
        .expect("Failed to create or open the file");

    // write default JSON in the event of needing to create a new file
    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("Failed to read file");
    if content.trim().is_empty() {
        content = String::from("{\"items\":[]}");
        file.set_len(0).expect("Failed to truncate file");
        file.write_all(content.as_bytes())
            .expect("Failed to write default JSON");
    }

    let json: TodoJSON =
        serde_json::from_str(&fs::read_to_string(&path).expect("Failed to read file"))
            .expect("Failed to parse JSON");

    json.items.iter().for_each(|item| {
        out.push(TodoItem {
            is_done: item.is_done,
            name: item.name.clone(),
            description: item.description.clone(),
        })
    });

    out
}

pub fn write(app_state: &mut AppState) {
    let path = home_dir().unwrap().join(".tuidolist/items.json");

    let todo_json = TodoJSON {
        items: app_state
            .items
            .iter()
            .map(|item| JSONItem {
                is_done: item.is_done,
                name: item.name.clone(),
                description: item.description.clone(),
            })
            .collect(),
    };

    let json_string = serde_json::to_string_pretty(&todo_json).expect("Failed to serialize JSON");

    fs::write(&path, json_string).expect("Failed to write file");
}
