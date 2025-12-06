mod message_store;
mod models;
mod schema;

use crate::message_store::MessageStore;
use crate::models::Msg;
use diesel::{Connection, SqliteConnection};
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};
use std::io::{BufRead, BufReader};
use std::net::TcpStream;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

type Result<T> = anyhow::Result<T>;

fn main() -> Result<()> {
    let socket = TcpStream::connect("pi-zero-1:30003")?;
    let reader = BufReader::new(socket);

    let mut conn = SqliteConnection::establish("adsb.db")?;
    conn.run_pending_migrations(MIGRATIONS)
        .map_err(|e| anyhow::anyhow!("Failed to run database migrations: {}", e))?;

    let mut store = MessageStore::new(conn);

    for line in reader.lines() {
        handle_line(&mut store, line.map_err(|e| e.into()))
            .unwrap_or_else(|e| println!("Error: {}", e));
    }
    Ok(())
}

fn handle_line(store: &mut MessageStore, line: Result<String>) -> Result<()> {
    let line = line?;
    let msg: Msg = line.try_into()?;

    store.insert_message(&msg)?;

    println!("{msg:?}");
    Ok(())
}
