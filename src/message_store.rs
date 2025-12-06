use anyhow::Context;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper, SqliteConnection};
use crate::models::Msg;
use crate::schema::messages::dsl;

pub struct MessageStore {
    conn: SqliteConnection,
}

pub struct Query {
    pub message_type: Option<String>,
    pub transmission_type: Option<String>,
    pub start_timestamp: Option<i64>,
    pub end_timestamp: Option<i64>,
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

    pub fn get_messages(&mut self, query: Query) -> anyhow::Result<Vec<Msg>> {
        let mut sql = dsl::messages.select(Msg::as_select()).into_boxed();

        if let Some(message_type) = query.message_type {
            sql = sql.filter(dsl::message_type.eq(message_type));
        }

        if let Some(transmission_type) = query.transmission_type {
            sql = sql.filter(dsl::transmission_type.eq(transmission_type));
        }

        if let Some(start) = query.start_timestamp {
            sql = sql.filter(dsl::generated_timestamp.ge(start));
        }

        if let Some(end) = query.end_timestamp {
            sql = sql.filter(dsl::generated_timestamp.le(end));
        }

        sql.load(&mut self.conn).context("Failed to execute query")
    }
}