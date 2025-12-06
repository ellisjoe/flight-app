use std::str::FromStr;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use anyhow::{Context, Result};

#[derive(Debug)]
#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::messages)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Msg {
    pub message_type: String,
    pub transmission_type: Option<String>,
    pub session_id: String,
    pub aircraft_id: String,
    pub hex_ident: String,
    pub flight_id: String,
    pub generated_timestamp: i64,
    pub logged_timestamp: i64,

    pub callsign: Option<String>,
    pub altitude: Option<i64>,
    pub ground_speed: Option<i64>,
    pub track: Option<i64>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub vertical_rate: Option<i64>,
    pub squawk: Option<String>,
    pub alert: Option<bool>,
    pub emergency: Option<bool>,
    pub spi: Option<bool>,
    pub is_on_ground: Option<bool>,
}

impl TryFrom<String> for Msg {
    type Error = anyhow::Error;

    fn try_from(line: String) -> Result<Self, Self::Error> {
        let values = line.split(",").collect::<Vec<&str>>();

        if values.len() != 22 {
            anyhow::bail!(
                "Message must have exactly 22 elements: {}",
                values.join(",")
            );
        }

        Ok(Self {
            message_type: values[0].into(),
            transmission_type: parse_option(values[1])?,
            session_id: values[2].into(),
            aircraft_id: values[3].into(),
            hex_ident: values[4].into(),
            flight_id: values[5].into(),
            generated_timestamp: to_epoch_millis(values[6], values[7])?,
            logged_timestamp: to_epoch_millis(values[8], values[9])?,
            callsign: parse_option(values[10])?,
            altitude: parse_option(values[11])?,
            ground_speed: parse_option(values[12])?,
            track: parse_option(values[13])?,
            latitude: parse_option(values[14])?,
            longitude: parse_option(values[15])?,
            vertical_rate: parse_option(values[16])?,
            squawk: parse_option(values[17])?,
            alert: or_none(values[18]).map(|x| x == "1"),
            emergency: or_none(values[19]).map(|x| x == "1"),
            spi: or_none(values[20]).map(|x| x == "1"),
            is_on_ground: or_none(values[21]).map(|x| x == "1"),
        })
    }
}

fn parse_option<T, E>(str: &str) -> Result<Option<T>, E>
where
    T: FromStr<Err = E>,
{
    or_none(str).map(|x| x.parse()).transpose()
}

fn or_none(str: &str) -> Option<&str> {
    Some(str).filter(|s| !s.is_empty())
}

fn to_epoch_millis(date: &str, time: &str) -> Result<i64> {
    let date = format!("{} {}", date, time);
    let date_time = NaiveDateTime::parse_from_str(date.as_str(), "%Y/%m/%d %H:%M:%S%.3f")
        .context("Failed to parse datetime")?
        .and_local_timezone(chrono::Local)
        .earliest()
        .ok_or_else(|| anyhow::anyhow!("Unable to parse date: {}", date))?;
    Ok(date_time.timestamp_millis())
}
