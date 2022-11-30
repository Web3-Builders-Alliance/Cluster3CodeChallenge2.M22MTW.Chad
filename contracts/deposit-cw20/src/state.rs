use cw20::Expiration;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, BlockInfo, Coin, Uint128};
use cw_storage_plus::Map;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Cw20Deposits {
    pub count: i32,
    pub owner: String,
    pub contract: String,
    pub amount: Uint128,
    pub stake_time: Expiration,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Deposits {
    pub count: i32,
    pub owner: Addr,
    pub coins: Coin,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Cw721Deposits {
    pub owner: String,
    pub contract: String,
    pub token_id: String,
}

type Contract<'a> = &'a str;
type Owner<'a> = &'a str;
type TokenId<'a> = &'a str;
type Denom<'a> = &'a str;

pub const DEPOSITS: Map<(Owner, Denom), Deposits> = Map::new("deposits");
pub const CW20_DEPOSITS: Map<(Owner, Contract), Cw20Deposits> = Map::new("cw20deposits");
pub const CW721_DEPOSITS: Map<(Contract, Owner, TokenId), Cw721Deposits> =
    Map::new("cw721deposits");
