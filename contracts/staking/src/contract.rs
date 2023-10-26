use crate::helpers::validate_addresses;
use crate::state::{Config, State, ADMIN, CONFIG, STATE};
#[cfg(not(feature = "library"))]
use crate::{
    error::ContractError,
    execute::{
        execute_accept_ownership, execute_add_validator, execute_claim, execute_liquid_stake,
        execute_liquid_unstake, execute_remove_validator, execute_revoke_ownership_transfer,
        execute_transfer_ownership,
    },
    msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg},
};
use cosmwasm_std::CosmosMsg;
use cosmwasm_std::{
    entry_point, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128,
};
use cw2::set_contract_version;
use cw_utils::must_pay;
use osmosis_std::types::osmosis::tokenfactory::v1beta1::MsgCreateDenom;
// version info for migration info
const CONTRACT_NAME: &str = "crates.io:staking";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

///////////////////
/// INSTANTIATE ///
///////////////////
//TODO: Add validations
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    mut deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    let api = deps.api;
    let node_operators = validate_addresses(api, msg.node_operators)?;
    let validators = validate_addresses(api, msg.validators)?;

    // TODO: determine if info.sender is the admin or if we want to pass in with msg
    ADMIN.set(deps.branch(), Some(info.sender.clone()))?;

    // Init Config
    let config = Config {
        native_token_denom: msg.native_token_denom,
        liquid_stake_token_denom: format!(
            "factory/{0}/{1}",
            env.contract.address, msg.liquid_stake_token_denom
        ), //TODO determine the format to save in
        treasury_address: deps.api.addr_validate(&msg.treasury_address)?,
        node_operators,
        validators,
        batch_period: msg.batch_period,
        unbonding_period: msg.unbonding_period,
        protocol_fee_config: msg.protocol_fee_config,
        multisig_address_config: msg.multisig_address_config,
        minimum_liquid_stake_amount: msg.minimum_liquid_stake_amount,
        minimum_rewards_to_collect: msg.minimum_rewards_to_collect,
    };

    CONFIG.save(deps.storage, &config)?;

    // Init State
    let state = State {
        total_native_token: Uint128::zero(),
        total_liquid_stake_token: Uint128::zero(),
        native_token_to_stake: Uint128::zero(),
        pending_owner: None,
    };
    STATE.save(deps.storage, &state)?;

    // Create liquid stake token denom
    let tokenfactory_msg = MsgCreateDenom {
        sender: env.contract.address.to_string(),
        subdenom: config.liquid_stake_token_denom,
    };

    let cosmos_tokenfactory_msg: CosmosMsg = tokenfactory_msg.into();

    //TODO Update attributes
    Ok(Response::new()
        .add_attribute("action", "instantiate")
        .add_attribute("owner", info.sender)
        .add_message(cosmos_tokenfactory_msg))
}

///////////////
/// EXECUTE ///
///////////////

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    match msg {
        ExecuteMsg::LiquidStake {} => {
            let payment = must_pay(&info, &config.native_token_denom)?;
            execute_liquid_stake(deps, env, info, payment)
        }
        ExecuteMsg::LiquidUnstake {} => {
            let payment = must_pay(&info, &config.liquid_stake_token_denom)?;
            execute_liquid_unstake(deps, env, info, payment)
        }
        ExecuteMsg::Claim {} => execute_claim(deps, env, info),
        ExecuteMsg::TransferOwnership { new_owner } => {
            execute_transfer_ownership(deps, env, info, new_owner)
        }
        ExecuteMsg::AcceptOwnership {} => execute_accept_ownership(deps, env, info),
        ExecuteMsg::RevokeOwnershipTransfer {} => {
            execute_revoke_ownership_transfer(deps, env, info)
        }
        ExecuteMsg::AddValidator { new_validator } => {
            execute_add_validator(deps, env, info, new_validator)
        }
        ExecuteMsg::RemoveValidator { validator } => {
            execute_remove_validator(deps, env, info, validator)
        }
    }
}

/////////////
/// QUERY ///
/////////////

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    unimplemented!()
}

///////////////
/// MIGRATE ///
///////////////

