use reqwest::Client;
use serde::{de::DeserializeOwned, Serialize};

use crate::error::{Error, Result};

const HACKMD_API_BASE_URL: &str = "https://api.hackmd.io/v1";

pub struct Context {
    pub(crate) bearer: String,
    pub(crate) client: Client,
}

impl Context {
    pub fn new(token: &str) -> Context {
        let bearer = Self::make_bearer(token);
        let client = reqwest::Client::new();

        Context { bearer, client }
    }

    pub(crate) async fn get<T>(&self, path: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.client
            .get(Context::make_url(path))
            .header("Authorization", &self.bearer)
            .send()
            .await?
            .json()
            .await
            .map_err(Error::from)
    }

    pub(crate) async fn patch<T>(&self, path: &str, payload: &T) -> Result<()>
    where
        T: Serialize,
    {
        self.client
            .patch(Context::make_url(path))
            .header("Authorization", &self.bearer)
            .json(payload)
            .send()
            .await
            .map(drop)
            .map_err(Error::from)
    }

    fn make_bearer(token: &str) -> String {
        format!("Bearer {token}")
    }

    pub(crate) fn make_url(route: &str) -> String {
        format!("{HACKMD_API_BASE_URL}/{route}")
    }
}
