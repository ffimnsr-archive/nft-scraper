use hyper::header::HeaderValue;
use hyper::{Body, Request, Response};
use routerify::prelude::*;
use routerify::{Middleware, RequestInfo, Router};
use uuid::Uuid;

use super::handlers::{
    error_handler, handler_index, handler_nft, handler_populate, handler_not_found, handler_trace,
};
use crate::db::Pool;
use crate::errors::{ApiError, ApiResult, ServiceError, ServiceResult};

async fn add_request_id(req: Request<Body>) -> ApiResult<Request<Body>> {
    req.set_context(Uuid::new_v4());
    Ok(req)
}

async fn logger(
    res: Response<Body>,
    req_info: RequestInfo,
) -> ApiResult<Response<Body>> {
    let request_id = req_info
        .context::<Uuid>()
        .ok_or_else(|| ApiError::BadRequest("Unable to get request id".into()))?;
    log::info!(
        "[request-id:{}] {} {} {}",
        request_id,
        res.status().as_u16(),
        req_info.method(),
        req_info.uri().path()
    );
    Ok(res)
}

async fn set_request_id_header(
    mut res: Response<Body>,
    req_info: RequestInfo,
) -> ApiResult<Response<Body>> {
    let request_id = req_info
        .context::<Uuid>()
        .ok_or_else(|| ApiError::BadRequest("Unable to get request id".into()))
        .map(|c| c.to_string())?;
    let value = HeaderValue::from_str(request_id.as_str()).unwrap();
    res.headers_mut().append("x-request-id", value);
    Ok(res)
}

pub(crate) fn router(
    db: Pool,
) -> ServiceResult<Router<Body, ApiError>> {
    Router::<Body, ApiError>::builder()
        .data(db.clone())
        .middleware(Middleware::pre(add_request_id))
        .middleware(Middleware::post_with_info(set_request_id_header))
        .middleware(Middleware::post_with_info(logger))
        .get("/", handler_index)
        .get("/populate", handler_populate)
        .get("/nft", handler_nft)
        .get("/trace", handler_trace)
        .any(handler_not_found)
        .err_handler(error_handler)
        .build()
        .map_err(ServiceError::Router)
}
