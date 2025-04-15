use cosmwasm_std::{
    testing::{mock_env, mock_info},
    Uint128,
};
use milky_way::staking::{Batch, BatchStatus};

use crate::{
    contract::execute, state::CONFIG, tests::test_helper::ADMIN, types::BatchExpectedAmount,
};
use crate::{error::ContractError, msg::ExecuteMsg, state::BATCHES};

use super::test_helper::init;

#[test]
fn only_admin_can_slash_batches() {
    let mut deps = init();

    let mut batch = Batch::new(1, Uint128::new(1000), 1000);
    batch.expected_native_unstaked = Some(Uint128::new(1000));
    BATCHES.save(&mut deps.storage, 1, &batch).unwrap();

    let info = mock_info("bob", &[]);
    let msg = ExecuteMsg::SlashBatches {
        new_amounts: vec![BatchExpectedAmount {
            batch_id: 1,
            amount: Uint128::new(900),
        }],
    };

    let err = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap_err();
    assert!(match err {
        ContractError::Admin { .. } => true,
        _ => false,
    });
}

#[test]
fn not_halted_contract_fails() {
    let mut deps = init();

    let mut batch = Batch::new(1, Uint128::new(1000), 1000);
    batch.expected_native_unstaked = Some(Uint128::new(1000));
    BATCHES.save(&mut deps.storage, 1, &batch).unwrap();

    let info = mock_info(ADMIN, &[]);
    let msg = ExecuteMsg::SlashBatches {
        new_amounts: vec![BatchExpectedAmount {
            batch_id: 1,
            amount: Uint128::new(900),
        }],
    };

    let err = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap_err();
    assert!(match err {
        ContractError::NotStopped {} => true,
        _ => false,
    });
}

#[test]
fn slash_not_found_batch_fails() {
    let mut deps = init();
    CONFIG
        .update(
            &mut deps.storage,
            |mut config| -> Result<_, cosmwasm_std::StdError> {
                config.stopped = true;
                Ok(config)
            },
        )
        .unwrap();

    let mut batch = Batch::new(1, Uint128::new(1000), 1000);
    batch.expected_native_unstaked = Some(Uint128::new(1000));
    BATCHES.save(&mut deps.storage, 1, &batch).unwrap();

    let info = mock_info(ADMIN, &[]);
    let msg = ExecuteMsg::SlashBatches {
        new_amounts: vec![BatchExpectedAmount {
            batch_id: 2,
            amount: Uint128::new(900),
        }],
    };

    let err = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap_err();
    assert!(match err {
        ContractError::Std { .. } => true,
        _ => false,
    });
}

#[test]
fn slash_received_batch_fails() {
    let mut deps = init();
    CONFIG
        .update(
            &mut deps.storage,
            |mut config| -> Result<_, cosmwasm_std::StdError> {
                config.stopped = true;
                Ok(config)
            },
        )
        .unwrap();

    let mut batch = Batch::new(1, Uint128::new(1000), 1000);
    batch.expected_native_unstaked = Some(Uint128::new(1000));
    batch.status = BatchStatus::Received;
    BATCHES.save(&mut deps.storage, 1, &batch).unwrap();

    let info = mock_info(ADMIN, &[]);
    let msg = ExecuteMsg::SlashBatches {
        new_amounts: vec![BatchExpectedAmount {
            batch_id: 1,
            amount: Uint128::new(900),
        }],
    };

    let err = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap_err();
    assert!(match err {
        ContractError::UnexpecedBatchStatus { .. } => true,
        _ => false,
    });
}

#[test]
fn slash_correctly() {
    let mut deps = init();
    CONFIG
        .update(
            &mut deps.storage,
            |mut config| -> Result<_, cosmwasm_std::StdError> {
                config.stopped = true;
                Ok(config)
            },
        )
        .unwrap();

    let mut batch = Batch::new(1, Uint128::new(1000), 1000);
    batch.expected_native_unstaked = Some(Uint128::new(1000));
    batch.status = BatchStatus::Pending;
    BATCHES.save(&mut deps.storage, 1, &batch).unwrap();

    let mut batch = Batch::new(2, Uint128::new(1000), 1000);
    batch.expected_native_unstaked = Some(Uint128::new(1000));
    batch.status = BatchStatus::Submitted;
    BATCHES.save(&mut deps.storage, 2, &batch).unwrap();

    let info = mock_info(ADMIN, &[]);
    let msg = ExecuteMsg::SlashBatches {
        new_amounts: vec![
            BatchExpectedAmount {
                batch_id: 1,
                amount: Uint128::new(500),
            },
            BatchExpectedAmount {
                batch_id: 2,
                amount: Uint128::new(300),
            },
        ],
    };

    execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

    let batch = BATCHES.load(&deps.storage, 1).unwrap();
    assert_eq!(Uint128::new(500), batch.expected_native_unstaked.unwrap());

    let batch = BATCHES.load(&deps.storage, 2).unwrap();
    assert_eq!(Uint128::new(300), batch.expected_native_unstaked.unwrap());
}
