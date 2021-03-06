use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::Item;
use cw_storage_plus::Map;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub owner: Addr,
}

pub const STATE: Item<State> = Item::new("state");

pub const SCORE_BY_ADDRESS: Map<Addr, i32> = Map::new("score_by_address");

pub const SCORE_BY_ADDRESS_AND_TOKEN: Map<(Addr, Addr), i32> =
    Map::new("score_by_address_and_token");
