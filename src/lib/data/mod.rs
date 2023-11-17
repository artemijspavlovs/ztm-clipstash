use derive_more::{Display, From};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use uuid::Uuid;

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
