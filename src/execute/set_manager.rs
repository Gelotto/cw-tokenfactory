use crate::{error::ContractError, state::storage::MANAGER};
use cosmwasm_std::{attr, Addr, Response};

use super::Context;

pub fn exec_set_manager(
    ctx: Context,
    new_manager: Addr,
) -> Result<Response, ContractError> {
    let Context { deps, .. } = ctx;
    MANAGER.save(deps.storage, &deps.api.addr_validate(new_manager.as_str())?)?;
    Ok(Response::new().add_attributes(vec![
        attr("action", "set_manager"),
        attr("new_manager", new_manager.to_string()),
    ]))
}
