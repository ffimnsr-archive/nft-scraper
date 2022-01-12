use chrono::{DateTime, Utc};
use postgres_types::ToSql;
use super::Parameter;

#[derive(Debug, ToSql)]
pub(crate) struct NftData {
    pub token_id: i32,
    pub nft_contract_address: String,
    pub chain: String,
    pub metadata_url: String,
    pub file_url: String,
    pub cached_file_url: String,
    pub mint_date: DateTime<Utc>,
    pub updated_date: DateTime<Utc>,
}

impl<'a> NftData {
    pub fn parameters(&'a self) -> Vec<Parameter<'a>> {
        let params: Vec<Parameter<'a>> = vec![
            &self.token_id,
            &self.nft_contract_address,
            &self.chain,
            &self.metadata_url,
            &self.file_url,
            &self.cached_file_url,
            &self.mint_date,
            &self.updated_date,
        ];

        params
    }
}
