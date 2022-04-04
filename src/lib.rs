pub mod error;
mod io;

use clap::Parser;
use error::{AppError, AppResult, DynResult};
use git2::Repository;
use serde_json;
use std::fs;
use toml;

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Config {
  #[clap(long, short)]
  prefix: Option<String>,
  remote: Option<String>,
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

#[inline]
fn try_find_out_version() -> AppResult<String> {
  KNOWN_FILES
    .iter()
    .map(read_version_from_file)
    .find(|r| r.is_ok())
    .map(|r| r.unwrap())
    .ok_or_else(|| AppError::CannotFindVersion)
}

pub fn run(config: Config) -> AppResult<()> {
  let repo = check_git()?;
  let version = try_find_out_version()?;
  let pre = config.prefix.unwrap_or_else(|| "v".to_string());
  let tag = format!("{}{}", &pre, &version);
  let proceed = crate::io::question_bool(&format!("Tag: {}. Do you want to proceed?", &tag), true)?;
  if !proceed {
    return Err(AppError::AbortedByUser);
  }
  let remote_name: String = config.remote.unwrap_or("origin".into());
  let ctx = Context {
    tag: &tag,
    repo: &repo,
    remote_name: &remote_name
  };
  let already_exist = ctx.check_tag()?;
  if already_exist {
    let proceed = crate::io::question_bool("Tag already set. Do you want to move it?", false)?;
    if !proceed {
      return Err(AppError::AbortedByUser);
    }
    todo!();
  } else {
    ctx.set_tag()?;
    ctx.push_tag()?;
  }
  Ok(())
}

struct Context<'a> {
  tag: &'a str,
  repo: &'a Repository,
  remote_name: &'a str,
}

impl<'a> Context<'a>  {

  #[inline]
  fn check_tag(&self) -> AppResult<bool> {
    let list = self.repo.tag_names(Some(self.tag))?;
    let exist = list
      .iter()
      .filter(|s| s.is_some())
      .map(|o| o.unwrap())
      .find(|&s| s == self.tag);
    Ok(exist.is_some())
  }

  #[inline]
  fn move_tag(&self) -> AppResult<bool> {
    todo!();
  }

  #[inline]
  fn push_tag(&self) -> AppResult<()> {
    let mut remote = self.repo.find_remote(self.remote_name)?;
    remote.push(&[self.tag], None)?;
    Ok(())
  }

  #[inline]
  fn set_tag(&self) -> AppResult<()> {
    let obj = self.repo.revparse_single("HEAD")?;
    let sig = self.repo.signature()?;
    self.repo.tag(self.tag, &obj, &sig, "", false)?;
    Ok(())
  }
}

struct VersionFile {
  name: &'static str,
  version_getter: fn(&str) -> Option<String>,
}

fn read_version_from_file(f: &VersionFile) -> DynResult<String> {
  let file = fs::read_to_string(f.name)?;
  let version = (f.version_getter)(&file).ok_or(AppError::CannotFindVersion.into());
  if version.is_ok() {
    let v: &str = version.as_ref().unwrap();
    println!("Found version {} in file {}", v, f.name);
  }
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
