use std::fs;

use crate::errors::{ApiError, ApiResult, GenericResult};
use crate::{HeaderValues, MimeValues};
use hyper::{Body, Request, Response, StatusCode};

mod internal {
    use serde::Deserialize;

    #[derive(Debug, Clone, Deserialize)]
    pub struct ContractDefinition {
        pub response: String,
        pub nfts: Vec<Nft>,
        pub contract: Contract,
        pub total: Option<i64>,
    }

    #[derive(Debug, Clone, Deserialize)]
    pub struct Nft {
        pub chain: String,
        pub contract_address: String,
        pub token_id: String,
        pub metadata: Metadata,
        pub metadata_url: Option<String>,
        pub file_url: Option<String>,
        pub cached_file_url: Option<String>,
        pub mint_date: Option<String>,
        pub file_information: Option<FileInformation>,
        pub updated_date: Option<String>,
    }

    #[derive(Debug, Clone, Deserialize)]
    pub struct Metadata {
        pub description: Option<String>,
        pub background_color: Option<String>,
        pub external_url: Option<String>,
        pub image: Option<String>,
        pub name: Option<String>,
        pub animation_url: Option<String>,
        pub attributes: Option<serde_json::Value>,
    }

    #[derive(Debug, Clone, Deserialize)]
    pub struct FileInformation {
        pub height: i64,
        pub width: i64,
        pub file_size: i64,
    }

    #[derive(Debug, Clone, Deserialize)]
    pub struct Contract {
        pub name: Option<String>,
        pub symbol: Option<String>,
        #[serde(rename = "type")]
        pub type_field: Option<String>,
    }
}

pub use internal::ContractDefinition;

async fn get_contract_data(address: &str) -> GenericResult<ContractDefinition> {
    let url = format!("https://api.nftport.xyz/v0/nfts/{}?chain=ethereum&include=all&page_number=1", address);
    let api_key = dotenv::var("NFTPORT_API_KEY").unwrap();
    let resp = reqwest::Client::new()
        .get(url)
        .header("authorization", &api_key)
        .send()
        .await?;

    log::info!("TRACE: {:#?}", resp);

    resp.json::<ContractDefinition>()
        .await
        .map_err(|e| Box::from(e.to_string()))

}

pub(crate) async fn handler_populate(req: Request<Body>) -> ApiResult<Response<Body>> {
    log::info!("TRACE {:?}", req);

    let data = fs::read_to_string("collections.json").expect("Unable to read file");
    let addresses: Vec<String> = serde_json::from_str(&data).expect("JSON was not well formatted");

    for contract_address in addresses {
        log::info!("TRACE {}", contract_address);

        let contract_data = get_contract_data(&contract_address)
            .await
            .map_err(|e| ApiError::BadRequest(e.to_string()))?;
        
        log::info!("TRACE {}: {:?}", contract_address, contract_data);
    }

    Response::builder()
        .status(StatusCode::OK)
        .header(HeaderValues::CONTENT_TYPE, MimeValues::JSON_MIME_TYPE)
        .body(Body::from("TRACE: See the internal server logs.".to_string()))
        .map_err(ApiError::Http)
}
