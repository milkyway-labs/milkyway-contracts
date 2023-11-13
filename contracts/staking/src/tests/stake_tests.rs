#[cfg(test)]
mod staking_tests {
    use crate::contract::execute;
    use crate::msg::ExecuteMsg;
    use crate::state::BATCHES;
    use crate::tests::test_helper::init;
    use cosmwasm_std::testing::{mock_env, mock_info};
    use cosmwasm_std::{coins, Order};
    use milky_way::staking::BatchStatus;

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

        let pending_batch = BATCHES
            .range(&deps.storage, None, None, Order::Descending)
            .find(|r| r.is_ok() && r.as_ref().unwrap().1.status == BatchStatus::Pending)
            .unwrap()
            .unwrap()
            .1;
        assert!(pending_batch.id == 1);

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
}
