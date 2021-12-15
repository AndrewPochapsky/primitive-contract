#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult,
    Uint128,
};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ConfigResponse, ExecuteMsg, GetValueResponse, InstantiateMsg, QueryMsg};
use crate::state::{Config, Primitive, CONFIG, DATA};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:primitive-contract";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let config = Config {
        owner: info.sender.clone(),
    };
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    CONFIG.save(deps.storage, &config)?;

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
        ExecuteMsg::SetValue { name, value } => execute_set_value(deps, info, name, value),
        ExecuteMsg::DeleteValue { name } => execute_delete_value(deps, info, name),
    }
}

pub fn execute_set_value(
    deps: DepsMut,
    info: MessageInfo,
    name: String,
    value: Primitive,
) -> Result<Response, ContractError> {
    match DATA.load(deps.storage, (&info.sender, &name)) {
        Ok(_) => {
            DATA.update(deps.storage, (&info.sender, &name), |old| match old {
                Some(_) => Ok(value.clone()),
                None => Err(StdError::GenericErr {
                    msg: "error".to_string(),
                }),
            })?;
        }
        Err(_) => {
            DATA.update(deps.storage, (&info.sender, &name), |old| match old {
                Some(_) => Err(StdError::GenericErr {
                    msg: "error".to_string(),
                }),
                None => Ok(value.clone()),
            })?;
        }
    }

    Ok(Response::new()
        .add_attribute("method", "set_value")
        .add_attribute("sender", info.sender)
        .add_attribute("name", name)
        .add_attribute("value", format!("{:?}", value)))
}

pub fn execute_delete_value(
    deps: DepsMut,
    info: MessageInfo,
    name: String,
) -> Result<Response, ContractError> {
    DATA.remove(deps.storage, (&info.sender, &name));

    Ok(Response::new()
        .add_attribute("method", "delete_value")
        .add_attribute("sender", info.sender)
        .add_attribute("name", name))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_binary(&query_config(deps)?),
        QueryMsg::GetValue { address, name } => to_binary(&query_value(deps, address, name)?),
    }
}

fn query_value(deps: Deps, address: Addr, name: String) -> StdResult<GetValueResponse> {
    let value = DATA.load(deps.storage, (&address, &name))?;
    Ok(GetValueResponse { name, value })
}

fn query_config(deps: Deps) -> StdResult<ConfigResponse> {
    let config = CONFIG.load(deps.storage)?;
    Ok(ConfigResponse {
        owner: config.owner,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::from_binary;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies(&[]);

        let msg = InstantiateMsg {};
        let info = mock_info("creator", &[]);

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());
    }

    #[test]
    fn set_value() {
        let mut deps = mock_dependencies(&[]);

        let msg = InstantiateMsg {};
        let info = mock_info("creator", &[]);

        // we can just call .unwrap() to assert this was a success
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        let user1 = mock_info("user1", &[]);
        let msg = ExecuteMsg::SetValue {
            name: "test1".to_string(),
            value: Primitive::String("value1".to_string()),
        };
        let res = execute(deps.as_mut(), mock_env(), user1.clone(), msg).unwrap();
        assert_eq!(
            res,
            Response::new()
                .add_attribute("method", "set_value")
                .add_attribute("sender", "user1")
                .add_attribute("name", "test1")
                .add_attribute("value", "String(\"value1\")")
        );

        let query_res: GetValueResponse = from_binary(
            &query(
                deps.as_ref(),
                mock_env(),
                QueryMsg::GetValue {
                    address: user1.sender.clone(),
                    name: "test1".to_string(),
                },
            )
            .unwrap(),
        )
        .unwrap();

        assert_eq!(query_res.name, "test1".to_string());
        assert_eq!(query_res.value, Primitive::String("value1".to_string()));
    }
    #[test]
    fn delete_value() {
        let mut deps = mock_dependencies(&[]);

        let msg = InstantiateMsg {};
        let info = mock_info("creator", &[]);

        // we can just call .unwrap() to assert this was a success
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        let user1 = mock_info("user1", &[]);
        let msg = ExecuteMsg::SetValue {
            name: "test1".to_string(),
            value: Primitive::String("value1".to_string()),
        };
        let _res = execute(deps.as_mut(), mock_env(), user1.clone(), msg).unwrap();

        let query_res: GetValueResponse = from_binary(
            &query(
                deps.as_ref(),
                mock_env(),
                QueryMsg::GetValue {
                    address: user1.sender.clone(),
                    name: "test1".to_string(),
                },
            )
            .unwrap(),
        )
        .unwrap();

        assert_eq!(query_res.name, "test1".to_string());
        assert_eq!(query_res.value, Primitive::String("value1".to_string()));

        let msg = ExecuteMsg::DeleteValue {
            name: "test1".to_string(),
        };
        let res = execute(deps.as_mut(), mock_env(), user1.clone(), msg).unwrap();
        assert_eq!(
            res,
            Response::new()
                .add_attribute("method", "delete_value")
                .add_attribute("sender", "user1")
                .add_attribute("name", "test1")
        );
        let query_res = &query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::GetValue {
                address: user1.sender.clone(),
                name: "test1".to_string(),
            },
        );
        assert!(query_res.is_err());
    }
}
