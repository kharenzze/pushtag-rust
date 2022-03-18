use serde::Deserialize;
use std::convert::{TryFrom, TryInto};
use std::fs;
use std::process::Command;
use toml;

pub mod error;

use error::{AppError, DynResult};

#[derive(Debug, Default)]
pub struct Config {
  filename: String,
}

pub fn run(config: Config) -> DynResult<()> {
  check_git()?;
  read_vesion_from_file(&config.filename)?;
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

impl TryFrom<&str> for SupportedExtensions {
  type Error = AppError;

  fn try_from(value: &str) -> Result<Self, Self::Error> {
    let res: Self = match value {
      "toml" => SupportedExtensions::Toml,
      "json" => SupportedExtensions::Json,
      _ => return Err(AppError::UnhandleExtension),
    };
    Ok(res)
  }
}

pub fn read_vesion_from_file(filename: &str) -> DynResult<String> {
  let extension: SupportedExtensions = filename
    .split(".")
    .last()
    .ok_or_else(|| AppError::ExtensionNotFound)?
    .try_into()?;
  let file = fs::read_to_string(filename)?;
  let parsed: FileField = match extension {
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
