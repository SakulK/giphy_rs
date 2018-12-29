extern crate reqwest;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

#[derive(Serialize, Deserialize, Debug)]
pub struct GiphyGif {
    id: String,
    embed_url: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GiphyData {
    data: GiphyGif
}

#[derive(Debug)]
pub enum GiphyError {
    HttpError(reqwest::Error),
    ParsingError(serde_json::Error),
}

impl From<reqwest::Error> for GiphyError {
    fn from(e: reqwest::Error) -> Self {
        GiphyError::HttpError(e)
    }
}

impl From<serde_json::Error> for GiphyError {
    fn from(e: serde_json::Error) -> Self {
        GiphyError::ParsingError(e)
    }
}

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