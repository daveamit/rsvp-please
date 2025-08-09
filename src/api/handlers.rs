use crate::models::RsvpStatus;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Json},
};

// This async function handles a GET request to the /rsvp endpoint.
// It returns a JSON response with a 200 OK status code.
pub async fn get_rsvp_status() -> impl IntoResponse {
    let status = RsvpStatus {
        status: "You have been invited! Please RSVP.".to_string(),
    };
    (StatusCode::OK, Json(status))
}
