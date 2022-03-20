use crate::error::{DynResult, AppError};

pub fn question_bool(question: &str, default: bool) -> DynResult<bool> {
  let guide = if default {
    "Y/n"
  } else {
    "y/N"
  };
  println!("{} ({})", question, guide);
  let mut input = String::new();
  std::io::stdin().read_line(&mut input)?;
  let sanitized = input.trim().to_lowercase(); 
  return match sanitized.as_str() {
    "" => Ok(default),
    "y" => Ok(true),
    "yes" => Ok(true),
    "n" => Ok(false),
    "no" => Ok(false),
    _ => Err(AppError::InputError.into())
  } 
}