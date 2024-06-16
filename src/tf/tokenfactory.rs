use std::{fmt::Display, str::FromStr};

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, CosmosMsg, Uint128};

use crate::tf::{cosmos, injective, kujira, osmosis};

use super::{cosmos::common::Metadata, juno};

#[cw_serde]
pub enum TokenFactoryType {
    CosmWasm = 1,
    Kujira = 2,
    Injective = 3,
    Osmosis = 4,
    Juno = 5,
}
impl Display for TokenFactoryType {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        let str = match &self {
            Self::CosmWasm => String::from("CosmWasm"),
            Self::Kujira => String::from("Kujira"),
            Self::Injective => String::from("Injective"),
            Self::Osmosis => String::from("Osmosis"),
            Self::Juno => String::from("Juno"),
        };
        write!(f, "{}", str)
    }
}
impl FromStr for TokenFactoryType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "CosmWasm" => Ok(Self::CosmWasm),
            "Kujira" => Ok(Self::Kujira),
            "Injective" => Ok(Self::Injective),
            "Osmosis" => Ok(Self::Osmosis),
            "Juno" => Ok(Self::Juno),
            _ => Err(()),
        }
    }
}
impl TokenFactoryType {
    pub fn from_chain_id(chain_id: &String) -> Self {
        match chain_id.as_str() {
            "juno-1" | "testing" => Self::Juno,
            "osmosis-1" => Self::Osmosis,
            "injective-1" => Self::Injective,
            "kujira-1" => Self::Kujira,
            _ => Self::CosmWasm,
        }
    }

    pub fn burn(
        &self,
        address: Addr,
        denom: &str,
        amount: Uint128,
    ) -> CosmosMsg {
        match self {
            TokenFactoryType::CosmWasm => {
                <cosmos::denom::MsgBurn as Into<CosmosMsg>>::into(cosmos::denom::MsgBurn {
                    sender: address.to_string(),
                    amount: Some(cosmos::denom::Coin {
                        denom: denom.to_string(),
                        amount: amount.to_string(),
                    }),
                })
            },
            TokenFactoryType::Kujira => {
                <kujira::denom::MsgBurn as Into<CosmosMsg>>::into(kujira::denom::MsgBurn {
                    sender: address.to_string(),
                    amount: Some(kujira::denom::Coin {
                        denom: denom.to_string(),
                        amount: amount.to_string(),
                    }),
                })
            },
            TokenFactoryType::Injective => {
                <injective::denom::MsgBurn as Into<CosmosMsg>>::into(injective::denom::MsgBurn {
                    sender: address.to_string(),
                    amount: Some(injective::denom::Coin {
                        denom: denom.to_string(),
                        amount: amount.to_string(),
                    }),
                })
            },
            TokenFactoryType::Osmosis => {
                <osmosis::denom::MsgBurn as Into<CosmosMsg>>::into(osmosis::denom::MsgBurn {
                    sender: address.to_string(),
                    amount: Some(osmosis::denom::Coin {
                        denom: denom.to_string(),
                        amount: amount.to_string(),
                    }),
                    burn_from_address: address.to_string(),
                })
            },
            TokenFactoryType::Juno => {
                <juno::denom::MsgBurn as Into<CosmosMsg>>::into(juno::denom::MsgBurn {
                    sender: address.to_string(),
                    amount: Some(juno::denom::Coin {
                        denom: denom.to_string(),
                        amount: amount.to_string(),
                    }),
                })
            },
        }
    }

    pub fn mint(
        &self,
        sender: Addr,
        denom: String,
        amount: Uint128,
    ) -> CosmosMsg {
        match self {
            TokenFactoryType::CosmWasm => {
                <cosmos::denom::MsgMint as Into<CosmosMsg>>::into(cosmos::denom::MsgMint {
                    sender: sender.to_string(),
                    amount: Some(cosmos::denom::Coin {
                        denom: denom.to_string(),
                        amount: amount.to_string(),
                    }),
                })
            },
            TokenFactoryType::Kujira => {
                <kujira::denom::MsgMint as Into<CosmosMsg>>::into(kujira::denom::MsgMint {
                    sender: sender.to_string(),
                    amount: Some(kujira::denom::Coin {
                        denom: denom.to_string(),
                        amount: amount.to_string(),
                    }),
                    recipient: sender.to_string(),
                })
            },
            TokenFactoryType::Injective => {
                <injective::denom::MsgMint as Into<CosmosMsg>>::into(injective::denom::MsgMint {
                    sender: sender.to_string(),
                    amount: Some(injective::denom::Coin {
                        denom: denom.to_string(),
                        amount: amount.to_string(),
                    }),
                    mint_to_address: sender.to_string(),
                })
            },
            TokenFactoryType::Osmosis => {
                <osmosis::denom::MsgMint as Into<CosmosMsg>>::into(osmosis::denom::MsgMint {
                    sender: sender.to_string(),
                    amount: Some(osmosis::denom::Coin {
                        denom: denom.to_string(),
                        amount: amount.to_string(),
                    }),
                })
            },
            TokenFactoryType::Juno => {
                <juno::denom::MsgMint as Into<CosmosMsg>>::into(juno::denom::MsgMint {
                    sender: sender.to_string(),
                    amount: Some(juno::denom::Coin {
                        denom: denom.to_string(),
                        amount: amount.to_string(),
                    }),
                })
            },
        }
    }

