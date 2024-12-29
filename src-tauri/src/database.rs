use rusqlite::{Connection, Result};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Item {
  content_type: String,
  name: String,
  description: String,
  thumbnail_path: String,
  executable_path: String
}

#[tauri::command]
pub fn write_item(content_type: &str, name: &str, description: &str, thumbnail_path: &str, executable_path: &str) -> Result<(), String> {
  let item = Item {
    content_type: content_type.to_string(),
    name: name.to_string(),
    description: description.to_string(),
    thumbnail_path: thumbnail_path.to_string(),
    executable_path: executable_path.to_string()
  };

  let db = Connection::open("./test_database.db").map_err(|e| e.to_string())?;
  match db.execute(
    "
      CREATE TABLE content (
        content_type TEXT NOT NULL,
        name TEXT NOT NULL PRIMARY KEY,
        description TEXT,
        thumbnail_path TEXT,
        executable_path TEXT NOT NULL
      )
    ",
    (),
  ) {
    Ok(updated) => println!("{} rows updated", updated),
    Err(err) => println!("Update failed: {}", err),
  };

  match db.execute(
      "INSERT INTO content (content_type, name, description, thumbnail_path, executable_path) VALUES (?1, ?2, ?3, ?4, ?5)",
      (&item.content_type, &item.name, &item.description, &item.thumbnail_path, &item.executable_path)
  ) {
      Ok(updated) => println!("{} rows updated", updated),
      Err(err) => println!("Updated failed: {}", err),
  };

  Ok(())
}

#[tauri::command]
pub fn read_content() -> Result<Vec<Item>, String> {
  let db = Connection::open("./test_database.db").map_err(|e| e.to_string())?;
  let mut stmt = db.prepare("SELECT content_type, name, description, thumbnail_path, executable_path FROM content").map_err(|e| e.to_string())?;
  let item_iter = stmt.query_map([], |row| {
    Ok(Item {
      content_type: row.get(0)?,
      name: row.get(1)?,
      description: row.get(2)?,
      thumbnail_path: row.get(3)?,
      executable_path: row.get(4)?
    })
  }).map_err(|e| e.to_string())?;

  let mut items = Vec::new();
  for item in item_iter {
    items.push(item.map_err(|e| e.to_string())?);
  }

  Ok(items)
}