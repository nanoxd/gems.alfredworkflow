#[derive(Serialize, Deserialize, Debug)]
pub struct Gem {
  name: String,
  version: String,
  info: String,
  homepage_uri: String,
}

impl Gem {
  pub fn title(&self) -> String {
    format!("{} ({})", self.name, self.version)
  }

  pub fn subtitle(&self) -> String {
    format!("{}", self.info)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn test_gem() -> Gem {
    Gem {
      name: "cocoapods".to_string(),
      version: "1.4.0".to_string(),
      info: "CocoaPods manages library dependencies for your Xcode project.\n\nYou specify the dependencies for your project in one easy text file".to_string(),
      homepage_uri: "https://github.com/CocoaPods/CocoaPods".to_string()
    }
  }

  #[test]
  fn test_title() {
    let gem = test_gem();
    assert_eq!(gem.title(), "cocoapods (1.4.0)")
  }

  #[test]
  fn test_subtitle() {
    let gem = test_gem();
    assert_eq!(gem.subtitle(), "CocoaPods manages library dependencies for your Xcode project.\n\nYou specify the dependencies for your project in one easy text file")
  }
}
