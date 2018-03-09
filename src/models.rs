#[derive(Serialize, Deserialize, Debug)]
pub struct Gem {
  pub name: String,
  pub version: String,
  pub info: String,
  pub homepage_uri: String,
}
