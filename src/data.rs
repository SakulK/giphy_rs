#[derive(Serialize, Deserialize, Debug)]
pub struct GiphyGif {
    pub id: String,
    pub slug: String,
    pub image_url: String,
    pub username: String,
    pub source: String,
    pub title: String,
    pub caption: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GiphyData {
    pub data: GiphyGif
}