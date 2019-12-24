use r2d2::ManageConnection;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::Connection;
use std::borrow::Cow;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("generic error")]
    Generic,

    #[error("rusqlite error: {0}")]
    Rusqlite(#[from] rusqlite::Error),
}

pub struct LoadBalancer {
    managers: Vec<SqliteConnectionManager>,
    manager_id: usize,
}

impl LoadBalancer {
    pub fn new() -> Self {
        Self {
            managers: Vec::new(),
            manager_id: 0,
        }
    }

    pub fn add(mut self, m: SqliteConnectionManager) -> Self {
        self.managers.push(m);
        self
    }

    pub fn build(self) -> Result<Self, Error> {
        // TODO: Validation
        Ok(self)
    }
}

impl ManageConnection for LoadBalancer {
    type Connection = BalancedConnection;
    type Error = Error;

    fn connect(&self) -> Result<Self::Connection, Self::Error> {
        // Choose the first backend for now
        Ok(BalancedConnection(
            self.managers[self.manager_id].connect()?,
        ))
    }

    fn is_valid(&self, _conn: &mut Self::Connection) -> Result<(), Self::Error> {
        Ok(())
    }

    fn has_broken(&self, _conn: &mut Self::Connection) -> bool {
        false
    }
}

pub struct BalancedConnection(Connection);

impl BalancedConnection {
    pub fn execute<T>(&mut self, query: &str, params: &[T]) -> Result<(), Error> {
        Ok(())
    }
}

#[derive(Clone)]
pub enum SqlValue {}

pub trait ToSql {
    fn to_sql<'a>(&self) -> Result<Cow<'a, SqlValue>, Error>;
}
