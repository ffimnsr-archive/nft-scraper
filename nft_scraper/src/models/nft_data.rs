use crate::{db::Pool, errors::GenericResult};
use chrono::{DateTime, Utc};
use postgres_types::ToSql;
use super::Parameter;

#[derive(Debug, ToSql)]
pub(crate) struct NftData {
    pub nft_contract_address: String,
    pub chain: String,
    pub metadata_url: String,
    pub file_url: String,
    pub cached_file_url: String,
    pub token_id: i32,
    pub mint_date: DateTime<Utc>,
    pub updated_date: DateTime<Utc>,
}

impl<'a> NftData {
    fn parameters(&'a self) -> Vec<Parameter<'a>> {
        let params: Vec<Parameter<'a>> = vec![
            &self.nft_contract_address,
            &self.chain,
            &self.metadata_url,
            &self.file_url,
            &self.cached_file_url,
            &self.token_id,
            &self.mint_date,
            &self.updated_date,
        ];

        params
    }

    pub async fn save(&self, pool: &Pool) -> GenericResult<i32> {
        let db = pool.get().await.map_err(|e| e.to_string())?;

        let query = String::from(
            "INSERT INTO nft_data \
                (nft_contract_address, chain, metadata_url, file_url, cached_file_url, token_id, mint_date, updated_date) \
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8) \
            RETURNING id",
        );

        let row = db.query_one(&query, &self.parameters())
            .await
            .map_err(|e| e.to_string())?;

        Ok(row.get(0))
    }    
}
