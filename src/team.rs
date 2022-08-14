use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Team {
    id: Uuid,
    owner_id: Uuid,
    path: String,
    name: String,
    logo: String,
    description: String,
    // TODO: this should be an enum, but this is poorly documented :'(
    // https://hackmd.io/@hackmd-api/developer-portal/https%3A%2F%2Fhackmd.io%2F%40hackmd-api%2Fuser-notes-api#Create-a-note
    visibility: String,
    // TODO: we should use chrono here
    created_at: String,
}
