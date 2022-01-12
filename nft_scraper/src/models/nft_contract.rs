use crate::{db::Pool, errors::GenericResult};
use postgres_types::ToSql;
use super::Parameter;

#[derive(Debug, ToSql)]
pub(crate) struct NftContract {
    pub address: String,
    pub name: String,
    pub symbol: String,
    pub type_field: String,
}

impl<'a> NftContract {
    fn parameters(&'a self) -> Vec<Parameter<'a>> {
        let params: Vec<Parameter<'a>> = vec![
            &self.address,
            &self.name,
            &self.symbol,
            &self.type_field,
        ];

        params
    }

    pub async fn save(&self, pool: &Pool) -> GenericResult<()> {
        let db = pool.get().await.map_err(|e| e.to_string())?;

        let query = String::from(
            "INSERT INTO nft_contract \
                (address, name, symbol, type) \
            VALUES ($1, $2, $3, $4) \
            RETURNING address",
        );

        db.query_one(&query, &self.parameters())
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }
}
