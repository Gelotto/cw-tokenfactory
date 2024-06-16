use crate::error::ContractError;
use crate::execute::burn::exec_burn;
use crate::execute::mint::{exec_mint, transfer_minted_coins};
use crate::execute::remove_denom_admin::exec_remove_denom_admin;
use crate::execute::set_denom_admin::exec_set_denom_admin;
use crate::execute::set_denom_metadata::exec_set_denom_metadata;
use crate::execute::set_manager::exec_set_manager;
use crate::execute::Context;
use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use crate::query::info::query_info;
use crate::query::{query_config, ReadonlyContext};
use crate::state::storage::MANAGER;
use crate::state::{
    self, transfer_initial_balances, INITIAL_BALANCES_REPLY_ID, INITIAL_MINT_REPLY_ID,
};
use cosmwasm_std::{ensure_eq, entry_point, to_json_binary, Reply};
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response};
use cw2::set_contract_version;

const CONTRACT_NAME: &str = "crates.io:cw-tokenfactory";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    Ok(state::init(Context { deps, env, info }, msg)?)
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    let ctx = Context { deps, env, info };

    ensure_eq!(
        ctx.info.sender,
        MANAGER.load(ctx.deps.storage)?,
        ContractError::NotAuthorized {
            reason: "only manager can perform this action".to_owned()
        }
    );

    match msg {
        ExecuteMsg::SetDenomMetadata { metadata } => exec_set_denom_metadata(ctx, metadata),
        ExecuteMsg::SetDenomAdmin { address } => exec_set_denom_admin(ctx, address),
        ExecuteMsg::RemoveDenomAdmin {} => exec_remove_denom_admin(ctx),
        ExecuteMsg::SetManager { address } => exec_set_manager(ctx, address),
        ExecuteMsg::Mint { recipient, amount } => exec_mint(ctx, recipient, amount),
        ExecuteMsg::Burn { amount } => exec_burn(ctx, amount),
    }
}

#[entry_point]
pub fn reply(
    deps: DepsMut,
    _env: Env,
    reply: Reply,
) -> Result<Response, ContractError> {
    if reply.id == INITIAL_BALANCES_REPLY_ID {
        // Finish creating the new market contract, storing its address
        transfer_initial_balances(deps, reply)
    } else if reply.id >= INITIAL_MINT_REPLY_ID {
        transfer_minted_coins(deps, reply)
    } else {
        Err(ContractError::NotAuthorized {
            reason: format!("unrecognized reply id {}", reply.id),
        })
    }
}

#[entry_point]
pub fn query(
    deps: Deps,
    env: Env,
    msg: QueryMsg,
) -> Result<Binary, ContractError> {
    let ctx = ReadonlyContext { deps, env };
    let result = match msg {
        QueryMsg::Config {} => to_json_binary(&query_config(ctx)?),
        QueryMsg::Info {} => to_json_binary(&query_info(ctx)?),
    }?;
    Ok(result)
}

#[entry_point]
pub fn migrate(
    deps: DepsMut,
    _env: Env,
    _msg: MigrateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    Ok(Response::default())
}