    pub fn change_admin(
        &self,
        sender: Addr,
        denom: &str,
        new_admin: Addr,
    ) -> CosmosMsg {
        match self {
            TokenFactoryType::CosmWasm => <cosmos::denom::MsgChangeAdmin as Into<CosmosMsg>>::into(
                cosmos::denom::MsgChangeAdmin {
                    sender: sender.to_string(),
                    denom: denom.to_string(),
                    new_admin: new_admin.to_string(),
                },
            ),
            TokenFactoryType::Kujira => <kujira::denom::MsgChangeAdmin as Into<CosmosMsg>>::into(
                kujira::denom::MsgChangeAdmin {
                    sender: sender.to_string(),
                    denom: denom.to_string(),
                    new_admin: new_admin.to_string(),
                },
            ),
            TokenFactoryType::Injective => {
                <injective::denom::MsgChangeAdmin as Into<CosmosMsg>>::into(
                    injective::denom::MsgChangeAdmin {
                        sender: sender.to_string(),
                        denom: denom.to_string(),
                        new_admin: new_admin.to_string(),
                    },
                )
            },
            TokenFactoryType::Osmosis => <osmosis::denom::MsgChangeAdmin as Into<CosmosMsg>>::into(
                osmosis::denom::MsgChangeAdmin {
                    sender: sender.to_string(),
                    denom: denom.to_string(),
                    new_admin: new_admin.to_string(),
                },
            ),
            TokenFactoryType::Juno => <juno::denom::MsgChangeAdmin as Into<CosmosMsg>>::into(
                juno::denom::MsgChangeAdmin {
                    sender: sender.to_string(),
                    denom: denom.to_string(),
                    new_admin: new_admin.to_string(),
                },
            ),
        }
    }

    pub fn create_denom(
        &self,
        address: Addr,
        subdenom: &str,
    ) -> CosmosMsg {
        match self {
            TokenFactoryType::CosmWasm => <cosmos::denom::MsgCreateDenom as Into<CosmosMsg>>::into(
                cosmos::denom::MsgCreateDenom {
                    sender: address.to_string(),
                    subdenom: subdenom.to_string(),
                },
            ),
            TokenFactoryType::Kujira => <kujira::denom::MsgCreateDenom as Into<CosmosMsg>>::into(
                kujira::denom::MsgCreateDenom {
                    sender: address.to_string(),
                    subdenom: subdenom.to_string(),
                },
            ),
            TokenFactoryType::Injective => {
                <injective::denom::MsgCreateDenom as Into<CosmosMsg>>::into(
                    injective::denom::MsgCreateDenom {
                        sender: address.to_string(),
                        subdenom: subdenom.to_string(),
                    },
                )
            },
            TokenFactoryType::Osmosis => <osmosis::denom::MsgCreateDenom as Into<CosmosMsg>>::into(
                osmosis::denom::MsgCreateDenom {
                    sender: address.to_string(),
                    subdenom: subdenom.to_string(),
                },
            ),
            TokenFactoryType::Juno => <juno::denom::MsgCreateDenom as Into<CosmosMsg>>::into(
                juno::denom::MsgCreateDenom {
                    sender: address.to_string(),
                    subdenom: subdenom.to_string(),
                },
            ),
        }
    }

    pub fn set_denom_metadata(
        &self,
        address: Addr,
        metadata: Metadata,
    ) -> CosmosMsg {
        match self {
            TokenFactoryType::CosmWasm => {
                <cosmos::denom::MsgSetDenomMetadata as Into<CosmosMsg>>::into(
                    cosmos::denom::MsgSetDenomMetadata {
                        sender: address.to_string(),
                        metadata: Some(metadata),
                    },
                )
            },
            TokenFactoryType::Kujira => {
                <kujira::denom::MsgSetDenomMetadata as Into<CosmosMsg>>::into(
                    kujira::denom::MsgSetDenomMetadata {
                        sender: address.to_string(),
                        metadata: Some(metadata),
                    },
                )
            },
            TokenFactoryType::Injective => {
                <injective::denom::MsgSetDenomMetadata as Into<CosmosMsg>>::into(
                    injective::denom::MsgSetDenomMetadata {
                        sender: address.to_string(),
                        metadata: Some(metadata),
                    },
                )
            },
            TokenFactoryType::Osmosis => {
                <osmosis::denom::MsgSetDenomMetadata as Into<CosmosMsg>>::into(
                    osmosis::denom::MsgSetDenomMetadata {
                        sender: address.to_string(),
                        metadata: Some(metadata),
                    },
                )
            },
            TokenFactoryType::Juno => <juno::denom::MsgSetDenomMetadata as Into<CosmosMsg>>::into(
                juno::denom::MsgSetDenomMetadata {
                    sender: address.to_string(),
                    metadata: Some(metadata),
                },
            ),
        }
    }

    pub fn admin_path(&self) -> String {
        match self {
            Self::CosmWasm => "/cosmwasm.tokenfactory.v1.Query/DenomInfo",
            Self::Kujira => "/kujira.tokenfactory.v1.Query/DenomInfo",
            Self::Injective => "/injective.tokenfactory.v1.Query/DenomInfo",
            Self::Osmosis => "/osmosis.tokenfactory.v1.Query/DenomInfo",
            Self::Juno => "/osmosis.tokenfactory.v1.Query/DenomInfo",
        }
        .to_string()
    }
}

#[cw_serde]
pub struct QueryDenomAuthorityMetadataRequest {
    pub denom: String,
}

#[cw_serde]
pub struct QueryDenomAuthorityMetadataResponse {
    pub admin: String,
}
