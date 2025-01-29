use cosmwasm_std::{Binary, Coin, CosmosMsg};
use initia_proto::miniwasm::tokenfactory::v1::{MsgBurn, MsgCreateDenom, MsgMint};
use initia_proto::traits::MessageExt;

pub fn create_denom(sender: String, subdenom: String) -> CosmosMsg {
    let bytes = MsgCreateDenom { sender, subdenom }.to_bytes().unwrap();
    let data = Binary::from(bytes);
    CosmosMsg::Stargate {
        type_url: "/miniwasm.tokenfactory.v1.MsgCreateDenom".to_string(),
        value: data,
    }
}

pub fn mint(sender: String, amount: Coin, mint_to_address: String) -> CosmosMsg {
    let bytes = MsgMint {
        sender,
        amount: Some(initia_proto::cosmos::base::v1beta1::Coin {
            denom: amount.denom,
            amount: amount.amount.to_string(),
        }),
        mint_to_address,
    }
    .to_bytes()
    .unwrap();

    let data = Binary::from(bytes);
    CosmosMsg::Stargate {
        type_url: "/miniwasm.tokenfactory.v1.MsgMint".to_string(),
        value: data,
    }
}

pub fn burn(sender: String, amount: Coin, burn_from_address: String) -> CosmosMsg {
    if burn_from_address != sender {
        panic!("on miniwasm burn from address must be the sender")
    }

    let bytes = MsgBurn {
        sender,
        amount: Some(initia_proto::cosmos::base::v1beta1::Coin {
            denom: amount.denom,
            amount: amount.amount.to_string(),
        }),
    }
    .to_bytes()
    .unwrap();
    let data = Binary::from(bytes);
    CosmosMsg::Stargate {
        type_url: "/miniwasm.tokenfactory.v1.MsgBurn".to_string(),
        value: data,
    }
}
