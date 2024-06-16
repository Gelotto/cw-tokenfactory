use crate::{
    error::ContractError,
    state::storage::{AMOUNT_BURNED, FACTORY, FULL_DENOM},
};
use cosmwasm_std::{attr, Response, StdError, Uint128};

use super::Context;

pub fn exec_burn(
    ctx: Context,
    amount: Uint128,
) -> Result<Response, ContractError> {
    let Context { deps, env, .. } = ctx;
    let factory = FACTORY.load(deps.storage)?;
    let denom = FULL_DENOM.load(deps.storage)?;

    AMOUNT_BURNED.update(deps.storage, |n| -> Result<_, ContractError> {
        Ok(n.checked_add(amount.into())
            .map_err(|e| ContractError::Std(StdError::overflow(e)))?)
    })?;

    Ok(Response::new()
        .add_attributes(vec![attr("action", "burn")])
        .add_message(factory.burn(env.contract.address.to_owned(), &denom, amount)))
}
