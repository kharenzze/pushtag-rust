use thiserror::Error;

#[derive(Debug, Clone, Copy, Error, PartialEq, Eq)]
pub enum AppError {
  #[error("Extension not found")]
  ExtensionNotFound,
  #[error("Unhandled extension")]
  UnhandleExtension,
}