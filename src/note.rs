use serde::{Deserialize, Serialize};

use crate::{
    context::Context,
    error::{Error, Result},
    user::SimplifiedUser,
};

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Note {
    pub id: String,
    pub title: String,
    pub tags: Vec<String>,
    pub created_at: usize,
    // TODO: should be an enum, not enough documentation
    pub publish_type: String,
    pub published_at: Option<String>,
    pub permalink: Option<String>,
    pub short_id: String,
    pub content: String,
    pub last_changed_at: usize,
    pub last_changed_user: Option<SimplifiedUser>,
    pub user_path: String,
    pub team_path: Option<String>,
    pub read_permission: String,
    pub write_permission: String,
}

impl Note {
    pub async fn get(context: &Context, id: &str) -> Result<Note> {
        let path = format!("notes/{id}");

        context
            .client
            .get(Context::make_url(&path))
            .header("Authorization", &context.bearer)
            .send()
            .await?
            .json()
            .await
            .map_err(Error::from)
    }

    pub async fn update(context: &Context, id: &str, update: &NoteUpdate) -> Result<()> {
        update.patch(context, id).await
    }
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Notes {
    #[serde(flatten)]
    notes: Vec<SimplifiedNote>,
}

impl Notes {
    pub async fn all(context: &Context) -> Result<Notes> {
        context
            .client
            .get(Context::make_url("notes"))
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
pub struct SimplifiedNote {
    id: String,
    title: String,
    tags: Vec<String>,
    created_at: usize,
    // TODO: should be an enum, not enough documentation
    publish_type: String,
    published_at: Option<String>,
    permalink: Option<String>,
    short_id: String,
    last_changed_at: usize,
    last_changed_user: SimplifiedUser,
    user_path: String,
    team_path: Option<String>,
    read_permission: String,
    write_permission: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NoteUpdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_permission: Option<String>,
}

impl NoteUpdate {
    pub async fn patch(&self, context: &Context, id: &str) -> Result<()> {
        let path = format!("notes/{id}");

        context
            .client
            .patch(Context::make_url(&path))
            .header("Authorization", &context.bearer)
            .json(self)
            .send()
            .await?;

        Ok(())
    }
}
