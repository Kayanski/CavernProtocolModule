use cosmwasm_std::{Uint128, CosmosMsg};
use abstract_sdk::Execution;
use abstract_sdk::features::AbstractResponse;
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, wasm_execute, Coin};

use crate::handlers::query::query_market_config;

use crate::contract::{App, AppResult};

use crate::msg::AppExecuteMsg;
use crate::state::CONFIG;

pub fn execute_handler(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    app: App,
    msg: AppExecuteMsg,
) -> AppResult {
    match msg {
        AppExecuteMsg::Deposit {amount} => execute_deposit(deps, info, app, amount),
        AppExecuteMsg::Withdraw {amount} => execute_deposit(deps, info, app, amount),
        AppExecuteMsg::UpdateConfig {} => update_config(deps, info, app),
    }
}

/// Update the configuration of the app
fn execute_deposit(deps: DepsMut, msg_info: MessageInfo, app: App, amount: Uint128) -> AppResult {
    // Only the admin should be able to call this, because this is a private account
    app.admin.assert_admin(deps.as_ref(), &msg_info.sender)?;

    let config = CONFIG.load(deps.storage)?;
    let market_config = query_market_config(deps.as_ref())?;
    // We call an action on the proxy directly
    let executor = app.executor(deps.as_ref());
    let deposit_msg: CosmosMsg = wasm_execute(
        config.market_contract,
        &moneymarket::market::ExecuteMsg::DepositStable {  },
        vec![Coin{
            denom: market_config.stable_denom,
            amount
        }]
    )?.into();
    let deposit_msg = executor.execute(vec![deposit_msg.into()]);

    Ok(app.tag_response(Response::default(), "update_config").add_messages(deposit_msg))
}

/// Update the configuration of the app
fn update_config(deps: DepsMut, msg_info: MessageInfo, app: App) -> AppResult {
    // Only the admin should be able to call this
    app.admin.assert_admin(deps.as_ref(), &msg_info.sender)?;
    let mut _config = CONFIG.load(deps.storage)?;

    Ok(app.tag_response(Response::default(), "update_config"))
}
