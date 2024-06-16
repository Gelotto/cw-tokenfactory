use crate::{
    error::ContractError,
    msg::NewDenomMetadata,
    state::storage::{DENOM_METADATA, FACTORY, FULL_DENOM},
};
use cosmwasm_std::{attr, Response};

use super::Context;

pub fn exec_set_denom_metadata(
    ctx: Context,
    metadata: NewDenomMetadata,
) -> Result<Response, ContractError> {
    let Context { deps, env, .. } = ctx;
    let factory = FACTORY.load(deps.storage)?;
    let full_denom = FULL_DENOM.load(deps.storage)?;
    DENOM_METADATA.save(deps.storage, &metadata)?;
    Ok(Response::new()
        .add_attributes(vec![attr("action", "set_denom_metadata")])
        .add_message(factory.set_denom_metadata(
            env.contract.address.to_owned(),
            metadata.to_denom_metadata(&full_denom),
        )))
}
