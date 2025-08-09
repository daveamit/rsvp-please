use serde::{Deserialize, Serialize};

// This struct defines the shape of our API response.
#[derive(Serialize, Deserialize)]
pub struct RsvpStatus {
    pub status: String,
}
