pub mod nft_contract;
pub mod nft_data;
pub mod nft_file_information;
pub mod nft_metadata;

pub use nft_contract::*;
pub use nft_data::*;
pub use nft_file_information::*;
pub use nft_metadata::*;

use postgres_types::ToSql;

pub(crate) type Parameter<'a> = &'a (dyn ToSql + Sync);