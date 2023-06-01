use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};

use crate::contract::{App, AppResult};
use crate::msg::AppInstantiateMsg;
use crate::state::{Config, CONFIG};

pub fn instantiate_handler(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _app: App,
    msg: AppInstantiateMsg,
) -> AppResult {
    let config: Config = Config {
        market_contract: deps.api.addr_validate(&msg.market_contract)?
    };

    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new())
}
