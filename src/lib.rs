pub mod error;

use error::{AppError, AppResult, DynResult};
use serde::Deserialize;
use std::fs;
use std::process::Command;
use toml;

#[derive(Debug, Default)]
pub struct Config {
  filename: String,
}

const KNOWN_FILES: [VersionFile; 2] = [
  VersionFile {
    name: "Cargo.toml",
    extension: SupportedExtensions::Toml,
  },
  VersionFile {
    name: "package.json",
    extension: SupportedExtensions::Json,
  },
];

pub fn run(config: Config) -> AppResult<()> {
  check_git().map_err(|_| AppError::GitError);
  let version = KNOWN_FILES
    .iter()
    .map(|f| read_vesion_from_file(f))
    .find(|r| r.is_ok())
    .map(|r| r.unwrap())
    .ok_or_else(|| AppError::CannotFindVersion)?;
  println!("{}", &version);
  Ok(())
}

#[derive(Deserialize, Debug)]
struct FileField {
  version: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SupportedExtensions {
  Toml,
  Json,
}

struct VersionFile {
  name: &'static str,
  extension: SupportedExtensions,
}

fn read_vesion_from_file(f: &VersionFile) -> DynResult<String> {
  let file = fs::read_to_string(f.name)?;
  let parsed: FileField = match f.extension {
    SupportedExtensions::Toml => toml::from_str(&file)?,
    SupportedExtensions::Json => todo!(),
  };
  Ok("".to_string())
}

fn check_git() -> DynResult<()> {
  let child = Command::new("git --version").spawn()?;
  let out = child.wait_with_output()?;
  let text: String = String::from_utf8_lossy(&out.stdout).into_owned();
  println!("{}", &text);
  Ok(())
}
