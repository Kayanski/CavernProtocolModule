use crate::contract::{App, AppResult};
use crate::msg::{AppQueryMsg, ConfigResponse};
use crate::state::CONFIG;
use cosmwasm_std::{to_binary, Binary, Deps, Env, StdResult, QueryRequest, WasmQuery};

pub fn query_handler(deps: Deps, _env: Env, _app: &App, msg: AppQueryMsg) -> AppResult<Binary> {
    match msg {
        AppQueryMsg::Config {} => to_binary(&query_config(deps)?),
        AppQueryMsg::MarketConfig {} => to_binary(&query_market_config(deps)?),
    }
    .map_err(Into::into)
}

fn query_config(deps: Deps) -> StdResult<ConfigResponse> {
    let _config = CONFIG.load(deps.storage)?;
    Ok(ConfigResponse {})
}

pub fn query_market_config(deps: Deps) -> StdResult<moneymarket::market::ConfigResponse>{
    let config = CONFIG.load(deps.storage)?;
    let config_response = deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart{
        contract_addr: config.market_contract.to_string(),
        msg: to_binary(&moneymarket::market::QueryMsg::Config {  })?
    }))?;

    Ok(config_response)
}