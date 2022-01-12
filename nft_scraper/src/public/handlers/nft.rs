use std::collections::HashMap;

use crate::errors::{ApiError, ApiResult};
use crate::{HeaderValues, MimeValues};
use crate::db::Pool;
use crate::models::Nfts;
use routerify::prelude::*;
use hyper::{Body, Request, Response, StatusCode};

pub(crate) async fn handler_nft(req: Request<Body>) -> ApiResult<Response<Body>> {
    let pool = req
        .data::<Pool>()
        .ok_or_else(ApiError::fatal("Unable to get database pool connection"))?;

    let params: HashMap<String, String> = req
        .uri()
        .query()
        .map(|v| {
            url::form_urlencoded::parse(v.as_bytes())
                .into_owned()
                .collect()
        })
        .unwrap_or_else(HashMap::new);

    if params.is_empty() {
        return ApiError::bad_request_err::<_>("Url query parameters is empty");
    }

    log::info!("params {:?}", params);

    let name = params.get("name")
        .map(|c| c.to_owned());

    let contract_address = params.get("address")
        .map(|c| c.to_owned());

    let token_id = params.get("token_id")
        .map(|c| c.to_owned());

    if name.is_some() && contract_address.is_some() {
        return ApiError::bad_request_err::<_>("Name and contract address cannot be set at same time");
    }

    if contract_address.is_some() && token_id.is_some() {
        let contract_address = contract_address.unwrap();
        let token_id = token_id.unwrap().parse::<i32>().unwrap();
        let tmp = Nfts::get_by_contract_address_and_token_id(&pool, &contract_address, token_id).await.unwrap();

        Response::builder()
            .status(StatusCode::OK)
            .header(HeaderValues::CONTENT_TYPE, MimeValues::JSON_MIME_TYPE)
            .body(Body::from(serde_json::to_string(&tmp).unwrap()))
            .map_err(ApiError::Http)        
    } else if contract_address.is_some() {
        let contract_address = contract_address.unwrap();
        let tmp = Nfts::get_by_contract_address(&pool, &contract_address).await.unwrap();

        Response::builder()
            .status(StatusCode::OK)
            .header(HeaderValues::CONTENT_TYPE, MimeValues::JSON_MIME_TYPE)
            .body(Body::from(serde_json::to_string(&tmp).unwrap()))
            .map_err(ApiError::Http)        
    } else if name.is_some() {
        let name = name.unwrap();
        let tmp = Nfts::get_by_name(&pool, &name).await.unwrap();

        Response::builder()
            .status(StatusCode::OK)
            .header(HeaderValues::CONTENT_TYPE, MimeValues::JSON_MIME_TYPE)
            .body(Body::from(serde_json::to_string(&tmp).unwrap()))
            .map_err(ApiError::Http)   
    } else {
        ApiError::bad_request_err::<_>("Name and contract address cannot be set at same time")
    }
}
