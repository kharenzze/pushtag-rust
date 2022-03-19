pub mod error;

use error::{AppError, AppResult, DynResult};
use git2::Repository;
use serde_json;
use std::fs;
use toml;

#[derive(Debug, Default)]
pub struct Config {
  filename: String,
}

const KNOWN_FILES: [VersionFile; 2] = [
  VersionFile {
    name: "Cargo.toml",
    version_getter: |file_content| {
      let parsed: toml::Value = toml::from_str(file_content).ok()?;
      let version: String = parsed
        .as_table()?
        .get("package")?
        .as_table()?
        .get("version")?
        .as_str()?
        .into();
      Some(version)
    },
  },
  VersionFile {
    name: "package.json",
    version_getter: |file_content| {
      let parsed: serde_json::Value = serde_json::from_str(&file_content).ok()?;
      let version: String = parsed.as_object()?.get("version")?.as_str()?.into();
      Some(version)
    },
  },
];

pub fn run(config: Config) -> AppResult<()> {
  check_git()?;
  let version = KNOWN_FILES
    .iter()
    .map(read_vesion_from_file)
    .find(|r| r.is_ok())
    .map(|r| r.unwrap())
    .ok_or_else(|| AppError::CannotFindVersion)?;
  Ok(())
}

struct VersionFile {
  name: &'static str,
  version_getter: fn(&str) -> Option<String>,
}

fn read_vesion_from_file(f: &VersionFile) -> DynResult<String> {
  let file = fs::read_to_string(f.name)?;
  let version = (f.version_getter)(&file).ok_or(AppError::CannotFindVersion.into());
  version
}

fn check_git() -> AppResult<Repository> {
  let repo = Repository::open(".").map_err(|_| AppError::GitRepoNotFound);
  repo
}

#[cfg(test)]
mod tests {
  use super::KNOWN_FILES;
  use std::fs;

  #[test]
  fn readers() {
    for vf in KNOWN_FILES.iter() {
      let path = format!("./tests/resources/{}", vf.name);
      let file = fs::read_to_string(path).expect("Missing file");
      let version = (vf.version_getter)(&file).unwrap();
      assert_eq!(&version, "1.2.3");
    }
  }
}
