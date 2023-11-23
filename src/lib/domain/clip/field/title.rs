use crate::domain::clip::ClipError;
use rocket::form::{self, FromFormField, ValueField};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Title(Option<String>);

impl Title {
    // the generic annotation on the New function provides the ability to input
    // either a `String` or `Option`, the `Into` will convert it into the appropriate type
    pub fn new<T: Into<Option<String>>>(title: T) -> Self {
        // .into will convert the title into the necessary type
        // if it will be an Option already it will pass that down,
        // if it will be a String, it will convert the value into <Option<String>>
        let title: Option<String> = title.into();

        match title {
            Some(title) => {
                if !title.trim().is_empty() {
                    Self(Some(title))
                } else {
                    Self(None)
                }
            }
            None => Self(None),
        }
    }

    pub fn into_inner(self) -> Option<String> {
        self.0
    }
}

impl Default for Title {
    fn default() -> Self {
        Self::new(None)
    }
}

impl FromStr for Title {
    type Err = ClipError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(s.to_string()))
    }
}

#[rocket::async_trait]
impl<'r> FromFormField<'r> for Title {
    fn from_value(field: ValueField<'r>) -> form::Result<'r, Self> {
        Ok(Self::new(field.value.to_owned()))
    }
}
