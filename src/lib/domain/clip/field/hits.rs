// Constructor creates a public `new` method which returns the structures inner value
use derive_more::Constructor; // returns the structure with the inner value
use serde::{Deserialize, Serialize};

#[derive(Clone, Constructor, Debug, Serialize, Deserialize)]
pub struct Hits(u64);

impl Hits {
    pub fn into_inner(self) -> u64 {
        self.0
    }
}
