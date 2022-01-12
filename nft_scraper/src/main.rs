//! This module is the main entrypoint for nft scraper api.

pub(crate) use crate::mime as MimeValues;
pub(crate) use hyper::header as HeaderValues;
use std::env;

use crate::errors::ServiceResult;

mod db;
mod errors;
mod mime;
mod models;
mod public;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[tokio::main]
async fn main() -> ServiceResult<()> {
    dotenv::dotenv().ok();

    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "nft_scraper=info,hyper=info");
    }

    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    log::info!("Booting up NFT Scraper v{}", VERSION);

    let db = db::get_db_pool()?;
    public::serve(db.clone()).await
}
