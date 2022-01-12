use postgres_types::ToSql;
use super::Parameter;

#[derive(Debug, ToSql)]
pub(crate) struct NftContract {
    pub address: String,
    pub name: String,
    pub symbol: String,
    pub nft_type: String,
}

impl<'a> NftContract {
    pub fn parameters(&'a self) -> Vec<Parameter<'a>> {
        let params: Vec<Parameter<'a>> = vec![
            &self.address,
            &self.name,
            &self.symbol,
            &self.nft_type,
        ];

        params
    }
}
