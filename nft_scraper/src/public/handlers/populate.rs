use std::fs;

use crate::errors::{ApiError, ApiResult, GenericResult};
use crate::{HeaderValues, MimeValues};
use crate::models::*;
use crate::db::Pool;
use routerify::prelude::*;
use chrono::{DateTime, Utc};
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
        pub metadata: Option<Metadata>,
        pub metadata_url: Option<String>,
        pub file_url: Option<String>,
        pub cached_file_url: Option<String>,
        pub mint_date: Option<String>,
        pub file_information: Option<FileInformation>,
        pub updated_date: Option<String>,
    }

    #[derive(Debug, Default, Clone, Deserialize)]
    pub struct Metadata {
        pub description: Option<String>,
        pub background_color: Option<String>,
        pub external_url: Option<String>,
        pub image: Option<String>,
        pub name: Option<String>,
        pub animation_url: Option<String>,
        pub attributes: Option<serde_json::Value>,
    }

    #[derive(Debug, Default, Clone, Deserialize)]
    pub struct FileInformation {
        pub height: i32,
        pub width: i32,
        pub file_size: i32,
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

async fn populate_db(pool: &Pool, address: &str) -> GenericResult<()> {
    let api_key = dotenv::var("NFTPORT_API_KEY").unwrap();
    let page_size = 50i64;

    let url = format!("https://api.nftport.xyz/v0/nfts/{}?chain=ethereum", address);
    let resp_total_count = reqwest::Client::new()
        .get(url)
        .header("authorization", &api_key)
        .send()
        .await?;

    let total_count = resp_total_count.json::<ContractDefinition>()
        .await
        .unwrap()
        .total
        .unwrap();

    let total_page_number = total_count / page_size;

    for i in 1..total_page_number {
        let url = format!("https://api.nftport.xyz/v0/nfts/{}?chain=ethereum&include=all&page_number={}&page_size={}", address, i, page_size);
        let resp = reqwest::Client::new()
            .get(url)
            .header("authorization", &api_key)
            .send()
            .await?;
    
        let definition = resp.json::<ContractDefinition>()
            .await
            .unwrap();
    
        if definition.nfts.is_empty() {
            break;
        }

        
        if i == 1 {
            let nft_contract = NftContract {
                address: address.to_string(),
                name: definition.contract.name.unwrap_or_default(),
                symbol: definition.contract.symbol.unwrap_or_default(),
                type_field: definition.contract.type_field.unwrap_or_default(),
            };

            nft_contract.save(&pool).await.unwrap();
        }

        // TODO: need to batch transact and do rollback when failure
        for nft in definition.nfts {
            let nft_data = NftData {
                nft_contract_address: address.to_string(),
                chain: "ethereum".to_string(),
                metadata_url: nft.metadata_url.unwrap_or_default(),
                file_url: nft.file_url.unwrap_or_default(),
                cached_file_url: nft.cached_file_url.unwrap_or_default(),
                token_id: nft.token_id.parse::<i32>().unwrap_or_default(),
                mint_date: "2014-11-28T21:00:09+09:00".parse::<DateTime<Utc>>().unwrap(),
                updated_date: "2014-11-28T21:00:09+09:00".parse::<DateTime<Utc>>().unwrap(),
            };

            let nft_data_id = nft_data.save(&pool).await.unwrap();

            let file_info = nft.file_information.unwrap_or_default();
            let nft_file_info = NftFileInformation {
                nft_data_id,
                height: file_info.height,
                width: file_info.width,
                file_size: file_info.file_size,
            };

            nft_file_info.save(&pool).await.unwrap(); 
            
            let meta = nft.metadata.unwrap_or_default();
            let nft_metadata = NftMetadata {
                nft_data_id,
                description: meta.description.unwrap_or_default(),
                background_color: meta.background_color.unwrap_or_default(),
                external_url: meta.external_url.unwrap_or_default(),
                image: meta.image.unwrap_or_default(),
                name: meta.name.unwrap_or_default(),
                animation_url: meta.animation_url.unwrap_or_default(),
                attributes: meta.attributes.unwrap_or_default().to_string(),
            };

            nft_metadata.save(&pool).await.unwrap();  
        }

        if i == 1 {
            break;
        }
    }

    Ok(())
}

pub(crate) async fn handler_populate(req: Request<Body>) -> ApiResult<Response<Body>> {
    let pool = req
        .data::<Pool>()
        .ok_or_else(ApiError::fatal("Unable to get database pool connection"))?
        .clone();

    log::info!("TRACE {:?}", req);

    let data = fs::read_to_string("collections.json").expect("Unable to read file");
    let addresses: Vec<String> = serde_json::from_str(&data).expect("JSON was not well formatted");

    tokio::spawn(async move {
        for contract_address in addresses {
            log::info!("TRACE {}", contract_address);
    
            populate_db(&pool.clone(), &contract_address)
                .await
                .map_err(|e| ApiError::BadRequest(e.to_string()))
                .unwrap();
        }    
    });

    Response::builder()
        .status(StatusCode::OK)
        .header(HeaderValues::CONTENT_TYPE, MimeValues::JSON_MIME_TYPE)
        .body(Body::from("POPULATE_DB".to_string()))
        .map_err(ApiError::Http)
}
