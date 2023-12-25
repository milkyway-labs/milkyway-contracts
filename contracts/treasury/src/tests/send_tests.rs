#[cfg(test)]
mod ownership_tests {
    use crate::contract::execute;
    use crate::msg::ExecuteMsg;
    use crate::tests::test_helper::{init, OSMO3};
    use cosmwasm_std::testing::{mock_env, mock_info};
    use cosmwasm_std::{Coin, Uint128};

    #[test]
    fn proper_send() {
        let mut deps = init();
        let info = mock_info(OSMO3, &vec![]);

        let msg = ExecuteMsg::SpendFunds {
            amount: Coin {
                denom: "uosmo".to_string(),
                amount: Uint128::from(1000u128),
            },
            receiver: "celestia1sfhy3emrgp26wnzuu64p06kpkxd9phel74e0yx".to_string(),
            channel_id: None,
        };

        let res = execute(deps.as_mut(), mock_env(), info.clone(), msg);
        assert!(res.is_err()); // wrong address

        let msg = ExecuteMsg::SpendFunds {
            amount: Coin {
                denom: "uosmo".to_string(),
                amount: Uint128::from(1000u128),
            },
            receiver: "osmo12z558dm3ew6avgjdj07mfslx80rp9sh8nt7q3w".to_string(),
            channel_id: None,
        };

        let res = execute(deps.as_mut(), mock_env(), info.clone(), msg);
        assert!(res.is_ok());

        let attrs = res.unwrap().attributes;
        assert_eq!(attrs[0].value, "spend_funds");
        assert_eq!(
            attrs[1].value,
            "osmo12z558dm3ew6avgjdj07mfslx80rp9sh8nt7q3w"
        );
        assert_eq!(attrs[2].value, "1000");
        assert_eq!(attrs[3].value, "uosmo");
    }

    #[test]
    fn proper_send_ibc() {
        let mut deps = init();
        let info = mock_info(OSMO3, &vec![]);

        let msg = ExecuteMsg::SpendFunds {
            amount: Coin {
                denom: "uosmo".to_string(),
                amount: Uint128::from(1000u128),
            },
            receiver: "osmo12z558dm3ew6avgjdj07mfslx80rp9sh8nt7q3w".to_string(),
            channel_id: Some("channel-123".to_string()),
        };

        let res = execute(deps.as_mut(), mock_env(), info.clone(), msg);
        assert!(res.is_err()); // wrong address

        let msg = ExecuteMsg::SpendFunds {
            amount: Coin {
                denom: "uosmo".to_string(),
                amount: Uint128::from(1000u128),
            },
            receiver: "celestia1sfhy3emrgp26wnzuu64p06kpkxd9phel74e0yx".to_string(),
            channel_id: Some("channel-123".to_string()),
        };

        let res = execute(deps.as_mut(), mock_env(), info.clone(), msg);
        assert!(res.is_ok());

        let attrs = res.unwrap().attributes;
        assert_eq!(attrs[0].value, "spend_funds");
        assert_eq!(
            attrs[1].value,
            "celestia1sfhy3emrgp26wnzuu64p06kpkxd9phel74e0yx"
        );
        assert_eq!(attrs[2].value, "1000");
        assert_eq!(attrs[3].value, "uosmo");
        assert_eq!(attrs[4].value, "channel-123");
    }
}
