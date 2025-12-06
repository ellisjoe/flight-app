mod models;
mod schema;

use std::io::{BufRead, BufReader};
use std::net::TcpStream;
use diesel::{Connection, RunQueryDsl, SqliteConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use crate::models::Msg;
use crate::schema::messages::dsl;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

type Result<T> = anyhow::Result<T>;

fn main() -> Result<()> {
    let socket = TcpStream::connect("pi-zero-1:30003")?;
    let reader = BufReader::new(socket);

    let mut conn = SqliteConnection::establish("adsb.db")?;
    conn.run_pending_migrations(MIGRATIONS)
        .map_err(|e| anyhow::anyhow!("Failed to run database migrations: {}", e))?;

    for line in reader.lines() {
        handle_line(&mut conn, line.map_err(|e| e.into())).unwrap_or_else(|e| println!("Error: {}", e));
    }
    Ok(())
}

fn handle_line(conn: &mut SqliteConnection, line: Result<String>) -> Result<()> {
    let line = line?;
    let msg: Msg = line.try_into()?;

    diesel::insert_into(dsl::messages)
        .values(&msg)
        .execute(conn)?;

    println!("{msg:?}");
    Ok(())
}

// #[derive(Debug)]
// struct Msg {
//     message_type: String,
//     transmission_type: String,
//     session_id: String,
//     aircraft_id: String,
//     hex_ident: String,
//     flight_id: String,
//     generated: i64,
//     logged: i64,
//
//     callsign: Option<String>,
//     altitude: Option<i64>,
//     ground_speed: Option<i64>,
//     track: Option<i64>,
//     latitude: Option<f64>,
//     longitude: Option<f64>,
//     vertical_rate: Option<i64>,
//     squawk: Option<String>,
//     alert: Option<bool>,
//     emergency: Option<bool>,
//     spi: Option<bool>,
//     is_on_ground: Option<bool>,
// }
//
// impl TryFrom<String> for Msg {
//     type Error = Box<dyn Error>;
//
//     fn try_from(line: String) -> Result<Self, Self::Error> {
//         let values = line.split(",").collect::<Vec<&str>>();
//
//         if values.len() != 22 {
//             return Err(format!(
//                 "Message must have exactly 22 elements: {}",
//                 values.join(",")
//             )
//             .into());
//         }
//
//         Ok(Self {
//             message_type: values[0].into(),
//             transmission_type: values[1].into(),
//             session_id: values[2].into(),
//             aircraft_id: values[3].into(),
//             hex_ident: values[4].into(),
//             flight_id: values[5].into(),
//             generated: to_epoch_millis(values[6], values[7])?,
//             logged: to_epoch_millis(values[8], values[9])?,
//             callsign: parse_option(values[10])?,
//             altitude: parse_option(values[11])?,
//             ground_speed: parse_option(values[12])?,
//             track: parse_option(values[13])?,
//             latitude: parse_option(values[14])?,
//             longitude: parse_option(values[15])?,
//             vertical_rate: parse_option(values[16])?,
//             squawk: parse_option(values[17])?,
//             alert: or_none(values[18]).map(|x| x == "1"),
//             emergency: or_none(values[19]).map(|x| x == "1"),
//             spi: or_none(values[20]).map(|x| x == "1"),
//             is_on_ground: or_none(values[21]).map(|x| x == "1"),
//         })
//     }
// }
//
// fn parse_option<T, E>(str: &str) -> Result<Option<T>, E>
// where
//     T: FromStr<Err = E>,
// {
//     or_none(str).map(|x| x.parse()).transpose()
// }
//
// fn or_none(str: &str) -> Option<&str> {
//     Some(str).filter(|s| !s.is_empty())
// }
//
// fn to_epoch_millis(date: &str, time: &str) -> Result<i64, Box<dyn Error>> {
//     let date = format!("{} {}", date, time);
//     let date_time = NaiveDateTime::parse_from_str(date.as_str(), "%Y/%m/%d %H:%M:%S%.3f")?
//         .and_local_timezone(chrono::Local)
//         .earliest()
//         .ok_or_else(|| format!("Unable to parse date: {}", date))?;
//     Ok(date_time.timestamp_millis())
// }
