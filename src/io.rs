use crate::error::{AppError, AppResult};

pub fn question_bool(question: &str, default: bool) -> AppResult<bool> {
  let guide = if default {
    "Y/n"
  } else {
    "y/N"
  };
  println!("{} ({})", question, guide);
  let mut input = String::new();
  std::io::stdin().read_line(&mut input).map_err(|_| AppError::InputError)?;
  let sanitized = input.trim().to_lowercase(); 
  return match sanitized.as_str() {
    "" => Ok(default),
    "y" => Ok(true),
    "yes" => Ok(true),
    "n" => Ok(false),
    "no" => Ok(false),
    _ => Err(AppError::UnexpecetedInput)
  } 
}