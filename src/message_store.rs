use std::time::Instant;
use anyhow::Context;
use chrono::{Duration, Utc};
use diesel::{Connection, ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper, SqliteConnection};
use crate::models::Msg;
use crate::schema::messages::dsl;

pub struct MessageStore {
    conn: SqliteConnection,
}

impl MessageStore {
    pub fn new(conn: SqliteConnection) -> Self {
        Self { conn }
    }

    pub fn insert_message(&mut self, msg: &Msg) -> anyhow::Result<()> {
        diesel::insert_into(dsl::messages)
            .values(msg)
            .execute(&mut self.conn)?;
        Ok(())
    }

    pub fn get_messages(&mut self, start: Option<i64>, end: Option<i64>) -> anyhow::Result<Vec<Msg>> {
        let mut query = dsl::messages.select(Msg::as_select()).into_boxed();

        if let Some(start) = start {
            query = query.filter(dsl::generated_timestamp.ge(start));
        }

        if let Some(end) = end {
            query = query.filter(dsl::generated_timestamp.le(end));
        }

        query.load(&mut self.conn).context("Failed to execute query")
    }
}