#[entry_point]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    // TODO: note implement yet
    Ok(Response::new())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{MultisigAddressConfig, ProtocolFeeConfig};
    use cosmwasm_std::testing::{
        mock_dependencies, mock_env, mock_info, MockApi, MockStorage, MOCK_CONTRACT_ADDR,
    };
    use cosmwasm_std::{
        coins, from_binary, Addr, Attribute, ContractResult, CosmosMsg, OwnedDeps, Querier,
        StdError, SystemError, SystemResult,
    };

    fn init(mut deps: DepsMut) {
        let msg = InstantiateMsg {
            native_token_denom: "osmoTIA".to_string(),
            liquid_stake_token_denom: "stTIA".to_string(),
            treasury_address: "treasury".to_string(),
            node_operators: vec!["node1".to_string(), "node2".to_string()],
            validators: vec!["val1".to_string(), "val2".to_string()],
            batch_period: 86400,
            unbonding_period: 1209600,
            protocol_fee_config: ProtocolFeeConfig {
                dao_treasury_fee: Uint128::from(10u128),
            },
            multisig_address_config: MultisigAddressConfig {
                controller_address: Addr::unchecked("staker"),
                staker_address: Addr::unchecked("staker"),
                reward_collector_address: Addr::unchecked("reward_collector"),
            },
            minimum_liquid_stake_amount: Uint128::from(100u128),
            minimum_rewards_to_collect: Uint128::from(10u128),
        };
        let info = mock_info("creator", &coins(1000, "uosmo"));

        let res = instantiate(deps, mock_env(), info, msg);
    }
    #[test]
    fn proper_instantiation() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg {
            native_token_denom: "osmoTIA".to_string(),
            liquid_stake_token_denom: "stTIA".to_string(),
            treasury_address: "treasury".to_string(),
            node_operators: vec!["node1".to_string(), "node2".to_string()],
            validators: vec!["val1".to_string(), "val2".to_string()],
            batch_period: 86400,
            unbonding_period: 1209600,
            protocol_fee_config: ProtocolFeeConfig {
                dao_treasury_fee: Uint128::from(10u128),
            },
            multisig_address_config: MultisigAddressConfig {
                controller_address: Addr::unchecked("staker"),
                staker_address: Addr::unchecked("staker"),
                reward_collector_address: Addr::unchecked("reward_collector"),
            },
            minimum_liquid_stake_amount: Uint128::from(100u128),
            minimum_rewards_to_collect: Uint128::from(10u128),
        };
        let info = mock_info("creator", &coins(1000, "uosmo"));

        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(1, res.messages.len());
        let attrs = res.attributes;
        assert_eq!(attrs[0].value, "instantiate");
    }
    #[test]
    fn proper_add_validator() {
        let mut deps = mock_dependencies();
        let res = init(deps.as_mut());
        let info = mock_info("creator", &coins(1000, "uosmo"));
        let msg = ExecuteMsg::AddValidator {
            new_validator: "val3".to_string(),
        };

        let res = execute(deps.as_mut(), mock_env(), info, msg);
        assert!(res.is_ok());

        let attrs = res.unwrap().attributes;
        assert_eq!(attrs[0].value, "add_validator");
        assert_eq!(attrs[1].value, "val3");
    }

    #[test]
    fn duplicate_add_validator() {
        let mut deps = mock_dependencies();
        let res = init(deps.as_mut());
        let info = mock_info("creator", &coins(1000, "uosmo"));
        let msg = ExecuteMsg::AddValidator {
            new_validator: "val1".to_string(),
        };

        let res = execute(deps.as_mut(), mock_env(), info, msg);
        assert!(res.is_err());
    }
    #[test]
    fn proper_remove_validator() {
        let mut deps = mock_dependencies();
        let res = init(deps.as_mut());
        let info = mock_info("creator", &coins(1000, "uosmo"));
        let msg = ExecuteMsg::RemoveValidator {
            validator: "val1".to_string(),
        };

        let res = execute(deps.as_mut(), mock_env(), info, msg);
        assert!(res.is_ok());

        let attrs = res.unwrap().attributes;
        assert_eq!(attrs[0].value, "remove_validator");
        assert_eq!(attrs[1].value, "val1");
    }

    #[test]
    fn invalid_remove_validator() {
        let mut deps = mock_dependencies();
        let res = init(deps.as_mut());
        let info = mock_info("creator", &coins(1000, "uosmo"));
        let msg = ExecuteMsg::RemoveValidator {
            validator: "val3".to_string(),
        };

        let res = execute(deps.as_mut(), mock_env(), info, msg);
        assert!(res.is_err());
    }
    #[test]
    fn non_admin_remove_validator() {
        let mut deps = mock_dependencies();
        let res = init(deps.as_mut());
        let info = mock_info("bob", &coins(1000, "uosmo"));
        let msg = ExecuteMsg::RemoveValidator {
            validator: "val1".to_string(),
        };

        let res = execute(deps.as_mut(), mock_env(), info, msg);
        assert!(res.is_err());
    }
    #[test]
    fn non_admin_add_validator() {
        let mut deps = mock_dependencies();
        let res = init(deps.as_mut());
        let info = mock_info("bob", &coins(1000, "uosmo"));
        let msg = ExecuteMsg::AddValidator {
            new_validator: "val3".to_string(),
        };

        let res = execute(deps.as_mut(), mock_env(), info, msg);
        assert!(res.is_err());
    }
    #[test]
    fn proper_transfer_ownership() {
        let mut deps = mock_dependencies();
        let res = init(deps.as_mut());
        let info = mock_info("creator", &coins(1000, "uosmo"));
        let msg = ExecuteMsg::TransferOwnership {
            new_owner: "new_owner".to_string(),
        };

        let res = execute(deps.as_mut(), mock_env(), info, msg);
        assert!(res.is_ok());

        let attrs = res.unwrap().attributes;
        assert_eq!(attrs[0].value, "transfer_ownership");
        assert_eq!(attrs[1].value, "new_owner");
    }
    #[test]
    fn non_admin_transfer_ownership() {
        let mut deps = mock_dependencies();
        let res = init(deps.as_mut());
        let info = mock_info("bob", &coins(1000, "uosmo"));
        let msg = ExecuteMsg::TransferOwnership {
            new_owner: "new_owner".to_string(),
        };

        let res = execute(deps.as_mut(), mock_env(), info, msg);
        assert!(res.is_err());
    }
    #[test]
    fn proper_claim_ownership() {
        let mut deps = mock_dependencies();
        let res = init(deps.as_mut());
        let info = mock_info("creator", &coins(1000, "uosmo"));
        let msg = ExecuteMsg::TransferOwnership {
            new_owner: "new_owner".to_string(),
        };

        let res = execute(deps.as_mut(), mock_env(), info, msg);
        assert!(res.is_ok());

        let info = mock_info("new_owner", &coins(1000, "uosmo"));
        let msg = ExecuteMsg::AcceptOwnership {};

        let res2 = execute(deps.as_mut(), mock_env(), info, msg);

        let attrs = res2.unwrap().attributes;
        assert_eq!(attrs[0].value, "accept_ownership");
        assert_eq!(attrs[1].value, "new_owner");
    }
    #[test]
    fn unauthorized_claim_ownership() {
        let mut deps = mock_dependencies();
        let res = init(deps.as_mut());
        let info = mock_info("creator", &coins(1000, "uosmo"));
        let msg = ExecuteMsg::TransferOwnership {
            new_owner: "new_owner".to_string(),
        };

        let res = execute(deps.as_mut(), mock_env(), info, msg);
        assert!(res.is_ok());

        let info = mock_info("bob", &coins(1000, "uosmo"));
        let msg = ExecuteMsg::AcceptOwnership {};

        let res2 = execute(deps.as_mut(), mock_env(), info, msg);

        assert!(res2.is_err());
    }
    #[test]
    fn proper_revoke_ownership_transfer() {
        let mut deps = mock_dependencies();
        let res = init(deps.as_mut());
        let info = mock_info("creator", &coins(1000, "uosmo"));
        let msg = ExecuteMsg::TransferOwnership {
            new_owner: "new_owner".to_string(),
        };

        let res = execute(deps.as_mut(), mock_env(), info, msg);
        assert!(res.is_ok());

        let info = mock_info("creator", &coins(1000, "uosmo"));
        let msg = ExecuteMsg::RevokeOwnershipTransfer {};

        let res2 = execute(deps.as_mut(), mock_env(), info, msg);

        let attrs = res2.unwrap().attributes;
        assert_eq!(attrs[0].value, "revoke_ownership_transfer");
    }
    #[test]
    fn non_admin_revoke_ownership_transfer() {
        let mut deps = mock_dependencies();
        let res = init(deps.as_mut());
        let info = mock_info("bob", &coins(1000, "uosmo"));
        let msg = ExecuteMsg::RevokeOwnershipTransfer {};

        let res2 = execute(deps.as_mut(), mock_env(), info, msg);

        assert!(res2.is_err());
    }
}
