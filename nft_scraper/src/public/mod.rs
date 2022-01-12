mod handlers;
mod router;

use crate::db::Pool;
use crate::errors::{ServiceError, ServiceResult};
use hyper::Server;
use routerify::RouterService;

pub(crate) async fn serve(db: Pool) -> ServiceResult<()> {
    let router = router::router(db)?;

    let service = RouterService::new(router).map_err(ServiceError::Router)?;

    let addr = "[::]:4444".parse().map_err(ServiceError::AddrParser)?;

    log::info!("NFT Scraper is now listening at {}", addr);
    Server::bind(&addr)
        .serve(service)
        .await
        .map_err(ServiceError::Hyper)
}
