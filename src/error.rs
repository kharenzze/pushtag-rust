use std::fmt::{Display};
use std::error::{Error};

#[derive(Debug, Clone, Copy)]
pub enum AppError {
  ExtensionNotFound,
  UnhandleExtension,
}

impl Display for AppError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let msg = match self {
      &AppError::ExtensionNotFound => "Extension not found",
      &AppError::UnhandleExtension => "Unhandled extension",
    };
    write!(f, "{}", msg)
  }
}

impl Error for AppError {}