use crate::{db::Pool, errors::GenericResult};
use chrono::{DateTime, Utc};
use serde::Serialize;
use tokio_postgres::Row;

#[derive(Debug, Clone, Serialize)]
pub(crate) struct Nfts {
    pub contract_address: String,
    pub contract_name: String,
    pub contract_symbol: String,
    pub name: String,
    pub description: String,
    pub image: String,
    pub external_url: String,
    pub chain: String,
    pub metadata_url: String,
    pub file_url: String,
    pub cached_file_url: String,
    pub mint_date: DateTime<Utc>,
    pub updated_date: DateTime<Utc>,
    pub token_id: i32,
}

impl Nfts {
    pub async fn get_by_contract_address(pool: &Pool, contract_address: &str) -> GenericResult<Vec<Self>> {
        let db = pool.get().await.map_err(|e| e.to_string())?;

        let query = String::from(
            "SELECT \
                contract_address, contract_name, contract_symbol, name, description, \
                image, external_url, chain, metadata_url, file_url, cached_file_url, \
                mint_date, updated_date, token_id \
            FROM v_nfts \
            WHERE contract_address = $1"
        );

        let rows = db.query(&query, &[&contract_address])
            .await
            .map(|op: Vec<Row>| {
                op.iter()
                    .map(|row| Self {
                        contract_address: row.get("contract_address"),
                        contract_name: row.get("contract_name"),
                        contract_symbol: row.get("contract_symbol"),
                        name: row.get("name"),
                        description: row.get("description"),
                        image: row.get("image"),
                        external_url: row.get("external_url"),
                        chain: row.get("chain"),
                        metadata_url: row.get("metadata_url"),
                        file_url: row.get("file_url"),
                        cached_file_url: row.get("cached_file_url"),
                        mint_date: row.get("mint_date"),
                        updated_date: row.get("updated_date"),
                        token_id: row.get("token_id"),
                    })
                    .collect::<Vec<Self>>()
            })
            .map_err(|e| e.to_string())?;

        Ok(rows)
    }

    pub async fn get_by_name(pool: &Pool, name: &str) -> GenericResult<Vec<Self>> {
        let db = pool.get().await.map_err(|e| e.to_string())?;

        let query = String::from(
            "SELECT \
                contract_address, contract_name, contract_symbol, name, description, \
                image, external_url, chain, metadata_url, file_url, cached_file_url, \
                mint_date, updated_date, token_id \
            FROM v_nfts \
            WHERE name = $1"
        );

        let rows = db.query(&query, &[&name])
            .await
            .map(|op: Vec<Row>| {
                op.iter()
                    .map(|row| Self {
                        contract_address: row.get("contract_address"),
                        contract_name: row.get("contract_name"),
                        contract_symbol: row.get("contract_symbol"),
                        name: row.get("name"),
                        description: row.get("description"),
                        image: row.get("image"),
                        external_url: row.get("external_url"),
                        chain: row.get("chain"),
                        metadata_url: row.get("metadata_url"),
                        file_url: row.get("file_url"),
                        cached_file_url: row.get("cached_file_url"),
                        mint_date: row.get("mint_date"),
                        updated_date: row.get("updated_date"),
                        token_id: row.get("token_id"),
                    })
                    .collect::<Vec<Self>>()
            })
            .map_err(|e| e.to_string())?;

        Ok(rows)
    }    

    pub async fn get_by_contract_address_and_token_id(pool: &Pool, contract_address: &str, token_id: i32) -> GenericResult<Vec<Self>> {
        let db = pool.get().await.map_err(|e| e.to_string())?;

        let query = String::from(
            "SELECT \
                contract_address, contract_name, contract_symbol, name, description, \
                image, external_url, chain, metadata_url, file_url, cached_file_url, \
                mint_date, updated_date, token_id \
            FROM v_nfts \
            WHERE contract_address = $1 AND token_id = $2"
        );

        let rows = db.query(&query, &[&contract_address, &token_id])
            .await
            .map(|op: Vec<Row>| {
                op.iter()
                    .map(|row| Self {
                        contract_address: row.get("contract_address"),
                        contract_name: row.get("contract_name"),
                        contract_symbol: row.get("contract_symbol"),
                        name: row.get("name"),
                        description: row.get("description"),
                        image: row.get("image"),
                        external_url: row.get("external_url"),
                        chain: row.get("chain"),
                        metadata_url: row.get("metadata_url"),
                        file_url: row.get("file_url"),
                        cached_file_url: row.get("cached_file_url"),
                        mint_date: row.get("mint_date"),
                        updated_date: row.get("updated_date"),
                        token_id: row.get("token_id"),
                    })
                    .collect::<Vec<Self>>()
            })
            .map_err(|e| e.to_string())?;

        Ok(rows)
    }      
}
