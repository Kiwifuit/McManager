use anyhow::Context;
use dotenv::dotenv;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  dotenv().context("while loading dotenv")?;

  dbg!(std::env::vars());

  Ok(())
}
