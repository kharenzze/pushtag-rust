use pushtag::error::AppResult;
use pushtag::{run, Config};

fn main() -> AppResult<()> {
  let config = Config::parse();
  run(config)
}
