use crate::{
    error::ContractError,
    msg::{ContractStats, InfoResponse},
    state::storage::{AMOUNT_BURNED, AMOUNT_MINTED, DENOM_METADATA, FULL_DENOM},
};

use super::ReadonlyContext;

pub fn query_info(ctx: ReadonlyContext) -> Result<InfoResponse, ContractError> {
    let ReadonlyContext { deps, .. } = ctx;
    Ok(InfoResponse {
        denom: FULL_DENOM.load(deps.storage)?,
        metadata: DENOM_METADATA.load(deps.storage)?,
        stats: ContractStats {
            amount_burned: AMOUNT_BURNED.load(deps.storage)?,
            amount_minted: AMOUNT_MINTED.load(deps.storage)?,
        },
    })
}
