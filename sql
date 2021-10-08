use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    let conn = Connection::open_in_memory()?;

    conn.execute(
        "CREATE TABLE message (
                  id              INTEGER PRIMARY KEY,
                  sender          TEXT NOT NULL,
                  message_type    TEXT NOT NULL,
                  message_content TEXT,
                  message_time    DATETIME
                  )",
        [],
    )?;

    Ok(())
}
