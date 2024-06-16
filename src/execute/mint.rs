use crate::{
    error::ContractError,
    msg::MintParams,
    state::storage::{AMOUNT_MINTED, FACTORY, FULL_DENOM, MINT_PARAMS, MINT_REPLY_ID_COUNTER},
};
use cosmwasm_std::{
    attr, Addr, BankMsg, Coin, DepsMut, Reply, Response, StdError, SubMsg, SubMsgResult, Uint128,
    Uint64,
};

use super::Context;

pub fn exec_mint(
    ctx: Context,
    recipient: Addr,
    amount: Uint128,
) -> Result<Response, ContractError> {
    let Context { deps, env, .. } = ctx;
    let denom = FULL_DENOM.load(deps.storage)?;
    let factory = FACTORY.load(deps.storage)?;

    let reply_id = MINT_REPLY_ID_COUNTER
        .update(deps.storage, |n| -> Result<_, ContractError> {
            Ok(n + Uint64::one())
        })?
        .u64()
        - 1;

    MINT_PARAMS.save(
        deps.storage,
        reply_id,
        &MintParams {
            address: recipient.to_owned(),
            amount,
        },
    )?;

    Ok(Response::new()
        .add_attributes(vec![attr("action", "mint")])
        .add_submessage(SubMsg::reply_always(
            factory.mint(env.contract.address.to_owned(), denom, amount),
            reply_id,
        )))
}

pub fn transfer_minted_coins(
    deps: DepsMut,
    reply: Reply,
) -> Result<Response, ContractError> {
    let mut send_msgs: Vec<SubMsg> = Vec::with_capacity(1);
    match reply.result {
        SubMsgResult::Ok(_) => {
            let denom = FULL_DENOM.load(deps.storage)?;
            let MintParams { amount, address } = MINT_PARAMS.load(deps.storage, reply.id)?;

            MINT_PARAMS.remove(deps.storage, reply.id);

            AMOUNT_MINTED.update(deps.storage, |n| -> Result<_, ContractError> {
                Ok(n.checked_add(amount.into())
                    .map_err(|e| ContractError::Std(StdError::overflow(e)))?)
            })?;

            send_msgs.push(SubMsg::new(BankMsg::Send {
                to_address: address.to_string(),
                amount: vec![Coin::new(amount.into(), denom.to_owned())],
            }))
        },
        SubMsgResult::Err(e) => {
            return Err(ContractError::Std(StdError::generic_err(e.to_string())))
        },
    }

    Ok(Response::new().add_submessages(send_msgs))
}
