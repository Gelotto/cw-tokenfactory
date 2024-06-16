use crate::{
    error::ContractError,
    state::storage::{FACTORY, FULL_DENOM},
};
use cosmwasm_std::{attr, Addr, Response};

use super::Context;

pub fn exec_remove_denom_admin(ctx: Context) -> Result<Response, ContractError> {
    let Context { deps, env, .. } = ctx;
    let factory = FACTORY.load(deps.storage)?;
    let denom = FULL_DENOM.load(deps.storage)?;

    Ok(Response::new()
        .add_attributes(vec![attr("action", "remove_denom_admin")])
        .add_message(factory.change_admin(
            env.contract.address.to_owned(),
            &denom,
            // apparently, one removes admin by setting admin to "null address"
            Addr::unchecked("0x000000000000000000000000000000000000000000000000000000000000000000"),
        )))
}
