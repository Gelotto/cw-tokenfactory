use cosmwasm_std::{Addr, Uint256, Uint64};
use cw_storage_plus::{Deque, Item, Map};

use crate::{
    msg::MintParams,
    tf::{cosmos::common::Metadata, tokenfactory::TokenFactoryType},
};

/// Manager can mint and perform admin tasks
pub const MANAGER: Item<Addr> = Item::new("manager");

/// Platform-specific bindings for the targeted tokenfactory implementation
pub const FACTORY: Item<TokenFactoryType> = Item::new("factory");

/// Full denom, like 'factory/{contractAddr}/{subdenom}
pub const FULL_DENOM: Item<String> = Item::new("full_denom");

/// Temp storage for facilitating the minting of tokens for satisfying initial balances
pub const INITIAL_BALANCES: Deque<MintParams> = Deque::new("initial_balances");

/// Mint reply ID counter
pub const MINT_REPLY_ID_COUNTER: Item<Uint64> = Item::new("mint_reply_id_counter");

/// Temp storage for use when minting
pub const MINT_PARAMS: Map<u64, MintParams> = Map::new("mint_params");

/// Denom metadata set through this contract
pub const DENOM_METADATA: Item<Metadata> = Item::new("denom_metadata");

/// Total amount of token minted through this contract
pub const AMOUNT_MINTED: Item<Uint256> = Item::new("amount_minted");

/// Total amount of token burned through this contract
pub const AMOUNT_BURNED: Item<Uint256> = Item::new("amount_burned");
