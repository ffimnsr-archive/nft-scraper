pub mod nft_contract;
pub mod nft_data;
pub mod nft_file_information;
pub mod nft_metadata;
pub mod nfts;

pub(crate) use nft_contract::*;
pub(crate) use nft_data::*;
pub(crate) use nft_file_information::*;
pub(crate) use nft_metadata::*;
pub(crate) use nfts::*;

use postgres_types::ToSql;

pub(crate) type Parameter<'a> = &'a (dyn ToSql + Sync);