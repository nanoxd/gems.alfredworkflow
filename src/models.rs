#[derive(Serialize, Deserialize, Debug)]
pub struct Gem {
  name: String,
  version: String,
  info: String,
  homepage_uri: String,
}
