pub mod burn;
pub mod mint;
pub mod remove_denom_admin;
pub mod set_denom_admin;
pub mod set_denom_metadata;
pub mod set_manager;

use cosmwasm_std::{DepsMut, Env, MessageInfo};

pub struct Context<'a> {
    pub deps: DepsMut<'a>,
    pub env: Env,
    pub info: MessageInfo,
}
