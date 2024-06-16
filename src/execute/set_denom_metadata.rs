use crate::{
    error::ContractError,
    state::storage::{DENOM_METADATA, FACTORY},
    tf::cosmos::common::Metadata,
};
use cosmwasm_std::{attr, Response};

use super::Context;

pub fn exec_set_denom_metadata(
    ctx: Context,
    metadata: Metadata,
) -> Result<Response, ContractError> {
    let Context { deps, env, .. } = ctx;
    let factory = FACTORY.load(deps.storage)?;
    DENOM_METADATA.save(deps.storage, &metadata)?;
    Ok(Response::new()
        .add_attributes(vec![attr("action", "set_denom_metadata")])
        .add_message(factory.set_denom_metadata(env.contract.address.to_owned(), metadata)))
}
