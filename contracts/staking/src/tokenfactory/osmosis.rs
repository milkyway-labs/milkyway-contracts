use cosmwasm_std::{Coin, CosmosMsg};
use osmosis_std::types::osmosis::tokenfactory::v1beta1::{MsgBurn, MsgCreateDenom, MsgMint};

pub fn create_denom(sender: String, subdenom: String) -> CosmosMsg {
    MsgCreateDenom { sender, subdenom }.into()
}

pub fn mint(sender: String, amount: Coin, mint_to_address: String) -> CosmosMsg {
    MsgMint {
        sender,
        amount: Some(osmosis_std::types::cosmos::base::v1beta1::Coin {
            denom: amount.denom,
            amount: amount.amount.to_string(),
        }),
        mint_to_address,
    }
    .into()
}

pub fn burn(sender: String, amount: Coin, burn_from_address: String) -> CosmosMsg {
    MsgBurn {
        sender,
        amount: Some(osmosis_std::types::cosmos::base::v1beta1::Coin {
            denom: amount.denom,
            amount: amount.amount.to_string(),
        }),
        burn_from_address,
    }
    .into()
}
