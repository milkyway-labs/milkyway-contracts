use crate::contract::execute;
use crate::msg::ExecuteMsg;
use crate::tests::test_helper::{init, OSMO3};
use cosmwasm_std::testing::{message_info, mock_env};
use cosmwasm_std::{coins, Addr};

#[test]
fn proper_transfer_ownership() {
    let mut deps = init();
    let info = message_info(&Addr::unchecked(OSMO3), &coins(1000, "uosmo"));
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
    let info = message_info(&deps.api.addr_make("bob"), &coins(1000, "uosmo"));
    let msg = ExecuteMsg::TransferOwnership {
        new_owner: "new_owner".to_string(),
    };

    let res = execute(deps.as_mut(), mock_env(), info, msg);
    assert!(res.is_err());
}

#[test]
fn proper_claim_ownership() {
    let mut deps = init();
    let info = message_info(&Addr::unchecked(OSMO3), &coins(1000, "uosmo"));
    let msg = ExecuteMsg::TransferOwnership {
        new_owner: "new_owner".to_string(),
    };

    let mut env = mock_env();

    let res = execute(deps.as_mut(), env.clone(), info, msg);
    assert!(res.is_ok());

    let info = message_info(&deps.api.addr_make("new_owner"), &coins(1000, "uosmo"));
    let msg = ExecuteMsg::AcceptOwnership {};

    let res = execute(deps.as_mut(), env.clone(), info.clone(), msg.clone());
    assert!(res.is_err()); // no time yet

    env.block.time = mock_env().block.time.plus_seconds(60 * 60 * 24 * 7);

    let res = execute(deps.as_mut(), env.clone(), info.clone(), msg.clone());
    assert!(res.is_ok());

    let attrs = res.unwrap().attributes;
    assert_eq!(attrs[0].value, "accept_ownership");
    assert_eq!(attrs[1].value, "new_owner");
}

#[test]
fn unauthorized_claim_ownership() {
    let mut deps = init();
    let info = message_info(&Addr::unchecked(OSMO3), &coins(1000, "uosmo"));
    let msg = ExecuteMsg::TransferOwnership {
        new_owner: "new_owner".to_string(),
    };

    let res = execute(deps.as_mut(), mock_env(), info, msg);
    assert!(res.is_ok());

    let info = message_info(&deps.api.addr_make("bob"), &coins(1000, "uosmo"));
    let msg = ExecuteMsg::AcceptOwnership {};

    let res2 = execute(deps.as_mut(), mock_env(), info, msg);

    assert!(res2.is_err());
}

#[test]
fn proper_revoke_ownership_transfer() {
    let mut deps = init();
    let info = message_info(&Addr::unchecked(OSMO3), &coins(1000, "uosmo"));
    let msg = ExecuteMsg::TransferOwnership {
        new_owner: "new_owner".to_string(),
    };

    let res = execute(deps.as_mut(), mock_env(), info, msg);
    assert!(res.is_ok());

    let info = message_info(&Addr::unchecked(OSMO3), &coins(1000, "uosmo"));
    let msg = ExecuteMsg::RevokeOwnershipTransfer {};

    let res2 = execute(deps.as_mut(), mock_env(), info, msg);

    let attrs = res2.unwrap().attributes;
    assert_eq!(attrs[0].value, "revoke_ownership_transfer");
}

#[test]
fn non_admin_revoke_ownership_transfer() {
    let mut deps = init();
    let info = message_info(&deps.api.addr_make("bob"), &coins(1000, "uosmo"));
    let msg = ExecuteMsg::RevokeOwnershipTransfer {};

    let res2 = execute(deps.as_mut(), mock_env(), info, msg);

    assert!(res2.is_err());
}
