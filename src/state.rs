use cosmwasm_std::Addr;
use cw_storage_plus::Item;

#[cosmwasm_schema::cw_serde]
pub struct Config {
    pub market_contract: Addr,
}

pub const CONFIG: Item<Config> = Item::new("config");
