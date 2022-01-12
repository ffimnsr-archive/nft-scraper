use crate::{db::Pool, errors::GenericResult};
use postgres_types::ToSql;
use super::Parameter;

#[derive(Debug, ToSql)]
pub(crate) struct NftFileInformation {
    pub nft_data_id: i32,
    pub height: i32,
    pub width: i32,
    pub file_size: i32,
}

impl<'a> NftFileInformation {
    fn parameters(&'a self) -> Vec<Parameter<'a>> {
        let params: Vec<Parameter<'a>> = vec![
            &self.nft_data_id,
            &self.height,
            &self.width,
            &self.file_size,
        ];

        params
    }

    pub async fn save(&self, pool: &Pool) -> GenericResult<()> {
        let db = pool.get().await.map_err(|e| e.to_string())?;

        let query = String::from(
            "INSERT INTO nft_file_information \
                (nft_data_id, height, width, file_size) \
            VALUES ($1, $2, $3, $4) \
            RETURNING *",
        );

        db.query_one(&query, &self.parameters())
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }
}
