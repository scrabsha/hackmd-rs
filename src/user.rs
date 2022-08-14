use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    context::Context,
    error::{Error, Result},
    team::Team,
};

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
        context
            .client
            .get(Context::make_url("me"))
            .header("Authorization", &context.bearer)
            .send()
            .await?
            .json()
            .await
            .map_err(Error::from)
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
