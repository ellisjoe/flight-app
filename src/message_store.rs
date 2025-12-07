use crate::errors::{Error, Result};
use crate::models::Msg;
use crate::schema::messages::dsl;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper, SqliteConnection};
use std::ops::DerefMut;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct MessageStore {
    conn: Arc<Mutex<SqliteConnection>>,
}

pub struct Query {
    pub message_type: Option<String>,
    pub transmission_type: Option<String>,
    pub start_timestamp: Option<i64>,
    pub end_timestamp: Option<i64>,
}

impl MessageStore {
    pub fn new(conn: Arc<Mutex<SqliteConnection>>) -> Self {
        Self { conn }
    }

    pub fn insert_message(&self, msg: &Msg) -> Result<()> {
        let mut conn = self
            .conn
            .lock()
            .map_err(|e| Error::InternalError(format!("Lock poisoned: {e}")))?;
        diesel::insert_into(dsl::messages)
            .values(msg)
            .execute(conn.deref_mut())
            .map_err(|e| Error::InternalError(format!("Inserting message failed: {e}")))?;
        Ok(())
    }

    pub fn get_messages(&self, query: Query) -> Result<Vec<Msg>> {
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

        let mut conn = self
            .conn
            .lock()
            .map_err(|e| Error::InternalError(format!("Lock poisoned: {e}")))?;
        sql.load(conn.deref_mut())
            .map_err(|e| Error::InternalError(format!("Failed to execute query: {e}")))
    }
}
