use error::GiphyError;
use data::*;

const RANDOM_GIF_URL: &str = "https://api.giphy.com/v1/gifs/random";

pub struct GiphyClient {
    reqwest_client: reqwest::Client,
    api_key: String
}

impl GiphyClient {
    pub fn new(api_key: String) -> Self {
        GiphyClient {
            reqwest_client: reqwest::Client::new(),
            api_key
        }
    }

    pub fn random_gif(&self) -> Result<GiphyData, GiphyError> {
        self.call_api(RANDOM_GIF_URL, vec!())
    }

    pub fn random_gif_for_tag(&self, tag: String) -> Result<GiphyData, GiphyError> {
        self.call_api(RANDOM_GIF_URL, vec!(("tag".to_string(), tag)))
    }

    fn call_api<T: serde::de::DeserializeOwned>(&self, url: &str, params: Vec<(String, String)>) -> Result<T, GiphyError> {
        let http_result = self.reqwest_client.get(url)
            .query(&[("api_key", &self.api_key)])
            .query(&params)
            .send()?
            .text()?;
        let result: T = serde_json::from_str(&http_result)?;
        Ok(result)
    }
}