use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Uint128, Uint256};

use crate::{
    state::models::Config,
    tf::{cosmos::common::Metadata, tokenfactory::TokenFactoryType},
};

#[cw_serde]
pub struct InstantiateMsg {
    pub manager: Option<Addr>,
    pub factory: Option<TokenFactoryType>,
    pub initial_balances: Option<Vec<MintParams>>,
    pub metadata: NewDenomMetadata,
}

#[cw_serde]
pub struct NewDenomMetadata {
    pub symbol: String,
    pub decimals: u32,
    pub name: String,
    pub description: Option<String>,
    pub uri: Option<String>,
}

#[cw_serde]
pub struct MintParams {
    pub address: Addr,
    pub amount: Uint128,
}

#[cw_serde]
pub enum ExecuteMsg {
    Mint { recipient: Addr, amount: Uint128 },
    Burn { amount: Uint128 },
    SetManager { address: Addr },
    SetDenomMetadata { metadata: Metadata },
    SetDenomAdmin { address: Addr },
    RemoveDenomAdmin {},
}

#[cw_serde]
pub enum QueryMsg {
    Config {},
    Info {},
}

#[cw_serde]
pub struct MigrateMsg {}

#[cw_serde]
pub struct ConfigResponse(pub Config);

#[cw_serde]
pub struct ContractStats {
    pub amount_burned: Uint256,
    pub amount_minted: Uint256,
}

#[cw_serde]
pub struct InfoResponse {
    pub denom: String,
    pub metadata: Metadata,
    pub stats: ContractStats,
}
