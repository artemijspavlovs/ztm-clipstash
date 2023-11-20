use crate::data::DbId;
use crate::{ClipError, ShortCode, Time};
use chrono::{NaiveDateTime, Utc};
use std::convert::TryFrom;
use std::str::FromStr;

// FromRow will convert a sqlite row into a `Clip` automatically
// we need to specify fields and data types that are going to
// correspond with the database fields so the FromRow can perform the
// automatic conversion
#[derive(Debug, sqlx::FromRow)]
pub struct Clip {
    // difference between model::Clip and domain::Clip are that the
    // domain::Clip has fields with verification. This type will be used
    // when working inside the program.
    //
    // whereas the
    // model::Clip will have the database specific information. This type will
    // be used when working with the database.Since this type will have only
    // default types as the field values (String etc.), it might have invalid data.
    // because of that, this type should **only** be used when working with the
    // information from the database
    //
    // while the fields will be the same, the data types will differ.
    // This approach is introduced to have a level of abstraction between
    // our database and the program.
    pub(in crate::data) clip_id: String,
    pub(in crate::data) shortcode: String,
    pub(in crate::data) content: String,
    pub(in crate::data) title: Option<String>,
    pub(in crate::data) posted: NaiveDateTime,
    pub(in crate::data) expires: Option<NaiveDateTime>,
    pub(in crate::data) password: Option<String>,
    pub(in crate::data) hits: i64,
    // (in crate::data) make it so these fields are only accessible from within the
    // data module and only it can modify data in order to get data to and from the
    // database
}

impl TryFrom<Clip> for crate::domain::Clip {
    type Error = ClipError;
    fn try_from(value: Clip) -> Result<Self, Self::Error> {
        // these are necessary to transform core types into field types
        use crate::domain::clip::field;

        Ok(Self {
            clip_id: field::ClipId::new(DbId::from_str(value.clip_id.as_str())?),
            shortcode: field::ShortCode::from(value.shortcode),
            content: field::Content::new(value.content.as_str())?,
            title: field::Title::new(value.title),
            posted: field::Posted::new(Time::from_naive_utc(value.posted)),
            expires: field::Expires::new(value.expires.map(Time::from_naive_utc)),
            password: field::Password::new(value.password.unwrap_or_default())?,
            hits: field::Hits::new(u64::try_from(value.hits)?),
        })
    }
}

// create a model for the query vefore writing it
pub struct GetClip {
    pub(in crate::data) shortcode: String,
}

impl From<crate::service::ask::GetClip> for GetClip {
    fn from(value: crate::service::ask::GetClip) -> Self {
        Self {
            shortcode: value.shortcode.into_inner(),
        }
    }
}

impl From<ShortCode> for GetClip {
    fn from(value: ShortCode) -> Self {
        GetClip {
            shortcode: value.into_inner(),
        }
    }
}

impl From<String> for GetClip {
    fn from(value: String) -> Self {
        GetClip { shortcode: value }
    }
}

pub struct NewClip {
    pub(in crate::data) clip_id: String,
    pub(in crate::data) shortcode: String,
    pub(in crate::data) content: String,
    pub(in crate::data) title: Option<String>,
    pub(in crate::data) posted: i64,
    pub(in crate::data) expires: Option<i64>,
    pub(in crate::data) password: Option<String>,
}

impl From<crate::service::ask::NewClip> for NewClip {
    fn from(value: crate::service::ask::NewClip) -> Self {
        Self {
            clip_id: DbId::new().into(),
            shortcode: ShortCode::default().into(),
            content: value.content.into_inner(),
            title: value.title.into_inner(),
            posted: Utc::now().timestamp(),
            expires: value.expires.into_inner().map(|time| time.timestamp()),
            password: value.password.into_inner(),
        }
    }
}

pub struct UpdateClip {
    pub(in crate::data) shortcode: String,
    pub(in crate::data) content: String,
    pub(in crate::data) title: Option<String>,
    pub(in crate::data) expires: Option<i64>,
    pub(in crate::data) password: Option<String>,
}
