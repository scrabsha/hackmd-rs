use reqwest::Client;

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

    fn make_bearer(token: &str) -> String {
        format!("Bearer {token}")
    }

    pub(crate) fn make_url(route: &str) -> String {
        format!("{HACKMD_API_BASE_URL}/{route}")
    }
}
