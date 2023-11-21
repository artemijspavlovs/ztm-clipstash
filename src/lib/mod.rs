pub mod data;
pub mod domain;
pub mod service;
pub mod web;

// these exports allow to use these types directly from the crate
// using crate::Time syntax
// ShortCode will be utilised by the server and domain modules
pub use data::DataError;
pub use domain::clip::field::ShortCode;
pub use domain::clip::Clip;
pub use domain::clip::ClipError;
pub use domain::time::Time;
