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
    pub fn parameters(&'a self) -> Vec<Parameter<'a>> {
        let params: Vec<Parameter<'a>> = vec![
            &self.nft_data_id,
            &self.height,
            &self.width,
            &self.file_size,
        ];

        params
    }
}
