use postgres_types::ToSql;
use super::Parameter;

#[derive(Debug, ToSql)]
pub(crate) struct NftData {
    pub nft_data_id: i32,
    pub description: String,
    pub background_color: String,
    pub external_url: String,
    pub image: String,
    pub name: String,
    pub animation_url: String,
}

impl<'a> NftData {
    pub fn parameters(&'a self) -> Vec<Parameter<'a>> {
        let params: Vec<Parameter<'a>> = vec![
            &self.nft_data_id,
            &self.description,
            &self.background_color,
            &self.external_url,
            &self.image,
            &self.name,
            &self.animation_url,
        ];

        params
    }
}
