use thiserror::Error;

pub type DynResult<T> = Result<T, Box<dyn std::error::Error>>;
pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug, Clone, Copy, Error, PartialEq, Eq)]
pub enum AppError {
  #[error("Extension not found")]
  ExtensionNotFound,
  #[error("Unhandled extension")]
  UnhandleExtension,
  #[error("Git Repo not found")]
  GitRepoNotFound,
  #[error("Error reading file")]
  FileError,
  #[error("Could not find a version number in current folder")]
  CannotFindVersion,
  #[error("Error reading input")]
  InputError,
}