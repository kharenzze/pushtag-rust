use thiserror::Error;

pub type DynResult<T> = Result<T, Box<dyn std::error::Error>>;
pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug, Clone, Error, PartialEq, Eq)]
pub enum AppError {
  #[error("Extension not found")]
  ExtensionNotFound,
  #[error("Unhandled extension")]
  UnhandleExtension,
  #[error("Git Repo not found")]
  GitRepoNotFound,
  #[error("Git Error. {0}")]
  GitError(String),
  #[error("Error reading file")]
  FileError,
  #[error("Could not find a version number in current folder")]
  CannotFindVersion,
  #[error("Error reading input")]
  InputError,
  #[error("Unexpeceted input")]
  UnexpecetedInput,
  #[error("Aborted by user")]
  AbortedByUser,
}

impl From<git2::Error> for AppError {
  fn from(e: git2::Error) -> Self {
    AppError::GitError(e.to_string())
  }
}
