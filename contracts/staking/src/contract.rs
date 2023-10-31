use crate::execute::execute_submit_batch;
use crate::helpers::validate_addresses;
use crate::state::{Config, IbcConfig, State, ADMIN, CONFIG, IBC_CONFIG, PENDING_BATCH, STATE};
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
use cosmwasm_std::{
    entry_point, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128,
};
use cosmwasm_std::{CosmosMsg, IbcChannelOpenMsg, Timestamp};
use cw2::set_contract_version;
use cw_utils::must_pay;
use milky_way::staking::Batch;
use osmosis_std::types::osmosis::tokenfactory::v1beta1::MsgCreateDenom;
// version info for migration info
const CONTRACT_NAME: &str = "crates.io:staking";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
// TODO: Placeholder value for IBC timeout
const IBC_TIMEOUT: Timestamp = Timestamp::from_nanos(1000000000000);

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
        subdenom: msg.liquid_stake_token_denom,
    };

    let cosmos_tokenfactory_msg: CosmosMsg = tokenfactory_msg.into();

    let pending_batch = Batch::new(
        1,
        Uint128::zero(),
        env.block.time.seconds() + config.batch_period,
    );
    // Set pending batch and batches
    PENDING_BATCH.save(deps.storage, &pending_batch)?;

    let ibc_config = IbcConfig {
        channel: None,
        default_timeout: IBC_TIMEOUT,
    };
    IBC_CONFIG.save(deps.storage, &ibc_config)?;

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
        ExecuteMsg::SubmitBatch { batch_id } => execute_submit_batch(deps, env, info, batch_id),
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
    use crate::state::{MultisigAddressConfig, ProtocolFeeConfig, IBC_CONFIG};

    use cosmwasm_std::testing::{
        mock_dependencies, mock_env, mock_ibc_channel, mock_info, MockApi, MockQuerier, MockStorage,
    };
    use cosmwasm_std::{coins, Addr, OwnedDeps};

    fn init() -> OwnedDeps<MockStorage, MockApi, MockQuerier> {
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

        let _res = instantiate(deps.as_mut(), mock_env(), info, msg);

        let channel = cosmwasm_std::testing::mock_ibc_channel(
            "channel-123",
            cosmwasm_std::IbcOrder::Unordered,
            "mw-1",
        );
        let ibc_config = IbcConfig {
            channel: Some(channel),
            default_timeout: IBC_TIMEOUT,
        };
        IBC_CONFIG.save(&mut deps.storage, &ibc_config).unwrap();

        deps
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

        let batch = PENDING_BATCH.load(&deps.storage).unwrap();
        assert!(batch.id == 1);
    }
    #[test]
    fn proper_add_validator() {
        let mut deps = init();
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
        let mut deps = init();
        let info = mock_info("creator", &coins(1000, "uosmo"));
        let msg = ExecuteMsg::AddValidator {
            new_validator: "val1".to_string(),
        };

        let res = execute(deps.as_mut(), mock_env(), info, msg);
        assert!(res.is_err());
    }
    #[test]
    fn proper_remove_validator() {
        let mut deps = init();
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
        let mut deps = init();
        let info = mock_info("creator", &coins(1000, "uosmo"));
        let msg = ExecuteMsg::RemoveValidator {
            validator: "val3".to_string(),
        };

        let res = execute(deps.as_mut(), mock_env(), info, msg);
        assert!(res.is_err());
    }
    #[test]
    fn non_admin_remove_validator() {
        let mut deps = init();
        let info = mock_info("bob", &coins(1000, "uosmo"));
        let msg = ExecuteMsg::RemoveValidator {
            validator: "val1".to_string(),
        };

        let res = execute(deps.as_mut(), mock_env(), info, msg);
        assert!(res.is_err());
    }
    #[test]
    fn non_admin_add_validator() {
        let mut deps = init();
        let info = mock_info("bob", &coins(1000, "uosmo"));
        let msg = ExecuteMsg::AddValidator {
            new_validator: "val3".to_string(),
        };

        let res = execute(deps.as_mut(), mock_env(), info, msg);
        assert!(res.is_err());
    }
    #[test]
    fn proper_transfer_ownership() {
        let mut deps = init();
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
        let mut deps = init();
        let info = mock_info("bob", &coins(1000, "uosmo"));
        let msg = ExecuteMsg::TransferOwnership {
            new_owner: "new_owner".to_string(),
        };

        let res = execute(deps.as_mut(), mock_env(), info, msg);
        assert!(res.is_err());
    }
    #[test]
    fn proper_claim_ownership() {
        let mut deps = init();
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
        let mut deps = init();
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
        let mut deps = init();
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
        let mut deps = init();
        let info = mock_info("bob", &coins(1000, "uosmo"));
        let msg = ExecuteMsg::RevokeOwnershipTransfer {};

        let res2 = execute(deps.as_mut(), mock_env(), info, msg);

        assert!(res2.is_err());
    }
    #[test]
    fn proper_liquid_stake() {
        let mut deps = init();
        let info = mock_info("creator", &coins(1000, "osmoTIA"));
        let msg = ExecuteMsg::LiquidStake {};

        let res = execute(deps.as_mut(), mock_env(), info.clone(), msg);

        assert!(res.is_ok());

        // Unwrap once and store in a variable
        let unwrapped_res = res.unwrap();

        let attrs = &unwrapped_res.attributes;
        assert_eq!(attrs[0].value, "liquid_stake");

        let batch = PENDING_BATCH.load(&deps.storage).unwrap();
        assert!(batch.id == 1);

        // Use the previously unwrapped value
        assert_eq!(unwrapped_res.messages.len(), 2);
    }
    // // Create initial stake for bob
    //     fn prep_liquid_stake() -> OwnedDeps<MockStorage, MockApi, MockQuerier> {

    //         let mut deps = init();
    //         let info = mock_info("bob", &coins(1000, "osmoTIA"));
    //         let msg = ExecuteMsg::LiquidStake {};

    //         let res = execute(deps.as_mut(), mock_env(), info.clone(), msg);
    //         let batch = PENDING_BATCH.load(&deps.storage).unwrap();
    //         deps

    //     }

    #[test]
    fn proper_liquid_unstake() {
        let mut deps = init();

        let mut state = STATE.load(&deps.storage).unwrap();

        state.total_liquid_stake_token = Uint128::from(100_000u128);
        STATE.save(&mut deps.storage, &state).unwrap();

        let info = mock_info("bob", &coins(1000, "factory/cosmos2contract/stTIA"));
        let msg = ExecuteMsg::LiquidUnstake {};

        let res = execute(deps.as_mut(), mock_env(), info.clone(), msg);

        let resp = res.unwrap();

        let attrs = resp.attributes;
        assert_eq!(attrs[0].value, "liquid_unstake");

        // Submit batch
        assert_eq!(resp.messages.len(), 1);
    }
    // #[test]
    // fn double_liquid_unstake() {
    //     let mut deps = init();

    //     let mut state = STATE.load(&deps.storage).unwrap();

    //     state.total_liquid_stake_token = Uint128::from(100_000u128);
    //     STATE.save(&mut deps.storage, &state).unwrap();

    //     let mut pending_batch = PENDING_BATCH.load(&deps.storage).unwrap();
    //     pending_batch.liquid_unstake_requests.insert(
    //         Addr::unchecked("bob"),
    //         LiquidUnstakeRequest {
    //             user: Addr::unchecked("bob"),
    //             shares: Uint128::from(100u128),
    //         },
    //     );

    //     // PENDING_BATCH.save(&mut deps.storage, &pending_batch).unwrap();

    //     let info = mock_info("bob", &coins(1000, "factory/cosmos2contract/stTIA"));
    //     let msg = ExecuteMsg::LiquidUnstake {};

    //     let res = execute(deps.as_mut(), mock_env(), info.clone(), msg);

    //     let resp = res.unwrap();

    //     let attrs = resp.attributes;
    //     assert_eq!(attrs[0].value, "liquid_unstake");

    //     // Submit batch
    //     assert_eq!(resp.messages.len(), 1);

    // }
    #[test]
    fn invalid_denom_liquid_unstake() {
        let mut deps = init();

        let mut state = STATE.load(&deps.storage).unwrap();

        state.total_liquid_stake_token = Uint128::from(100_000u128);
        STATE.save(&mut deps.storage, &state).unwrap();

        let info = mock_info("bob", &coins(1000, "factory/bob/stTIA"));
        let msg = ExecuteMsg::LiquidUnstake {};

        let res = execute(deps.as_mut(), mock_env(), info, msg);

        assert!(res.is_err());
    }
    #[test]
    fn proper_submit_batch() {
        let mut deps = init();
        let mut env = mock_env();

        let mut state = STATE.load(&deps.storage).unwrap();
        let config = CONFIG.load(&deps.storage).unwrap();

        state.total_liquid_stake_token = Uint128::from(100_000u128);
        STATE.save(&mut deps.storage, &state).unwrap();

        // print!("{:?}", msgs);
        env.block.time = env.block.time.plus_seconds(config.batch_period + 1);
        let msg = ExecuteMsg::SubmitBatch { batch_id: 1 };

        let contract = env.contract.address.clone().to_string();

        let info = mock_info(&contract, &[]);
        let res = execute(deps.as_mut(), env, info, msg);
        print!("{:?}", res);
        assert!(res.is_ok());


    }
    #[test]
    fn not_ready_submit_batch() {
        let mut deps = init();
        let mut env = mock_env();

        let mut state = STATE.load(&deps.storage).unwrap();
        let config = CONFIG.load(&deps.storage).unwrap();

        state.total_liquid_stake_token = Uint128::from(100_000u128);
        STATE.save(&mut deps.storage, &state).unwrap();

        // batch isnt ready
        env.block.time = env.block.time.plus_seconds(config.batch_period - 1);
        let msg = ExecuteMsg::SubmitBatch { batch_id: 1 };

        let contract = env.contract.address.clone().to_string();

        let info = mock_info(&contract, &[]);
        let res = execute(deps.as_mut(), env, info, msg);
        
        assert!(res.is_err());


    }
}
