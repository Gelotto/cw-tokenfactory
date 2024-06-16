use crate::{
    error::ContractError,
    msg::{ContractStats, InfoResponse},
    state::storage::{AMOUNT_BURNED, AMOUNT_MINTED, DENOM_METADATA, FULL_DENOM},
};

use super::ReadonlyContext;

pub fn query_info(ctx: ReadonlyContext) -> Result<InfoResponse, ContractError> {
    let ReadonlyContext { deps, .. } = ctx;
    let denom = FULL_DENOM.load(deps.storage)?;
    Ok(InfoResponse {
        metadata: DENOM_METADATA.load(deps.storage)?.to_denom_metadata(&denom),
        stats: ContractStats {
            amount_burned: AMOUNT_BURNED.load(deps.storage)?,
            amount_minted: AMOUNT_MINTED.load(deps.storage)?,
        },
        denom,
    })
}
