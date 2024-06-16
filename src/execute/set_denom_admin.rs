use crate::{
    error::ContractError,
    state::storage::{FACTORY, FULL_DENOM},
};
use cosmwasm_std::{attr, Addr, Response};

use super::Context;

pub fn exec_set_denom_admin(
    ctx: Context,
    new_admin: Addr,
) -> Result<Response, ContractError> {
    let Context { deps, env, .. } = ctx;
    let factory = FACTORY.load(deps.storage)?;
    let denom = FULL_DENOM.load(deps.storage)?;

    Ok(Response::new()
        .add_attributes(vec![attr("action", "set_denom_admin")])
        .add_message(factory.change_admin(
            env.contract.address.to_owned(),
            &denom,
            deps.api.addr_validate(new_admin.as_str())?,
        )))
}
