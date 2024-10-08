mod nft;
mod populate;
mod trace;

use crate::errors::{ApiError, ApiResult, ServiceError};
use crate::{HeaderValues, MimeValues, VERSION};
use hyper::{Body, Method, Request, Response, StatusCode};

pub(crate) use nft::handler_nft;
pub(crate) use populate::handler_populate;
pub(crate) use trace::handler_trace;

pub(crate) async fn handler_index(_: Request<Body>) -> ApiResult<Response<Body>> {
    let data = serde_json::json!({
        "success": true,
        "message": format!("NFT Scraper v{}", VERSION),
    });

    Response::builder()
        .status(StatusCode::OK)
        .header(HeaderValues::CONTENT_TYPE, MimeValues::JSON_MIME_TYPE)
        .body(Body::from(data.to_string()))
        .map_err(ApiError::Http)
}

pub(crate) async fn handler_not_found(req: Request<Body>) -> ApiResult<Response<Body>> {
    match *req.method() {
        Method::OPTIONS => Response::builder()
            .status(StatusCode::NO_CONTENT)
            .header(HeaderValues::CONTENT_LENGTH, "0")
            .body(Body::empty())
            .map_err(ApiError::Http),

        _ => {
            let data = serde_json::json!({
                "success": false,
                "message": "Route not found",
            });

            Response::builder()
                .status(StatusCode::NOT_FOUND)
                .header(HeaderValues::CONTENT_TYPE, MimeValues::JSON_MIME_TYPE)
                .body(Body::from(data.to_string()))
                .map_err(ApiError::Http)
        }
    }
}

pub(crate) async fn error_handler(err: routerify::RouteError) -> Response<Body> {
    let svc_err = err.downcast::<ApiError>().unwrap();

    match svc_err.as_ref() {
        ApiError::Http(e) => {
            let data = serde_json::json!({
                "success": false,
                "message": e.to_string(),
            });

            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .header(HeaderValues::CONTENT_TYPE, MimeValues::JSON_MIME_TYPE)
                .body(Body::from(data.to_string()))
                .map_err(ServiceError::Http)
                .unwrap()
        }
        _ => {
            log::error!("Error trace {:?}", svc_err);

            let data = serde_json::json!({
                "success": false,
                "message": svc_err.to_string(),
            });

            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .header(HeaderValues::CONTENT_TYPE, MimeValues::JSON_MIME_TYPE)
                .body(Body::from(data.to_string()))
                .map_err(ServiceError::Http)
                .unwrap()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_handler_index_should_ok() {
        let req = Request::<Body>::default();
        let resp = handler_index(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_handler_404_should_ok() {
        let data = serde_json::json!({
            "success": false,
            "message": "Route not found",
        });

        let req = Request::<Body>::default();
        let resp = handler_not_found(req).await.unwrap();
        let (parts, body) = resp.into_parts();

        let body_raw = hyper::body::to_bytes(body).await.unwrap();
        let body = String::from_utf8(body_raw.to_vec()).unwrap();
        assert_eq!(parts.status, StatusCode::NOT_FOUND);
        assert_eq!(data.to_string(), body);
    }
}
