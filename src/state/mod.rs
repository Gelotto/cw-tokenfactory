pub mod models;
pub mod storage;

use cosmwasm_std::{
    BankMsg, Coin, DepsMut, Reply, Response, StdError, SubMsg, SubMsgResult, Uint128, Uint256,
    Uint64,
};
use storage::{
    AMOUNT_BURNED, AMOUNT_MINTED, DENOM_METADATA, FACTORY, FULL_DENOM, INITIAL_BALANCES, MANAGER,
    MINT_REPLY_ID_COUNTER,
};

use crate::{
    error::ContractError,
    execute::Context,
    msg::{InstantiateMsg, MintParams},
    tf::tokenfactory::TokenFactoryType,
};

pub const INITIAL_BALANCES_REPLY_ID: u64 = 0;
pub const INITIAL_MINT_REPLY_ID: u64 = 1_000_000u64;

/// Top-level initialization of contract state
pub fn init(
    ctx: Context,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let Context { env, deps, info } = ctx;
    let subdenom = msg.metadata.symbol.to_lowercase();
    let full_denom = format!("factory/{}/{}", env.contract.address, subdenom);
    let contract_addr = env.contract.address;

    let factory = msg
        .factory
        .unwrap_or_else(|| TokenFactoryType::from_chain_id(&env.block.chain_id));

    let denom_msgs: Vec<SubMsg> = vec![
        SubMsg::new(factory.create_denom(contract_addr.to_owned(), &subdenom)),
        SubMsg::new(factory.set_denom_metadata(
            contract_addr.to_owned(),
            msg.metadata.to_denom_metadata(&full_denom),
        )),
    ];

    MANAGER.save(
        deps.storage,
        &(if let Some(manager) = msg.manager {
            deps.api.addr_validate(manager.as_str())?
        } else {
            info.sender.to_owned()
        }),
    )?;

    let mut resp = Response::new()
        .add_attribute("action", "instantiate")
        .add_submessages(denom_msgs);

    if let Some(initial_balances) = msg.initial_balances {
        let mut total_initial_mint_amount = Uint128::zero();
        for x in initial_balances.iter() {
            deps.api.addr_validate(x.address.as_str())?;

            total_initial_mint_amount = total_initial_mint_amount
                .checked_add(x.amount.into())
                .map_err(|e| ContractError::Std(StdError::overflow(e)))?;

            INITIAL_BALANCES.push_back(deps.storage, x)?;
        }

        resp = resp.add_submessage(SubMsg::reply_always(
            factory.mint(
                contract_addr.to_owned(),
                full_denom.to_owned(),
                total_initial_mint_amount,
            ),
            INITIAL_BALANCES_REPLY_ID,
        ));
    }

    FULL_DENOM.save(deps.storage, &full_denom)?;
    FACTORY.save(deps.storage, &factory)?;
    MINT_REPLY_ID_COUNTER.save(deps.storage, &Uint64::from(INITIAL_MINT_REPLY_ID))?;
    AMOUNT_MINTED.save(deps.storage, &Uint256::zero())?;
    AMOUNT_BURNED.save(deps.storage, &Uint256::zero())?;
    DENOM_METADATA.save(deps.storage, &msg.metadata)?;

    Ok(resp)
}

pub fn transfer_initial_balances(
    deps: DepsMut,
    reply: Reply,
) -> Result<Response, ContractError> {
    let mut send_msgs: Vec<SubMsg> = Vec::with_capacity(4);
    match reply.result {
        SubMsgResult::Ok(_) => {
            let denom = FULL_DENOM.load(deps.storage)?;
            let n = INITIAL_BALANCES.len(deps.storage)?;
            let mut total_amount = Uint256::zero();

            for _ in 0..n {
                let MintParams { amount, address } =
                    INITIAL_BALANCES.pop_front(deps.storage)?.unwrap();

                total_amount = total_amount
                    .checked_add(amount.into())
                    .map_err(|e| ContractError::Std(StdError::overflow(e)))?;

                send_msgs.push(SubMsg::new(BankMsg::Send {
                    to_address: address.to_string(),
                    amount: vec![Coin::new(amount.into(), denom.to_owned())],
                }))
            }

            AMOUNT_MINTED.update(deps.storage, |n| -> Result<_, ContractError> {
                Ok(n.checked_add(total_amount)
                    .map_err(|e| ContractError::Std(StdError::overflow(e)))?)
            })?;
        },
        SubMsgResult::Err(e) => {
            return Err(ContractError::Std(StdError::generic_err(e.to_string())))
        },
    }

    Ok(Response::new().add_submessages(send_msgs))
}
