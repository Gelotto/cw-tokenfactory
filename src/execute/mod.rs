pub mod burn;
pub mod mint;
pub mod set_denom_admin;
pub mod set_denom_metadata;

use cosmwasm_std::{DepsMut, Env, MessageInfo};

pub struct Context<'a> {
    pub deps: DepsMut<'a>,
    pub env: Env,
    pub info: MessageInfo,
}
