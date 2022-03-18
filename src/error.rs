use thiserror::Error;

pub type DynResult<T> = Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, Clone, Copy, Error, PartialEq, Eq)]
pub enum AppError {
  #[error("Extension not found")]
  ExtensionNotFound,
  #[error("Unhandled extension")]
  UnhandleExtension,
}