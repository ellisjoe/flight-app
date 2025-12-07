mod errors;
mod message_store;
mod models;
mod schema;

use crate::errors::Error;
use crate::errors::Result;
use crate::message_store::MessageStore;
use crate::models::Msg;
use axum::extract::State;
use axum::routing::{get, post};
use axum::{Json, Router};
use tower_http::cors::CorsLayer;
use chrono::{DateTime, TimeZone, Utc};
use diesel::{Connection, SqliteConnection};
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use tokio::task;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

#[tokio::main]
async fn main() -> Result<()> {
    let mut conn = SqliteConnection::establish("adsb.db")
        .map_err(|e| Error::InternalError(format!("Could not connect to database: {}", e)))?;
    conn.run_pending_migrations(MIGRATIONS)
        .map_err(|e| Error::InternalError(format!("Failed to run database migrations: {}", e)))?;
    let conn = Arc::new(Mutex::new(conn));

    let mut store = MessageStore::new(conn);

    // Indexing
    let socket = TcpStream::connect("pi-zero-1:30003")
        .map_err(|e| Error::InternalError(format!("Could not connect to ADSB source: {}", e)))?;
    let reader = BufReader::new(socket);

    // TODO(jellis): How can I remove this?
    let mut cloned_store = store.clone();
    let indexer = task::spawn_blocking(move || {
        for line in reader.lines() {
            handle_line(
                &mut cloned_store,
                line.map_err(|e| Error::InternalError(format!("Failed to read line: {e}"))),
            )
            .unwrap_or_else(|e| println!("Error: {}", e));
        }
    });

    // Server
    let app = Router::new()
        .route("/planes", post(planes_handler))
        .layer(CorsLayer::permissive())
        .with_state(Arc::new(store.clone()));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app)
        .await
        .map_err(|e| Error::InternalError(format!("Failed to start web server: {}", e)))?;

    Ok(())
}

#[derive(Deserialize, Debug)]
struct Query {
    start: Option<DateTime<Utc>>,
    end: Option<DateTime<Utc>>,
}

#[derive(Serialize, Default)]
struct Plane {
    aircraft_id: String,
    hex_ident: String,
    generated_timestamp: DateTime<Utc>,
    logged_timestamp: DateTime<Utc>,
    callsign: String,
    altitude: i64,
    ground_speed: i64,
    track: i64,
    latitude: f64,
    longitude: f64,
    vertical_rate: i64,
    squawk: String,
}

#[axum::debug_handler]
async fn planes_handler(
    State(store): State<Arc<MessageStore>>,
    Json(query): Json<Query>,
) -> Result<Json<Vec<Plane>>> {
    let messages = store.get_messages(message_store::Query {
        message_type: Some("MSG".to_string()),
        transmission_type: None,
        start_timestamp: query.start.map(|t| t.timestamp_millis()),
        end_timestamp: query.end.map(|t| t.timestamp_millis()),
    })?;

    let mut current: HashMap<String, Plane> = HashMap::new();
    for msg in messages {
        let plane = current
            .entry(msg.hex_ident.clone())
            .or_insert_with(|| Plane::default());

        plane.aircraft_id = msg.aircraft_id;
        plane.hex_ident = msg.hex_ident;
        plane.generated_timestamp = to_date_time(msg.generated_timestamp)?;
        plane.logged_timestamp = to_date_time(msg.logged_timestamp)?;

        msg.callsign.map(|x| plane.callsign = x);
        msg.altitude.map(|x| plane.altitude = x);
        msg.ground_speed.map(|x| plane.ground_speed = x);
        msg.track.map(|x| plane.track = x);
        msg.latitude.map(|x| plane.latitude = x);
        msg.longitude.map(|x| plane.longitude = x);
        msg.vertical_rate.map(|x| plane.vertical_rate = x);
        msg.squawk.map(|x| plane.squawk = x);
    }

    Ok(Json(current.into_values().collect()))
}

fn to_date_time(timestamp: i64) -> Result<DateTime<Utc>> {
    Utc.timestamp_millis_opt(timestamp)
        .earliest()
        .ok_or(Error::InvalidArgument(format!(
            "Invalid timestamp: {timestamp}"
        )))
}

fn handle_line(store: &mut MessageStore, line: Result<String>) -> Result<()> {
    let line = line?;
    let msg: Msg = line.try_into()?;

    store.insert_message(&msg)?;

    println!("{msg:?}");
    Ok(())
}
