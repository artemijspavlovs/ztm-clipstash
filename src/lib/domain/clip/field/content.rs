use crate::domain::clip::ClipError;
use rocket::form::{self, FromFormField, ValueField};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]

// The String property of Content type is private
// it would have a pub keyword in front of it if it would be public
// we can access the Component from anywhere
// we can access the String only from the impl block for the struct
pub struct Content(String);

// The reason we create a new type for every field in the structure
// is because Rust will not create the struct unless the data is valid.
// since the String is private, the only way to create content is through the
impl Content {
    // `new` method
    pub fn new(content: &str) -> Result<Self, ClipError> {
        if !content.trim().is_empty() {
            Ok(Self(content.to_owned()))
        } else {
            Err(ClipError::EmptyContent)
        }
    }

    // into_inner is commonly used in the Rust ecosystem to transform the content
    // of a struct into the returnable type
    // will remove the instance of the Content after execution
    pub fn into_inner(self) -> String {
        self.0
    }

    // will *not remove* the instance of the Content after execution because reference
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[rocket::async_trait]
impl<'r> FromFormField<'r> for Content {
    fn from_value(field: ValueField<'r>) -> form::Result<'r, Self> {
        Ok(Self::new(field.value).map_err(|e| form::Error::validation(format!("{}", e)))?)
    }
}
