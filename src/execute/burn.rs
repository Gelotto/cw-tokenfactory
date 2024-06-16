use crate::{
    error::ContractError,
    state::storage::{FACTORY, FULL_DENOM},
};
use cosmwasm_std::{attr, Response, Uint128};

use super::Context;

pub fn exec_burn(
    ctx: Context,
    amount: Uint128,
) -> Result<Response, ContractError> {
    let Context { deps, env, info } = ctx;
    let factory = FACTORY.load(deps.storage)?;
    let denom = FULL_DENOM.load(deps.storage)?;

    Ok(Response::new()
        .add_attributes(vec![attr("action", "burn")])
        .add_message(factory.burn(env.contract.address.to_owned(), &denom, amount)))
}
