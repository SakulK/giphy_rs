use error::GiphyError;
use data::*;

const RANDOM_GIF_URL: &str = "https://api.giphy.com/v1/gifs/random?api_key=";

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
        let builder = self.reqwest_client.get(&format!("{}{}", RANDOM_GIF_URL, self.api_key));
        let http_result = self.reqwest_client.execute(builder.build()?)?.text()?;
        let result: GiphyData = serde_json::from_str(&http_result)?;
        Ok(result)
    }
}