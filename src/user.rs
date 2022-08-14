use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{context::Context, error::Result, team::Team};

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: Option<String>,
    pub user_path: String,
    pub photo: String,
    pub teams: Vec<Team>,
}

impl User {
    pub async fn me(context: &Context) -> Result<User> {
        context.get("me").await
    }
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SimplifiedUser {
    name: String,
    photo: String,
    biography: Option<String>,
    user_path: String,
}
