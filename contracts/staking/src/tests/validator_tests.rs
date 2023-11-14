#[cfg(test)]
mod validator_tests {
    use crate::contract::execute;
    use crate::msg::ExecuteMsg;
    use crate::tests::test_helper::{init, CELESTIA1, CELESTIA3};
    use cosmwasm_std::coins;
    use cosmwasm_std::testing::{mock_env, mock_info};

    #[test]
    fn proper_add_validator() {
        let mut deps = init();
        let info = mock_info("creator", &coins(1000, "uosmo"));
        let msg = ExecuteMsg::AddValidator {
            new_validator: CELESTIA3.to_string(),
        };

        let res = execute(deps.as_mut(), mock_env(), info, msg);
        assert!(res.is_ok());

        let attrs = res.unwrap().attributes;
        assert_eq!(attrs[0].value, "add_validator");
        assert_eq!(attrs[1].value, CELESTIA3);
    }

    #[test]
    fn duplicate_add_validator() {
        let mut deps = init();
        let info = mock_info("creator", &coins(1000, "uosmo"));
        let msg = ExecuteMsg::AddValidator {
            new_validator: CELESTIA1.to_string(),
        };

        let res = execute(deps.as_mut(), mock_env(), info, msg);
        assert!(res.is_err());
    }
    #[test]
    fn proper_remove_validator() {
        let mut deps = init();
        let info = mock_info("creator", &coins(1000, "uosmo"));
        let msg = ExecuteMsg::RemoveValidator {
            validator: CELESTIA1.to_string(),
        };

        let res = execute(deps.as_mut(), mock_env(), info, msg);
        assert!(res.is_ok());

        let attrs = res.unwrap().attributes;
        assert_eq!(attrs[0].value, "remove_validator");
        assert_eq!(attrs[1].value, CELESTIA1);
    }

    #[test]
    fn invalid_remove_validator() {
        let mut deps = init();
        let info = mock_info("creator", &coins(1000, "uosmo"));
        let msg = ExecuteMsg::RemoveValidator {
            validator: CELESTIA3.to_string(),
        };

        let res = execute(deps.as_mut(), mock_env(), info, msg);
        assert!(res.is_err());
    }
    #[test]
    fn non_admin_remove_validator() {
        let mut deps = init();
        let info = mock_info("bob", &coins(1000, "uosmo"));
        let msg = ExecuteMsg::RemoveValidator {
            validator: CELESTIA1.to_string(),
        };

        let res = execute(deps.as_mut(), mock_env(), info, msg);
        assert!(res.is_err());
    }
    #[test]
    fn non_admin_add_validator() {
        let mut deps = init();
        let info = mock_info("bob", &coins(1000, "uosmo"));
        let msg = ExecuteMsg::AddValidator {
            new_validator: CELESTIA3.to_string(),
        };

        let res = execute(deps.as_mut(), mock_env(), info, msg);
        assert!(res.is_err());
    }
}
