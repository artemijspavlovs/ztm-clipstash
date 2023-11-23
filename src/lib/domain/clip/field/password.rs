use crate::domain::clip::ClipError;
use rocket::form::{self, FromFormField, ValueField};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

// PartialEq will provide access to the == operation
// PartialOrd is necessary to use the PartialEq
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, PartialOrd, Default)]
pub struct Password(Option<String>);

impl Password {
    // the generic annotation on the New function provides the ability to input
    // either a `String` or `Option`, the `Into` will convert it into the appropriate type
    pub fn new<T: Into<Option<String>>>(password: T) -> Result<Self, ClipError> {
        // .into will convert the password into the necessary type
        // if it will be an Option already it will pass that down,
        // if it will be a String, it will convert the value into <Option<String>>
        let password: Option<String> = password.into();

        match password {
            // error is not returned because there is no need
            // it can be used to define password complexity rules afterwards
            Some(pass) => {
                if !pass.trim().is_empty() {
                    Ok(Self(Some(pass)))
                } else {
                    Ok(Self(None))
                }
            }
            None => Ok(Self(None)),
        }
    }

    pub fn into_inner(self) -> Option<String> {
        self.0
    }

    // String value is private for the Password struct, hence the only way to access
    // it is to expose some sore of functionality in the Structs scope to read it
    pub fn has_password(&self) -> bool {
        self.0.is_some()
    }
}

// provides the default implementation for the struct value when it is created
// impl Default for Password {
//     fn default() -> Self {
//         Self(None)
//     }
// }

impl FromStr for Password {
    type Err = ClipError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s.to_string())
    }
}

#[rocket::async_trait]
impl<'r> FromFormField<'r> for Password {
    fn from_value(field: ValueField<'r>) -> form::Result<'r, Self> {
        Ok(Self::new(field.value.to_owned())
            .map_err(|e| form::Error::validation(format!("{}", e)))?)
    }
}
