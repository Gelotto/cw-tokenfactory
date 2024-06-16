use crate::{
    error::ContractError,
    msg::ConfigResponse,
    state::{models::Config, storage::MANAGER},
};

use super::ReadonlyContext;

pub fn query_config(ctx: ReadonlyContext) -> Result<ConfigResponse, ContractError> {
    let ReadonlyContext { deps, .. } = ctx;
    Ok(ConfigResponse(Config {
        manager: MANAGER.load(deps.storage)?,
    }))
}
