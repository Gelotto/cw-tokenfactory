use crate::{
    error::ContractError,
    state::storage::{FACTORY, FULL_DENOM},
};
use cosmwasm_std::{attr, Addr, CanonicalAddr, Response};

use super::Context;

pub fn exec_remove_denom_admin(ctx: Context) -> Result<Response, ContractError> {
    let Context { deps, env, .. } = ctx;
    let factory = FACTORY.load(deps.storage)?;
    let denom = FULL_DENOM.load(deps.storage)?;

    // apparently, one removes admin by setting admin to "null address"
    let empty_canonical_addr = CanonicalAddr::from(vec![]);
    let empty_addr = Addr::unchecked(deps.api.addr_humanize(&empty_canonical_addr)?);

    Ok(Response::new()
        .add_attributes(vec![attr("action", "remove_denom_admin")])
        .add_message(factory.change_admin(env.contract.address.to_owned(), &denom, empty_addr)))
}
