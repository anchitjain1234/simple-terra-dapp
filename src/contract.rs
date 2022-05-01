#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, OwnerResponse, ScoreResponse};
use crate::state::{State, STATE, SCORE_BY_ADDRESS};

use cosmwasm_std::Addr;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:simple-terra-dapp";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let state = State {
        owner: info.sender.clone(),
    };
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    STATE.save(deps.storage, &state)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::SetScore { address, score } => try_set_score(deps, info, address, score),
    }
}

pub fn try_set_score(deps: DepsMut, info: MessageInfo, address: Addr, score: i32) -> Result<Response, ContractError> {
    let state = STATE.load(deps.storage)?;
    if info.sender != state.owner {
        return Err(ContractError::Unauthorized {});
    }

    SCORE_BY_ADDRESS.update(deps.storage, address, |_d: Option<i32>| -> StdResult<i32> {Ok(score)})?;
    Ok(Response::new().add_attribute("method", "set_score"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetOwner {} => to_binary(&get_owner(deps)?),
        QueryMsg::GetScore {address} => to_binary(&get_score(deps, address)?),
    }
}

fn get_owner(deps: Deps)  -> StdResult<OwnerResponse> {
    let state = STATE.load(deps.storage)?;
    Ok(OwnerResponse { owner: state.owner })
}

fn get_score(deps: Deps, address: Addr) -> StdResult<ScoreResponse> {
    if !SCORE_BY_ADDRESS.has(deps.storage, address.clone()) {
        return Ok(ScoreResponse {address: address, score: i32::MIN});
    }
    let score = SCORE_BY_ADDRESS.load(deps.storage, address.clone())?;
    Ok(ScoreResponse {address: address, score: score})
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins, from_binary};

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies(&[]);

        let msg = InstantiateMsg { };
        let info = mock_info("creator", &coins(1000, "earth"));

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());
    }

    #[test]
    fn check_owner() {
        let mut deps = mock_dependencies(&coins(2, "token"));

        let msg = InstantiateMsg { };
        let info = mock_info("creator", &coins(2, "token"));
        let original_owner = info.sender.clone();
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // should increase counter by 1
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetOwner {}).unwrap();
        let value: OwnerResponse = from_binary(&res).unwrap();
        println!("{}", value.owner);
        assert_eq!(original_owner, value.owner);
    }

    #[test]
    #[should_panic(expected = "Unauthorized")]
    fn set_score_unauthorized() {
        let mut deps = mock_dependencies(&coins(2, "token"));

        let msg = InstantiateMsg { };
        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // beneficiary can release it
        let info = mock_info("anyone", &coins(2, "token"));
        let msg = ExecuteMsg::SetScore {address: info.sender.clone(), score: 5};
        let _res = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();
    }

    #[test]
    fn set_score() {
        let mut deps = mock_dependencies(&coins(2, "token"));

        let msg = InstantiateMsg { };
        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

        let msg = ExecuteMsg::SetScore {address: info.sender.clone(), score: 5};
        let _res = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetScore {address: info.sender.clone()}).unwrap();
        let value: ScoreResponse = from_binary(&res).unwrap();
        assert_eq!(5, value.score);
        assert_eq!(info.sender.clone(), value.address);
    }

    #[test]
    fn get_score_when_key_not_present() {
        let mut deps = mock_dependencies(&coins(2, "token"));

        let msg = InstantiateMsg { };
        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetScore {address: info.sender.clone()}).unwrap();
        let value: ScoreResponse = from_binary(&res).unwrap();
        assert_eq!(i32::MIN, value.score);
        assert_eq!(info.sender.clone(), value.address);
    }
}
