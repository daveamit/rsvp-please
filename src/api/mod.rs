// Declare the handlers module here.
pub mod handlers;

use axum::{Router, routing::get};
// Use the get_rsvp_status function from the handlers module.
use crate::api::handlers::get_rsvp_status;

pub fn create_router() -> Router {
    Router::new().route("/rsvp", get(get_rsvp_status))
}
