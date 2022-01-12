use crate::{db::Pool, errors::GenericResult};
use postgres_types::ToSql;
use super::Parameter;

#[derive(Debug, ToSql)]
pub(crate) struct NftMetadata {
    pub nft_data_id: i32,
    pub description: String,
    pub background_color: String,
    pub external_url: String,
    pub image: String,
    pub name: String,
    pub animation_url: String,
    pub attributes: String,
}

impl<'a> NftMetadata {
    fn parameters(&'a self) -> Vec<Parameter<'a>> {
        let params: Vec<Parameter<'a>> = vec![
            &self.nft_data_id,
            &self.description,
            &self.background_color,
            &self.external_url,
            &self.image,
            &self.name,
            &self.animation_url,
            &self.attributes,
        ];

        params
    }

    pub async fn save(&self, pool: &Pool) -> GenericResult<()> {
        let db = pool.get().await.map_err(|e| e.to_string())?;

        let query = String::from(
            "INSERT INTO nft_metadata \
                (nft_data_id, description, background_color, external_url, image, name, animation_url, attributes) \
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8) \
            RETURNING *",
        );

        db.query_one(&query, &self.parameters())
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }
}
