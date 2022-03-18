use std::fs;
use std::process::Command;
use serde::{Deserialize};
use toml;

pub mod error;

use error::{AppError, DynResult};


pub fn run() -> DynResult<()> {
  check_git()?;
  Ok(())
}

#[derive(Deserialize, Debug)]
struct FileField {
  version: String
}

pub fn read_vesion_from_file(filename: &str) -> DynResult<String> {
  let extension = filename .split(".").last();
  if extension.is_none() {
    return Err(Box::new(AppError::ExtensionNotFound))
  }
  let extension = extension.unwrap();
  let file = fs::read_to_string(filename)?;
  let parsed: DynResult<FileField> = match extension {
    "toml" => toml::from_str(&file).map_err(|err| err.into()),
    _ => Err(AppError::UnhandleExtension.into())
  };
  let parsed: FileField = parsed?;
  Ok("".to_string())
}

fn check_git() -> DynResult<()> {
  let child = Command::new("git --version").spawn()?;
  let out = child.wait_with_output()?;
  let text: String = String::from_utf8_lossy(&out.stdout).into_owned();
  println!("{}", &text);
  Ok(())
}