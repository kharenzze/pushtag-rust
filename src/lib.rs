pub mod error;

use error::{AppError, AppResult, DynResult};
use serde::Deserialize;
use std::fs;
use std::process::Command;
use std::slice::SliceIndex;
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
    version_getter: |file_content| todo!(),
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

struct VersionFile {
  name: &'static str,
  version_getter: fn(&str) -> Option<String>,
}

fn read_vesion_from_file(f: &VersionFile) -> DynResult<String> {
  let file = fs::read_to_string(f.name)?;
  let version = (f.version_getter)(&file).ok_or(AppError::CannotFindVersion.into());
  version
}

fn check_git() -> DynResult<()> {
  let child = Command::new("git --version").spawn()?;
  let out = child.wait_with_output()?;
  let text: String = String::from_utf8_lossy(&out.stdout).into_owned();
  println!("{}", &text);
  Ok(())
}
