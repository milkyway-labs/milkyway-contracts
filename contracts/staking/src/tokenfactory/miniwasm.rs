use cosmwasm_std::{Binary, Coin, CosmosMsg, StdError};
use initia_proto::miniwasm::tokenfactory::v1::{MsgBurn, MsgCreateDenom, MsgMint};
use initia_proto::traits::MessageExt;

pub fn create_denom(sender: String, subdenom: String) -> Result<CosmosMsg, StdError> {
    let bytes = MsgCreateDenom { sender, subdenom }
        .to_bytes()
        .map_err(|_| StdError::generic_err("Failed to serialize MsgCreateDenom"))?;
    Ok(CosmosMsg::Stargate {
        type_url: "/miniwasm.tokenfactory.v1.MsgCreateDenom".to_string(),
        value: Binary::from(bytes),
    })
}

pub fn mint(sender: String, amount: Coin, mint_to_address: String) -> Result<CosmosMsg, StdError> {
    let bytes = MsgMint {
        sender,
        amount: Some(initia_proto::cosmos::base::v1beta1::Coin {
            denom: amount.denom,
            amount: amount.amount.to_string(),
        }),
        mint_to_address,
    }
    .to_bytes()
    .map_err(|_| StdError::generic_err("Failed to serialize MsgMint"))?;

    Ok(CosmosMsg::Stargate {
        type_url: "/miniwasm.tokenfactory.v1.MsgMint".to_string(),
        value: Binary::from(bytes),
    })
}

pub fn burn(
    sender: String,
    amount: Coin,
    burn_from_address: String,
) -> Result<CosmosMsg, StdError> {
    if burn_from_address != sender {
        return Err(StdError::generic_err(
            "on miniwasm burn from address must be equal to the sender",
        ));
    }

    let bytes = MsgBurn {
        sender,
        amount: Some(initia_proto::cosmos::base::v1beta1::Coin {
            denom: amount.denom,
            amount: amount.amount.to_string(),
        }),
    }
    .to_bytes()
    .map_err(|_| StdError::generic_err("Failed to serialize MsgBurn"))?;
    Ok(CosmosMsg::Stargate {
        type_url: "/miniwasm.tokenfactory.v1.MsgBurn".to_string(),
        value: Binary::from(bytes),
    })
}
