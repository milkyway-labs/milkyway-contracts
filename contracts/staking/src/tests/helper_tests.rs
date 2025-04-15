use crate::helpers::{compute_mint_amount, compute_unbond_amount};
use cosmwasm_std::Uint128;

// Basic test - based on figures from excalidraw
#[test]
fn test_compute_mint_amount() {
    let total_native_token = Uint128::from(2_000_000_000u128);
    let total_liquid_stake_token = Uint128::from(1_800_000_000u128);
    let native_to_stake = Uint128::from(100_000_000u128);
    let mint_amount = compute_mint_amount(
        total_native_token,
        total_liquid_stake_token,
        native_to_stake,
    );

    assert_eq!(mint_amount, Uint128::from(90_000_000u128));
}

// Basic test - based on figures from excalidraw
#[test]
fn test_compute_unbond_amount() {
    let total_native_token = Uint128::from(2_000_000_000u128);
    let total_liquid_stake_token = Uint128::from(1_800_000_000u128);
    let batch_unstake = Uint128::from(90_000_000u128);
    let unbond_amount =
        compute_unbond_amount(total_native_token, total_liquid_stake_token, batch_unstake);

    assert_eq!(unbond_amount, Uint128::from(100_000_000u128));
}
