pub mod model; // makes the model module available
pub mod query;

use derive_more::{Display, From};
use serde::{Deserialize, Serialize};
use sqlx::Sqlite;
use std::str::FromStr;
use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
pub enum DataError {
    #[error("database error: {0}")]
    Database(#[from] sqlx::Error), // encapsulates all sqlx error types
}

// create data types so that in case of database change, you would have to
// update everything database related in one place
pub type AppDatabase = Database<Sqlite>;

// pool of connections, sqlite will create
// multiple connections to the database and constantly reuse them
pub type DatabasePool = sqlx::sqlite::SqlitePool;

// allows to roll back if there are any issues
// if we have multiple requests to database and an error occurs during the requests
// the transaction will allow us to roll back.
pub type Transaction<'t> = sqlx::Transaction<'t, Sqlite>;

// row and result are for returning the quest results
pub type AppDatabaseRow = sqlx::sqlite::SqliteRow;
pub type AppQueryResult = sqlx::sqlite::SqliteQueryResult;

// request database to implement the sqlx database trait
// and encapsulates a pool
pub struct Database<D: sqlx::Database>(sqlx::Pool<D>);

impl Database<Sqlite> {
    pub async fn new(connection_string: &str) -> Self {
        let pool = sqlx::sqlite::SqlitePoolOptions::new()
            .connect(connection_string)
            .await;

        match pool {
            Ok(pool) => Self(pool),
            Err(e) => {
                eprintln!("{}\n", e);
                eprintln!("if the database has not been created, run: \n\tsqlx database setup\n");
                panic!("database error");
            }
        }
    }

    pub fn get_pool(&self) -> &DatabasePool {
        &self.0
    }
}

#[derive(Clone, Debug, From, Display, Deserialize, Serialize)]
pub struct DbId(Uuid);

impl DbId {
    pub fn new() -> DbId {
        Uuid::new_v4().into()
    }

    pub fn nil() -> DbId {
        Self(Uuid::nil()) // creates a uuid that is all zeros, user for invalid or non-existent
                          // uuid. The reason that the course used this is because the database Id
                          // is internal to the application. Once the web app serializes it and
                          // send it over the wire to the user, it will be anonimized, because
                          // the user should not be able to see it and/or try to change it using
                          // a request
    }
}

impl From<DbId> for String {
    fn from(value: DbId) -> Self {
        format!("{}", value.0)
    }
}

impl Default for DbId {
    fn default() -> Self {
        Self::new()
    }
}

impl FromStr for DbId {
    type Err = uuid::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(DbId(Uuid::parse_str(s)?))
    }
}

#[cfg(test)]
pub mod test {
    use crate::data::*;
    use tokio::runtime::Handle;

    // new_db is a helper function to create a database instance for tests
    pub fn new_db(handle: &Handle) -> AppDatabase {
        use sqlx::migrate::Migrator;
        use std::path::Path;

        handle.block_on(async move {
            // for databases other then sqlite - create a transaction or a checkpoint instead of a
            // database
            let db = AppDatabase::new(":memory:").await;
            let migrator = Migrator::new(Path::new("./migrations")).await.unwrap();
            let pool = db.get_pool();

            migrator.run(pool).await.unwrap();

            db
        })
    }
}
