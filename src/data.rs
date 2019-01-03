#[derive(Serialize, Deserialize, Debug)]
pub struct GiphyGif {
    pub id: String,
    pub embed_url: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GiphyData {
    pub data: GiphyGif
}