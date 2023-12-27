#[cfg(test)]
mod submit_batch_tests {
    use crate::contract::execute;
    use crate::msg::ExecuteMsg;
    use crate::state::{CONFIG, STATE};
    use crate::tests::test_helper::init;
    use cosmwasm_std::testing::{mock_env, mock_info};
    use cosmwasm_std::Uint128;

    #[test]
    fn empty_submit_batch() {
        let mut deps = init();
        let mut env = mock_env();

        let state = STATE.load(&deps.storage).unwrap();
        let config = CONFIG.load(&deps.storage).unwrap();

        STATE.save(&mut deps.storage, &state).unwrap();

        // print!("{:?}", msgs);
        env.block.time = env.block.time.plus_seconds(config.batch_period + 1);
        let msg = ExecuteMsg::SubmitBatch {};

        let contract = env.contract.address.clone().to_string();

        let info = mock_info(&contract, &[]);
        let res = execute(deps.as_mut(), env, info, msg);
        print!("{:?}", res);
        assert!(res.is_err());
    }

    #[test]
    fn not_ready_submit_batch() {
        let mut deps = init();
        let mut env = mock_env();

        let mut state = STATE.load(&deps.storage).unwrap();
        let config = CONFIG.load(&deps.storage).unwrap();

        state.total_liquid_stake_token = Uint128::from(100_000u128);
        state.total_native_token = Uint128::from(300_000u128);
        STATE.save(&mut deps.storage, &state).unwrap();

        // batch isnt ready
        env.block.time = env.block.time.plus_seconds(config.batch_period - 1);
        let msg = ExecuteMsg::SubmitBatch {};

        let contract = env.contract.address.clone().to_string();

        let info = mock_info(&contract, &[]);
        let res = execute(deps.as_mut(), env, info, msg);

        assert!(res.is_err());
    }
}
