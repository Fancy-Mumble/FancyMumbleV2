#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Server {
    pub description: String,
    pub host: String,
    pub port: u16,
    pub username: String,
}
