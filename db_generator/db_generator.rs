use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    let conn = Connection::open("/tmp/rust_project.db")?;

    conn.execute(
        "CREATE TABLE message (
                  id              INTEGER PRIMARY KEY,
                  sender          TEXT NOT NULL,
                  message_type    TEXT NOT NULL,
                  message_content TEXT
                  )",
        [],
    )?;

    Ok(())
}