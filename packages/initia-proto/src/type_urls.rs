//! Registry of type URLs associated with various protobuf types defined in
//! this crate.

// TODO(tarcieri): leverage first-class support for type URLs in prost?
// See: https://github.com/tokio-rs/prost/issues/299

use crate::{cosmos, ibc, initia, traits::TypeUrl};

impl TypeUrl for cosmos::bank::v1beta1::MsgSend {
    const TYPE_URL: &'static str = "/cosmos.bank.v1beta1.MsgSend";
}

impl TypeUrl for cosmos::bank::v1beta1::MsgMultiSend {
    const TYPE_URL: &'static str = "/cosmos.bank.v1beta1.MsgMultiSend";
}
 
impl TypeUrl for cosmos::distribution::v1beta1::MsgSetWithdrawAddress {
    const TYPE_URL: &'static str = "/cosmos.distribution.v1beta1.MsgSetWithdrawAddress";
}

impl TypeUrl for cosmos::distribution::v1beta1::MsgWithdrawDelegatorReward {
    const TYPE_URL: &'static str = "/cosmos.distribution.v1beta1.MsgWithdrawDelegatorReward";
}

impl TypeUrl for cosmos::distribution::v1beta1::MsgWithdrawValidatorCommission {
    const TYPE_URL: &'static str = "/cosmos.distribution.v1beta1.MsgWithdrawValidatorCommission";
}

impl TypeUrl for cosmos::distribution::v1beta1::MsgFundCommunityPool {
    const TYPE_URL: &'static str = "/cosmos.distribution.v1beta1.MsgFundCommunityPool";
}

impl TypeUrl for cosmos::feegrant::v1beta1::MsgGrantAllowance {
    const TYPE_URL: &'static str = "/cosmos.feegrant.v1beta1.MsgGrantAllowance";
}

impl TypeUrl for cosmos::feegrant::v1beta1::MsgRevokeAllowance {
    const TYPE_URL: &'static str = "/cosmos.feegrant.v1beta1.MsgRevokeAllowance";
}

impl TypeUrl for cosmos::feegrant::v1beta1::BasicAllowance {
    const TYPE_URL: &'static str = "/cosmos.feegrant.v1beta1.BasicAllowance";
}

impl TypeUrl for cosmos::feegrant::v1beta1::PeriodicAllowance {
    const TYPE_URL: &'static str = "/cosmos.feegrant.v1beta1.PeriodicAllowance";
}

impl TypeUrl for cosmos::feegrant::v1beta1::AllowedMsgAllowance {
    const TYPE_URL: &'static str = "/cosmos.feegrant.v1beta1.AllowedMsgAllowance";
}

impl TypeUrl for cosmos::staking::v1beta1::MsgDelegate {
    const TYPE_URL: &'static str = "/cosmos.staking.v1beta1.MsgDelegate";
}

impl TypeUrl for cosmos::staking::v1beta1::MsgUndelegate {
    const TYPE_URL: &'static str = "/cosmos.staking.v1beta1.MsgUndelegate";
}

impl TypeUrl for cosmos::staking::v1beta1::MsgBeginRedelegate {
    const TYPE_URL: &'static str = "/cosmos.staking.v1beta1.MsgBeginRedelegate";
}

impl TypeUrl for cosmos::base::abci::v1beta1::MsgData {
    const TYPE_URL: &'static str = "/cosmos.base.v1beta1.abci.MsgData";
}

impl TypeUrl for cosmos::base::abci::v1beta1::TxMsgData {
    const TYPE_URL: &'static str = "/cosmos.base.v1beta1.abci.TxMsgData";
}

impl TypeUrl for cosmos::auth::v1beta1::BaseAccount {
    const TYPE_URL: &'static str = "/cosmos.auth.v1beta1.BaseAccount";
}

impl TypeUrl for cosmos::auth::v1beta1::ModuleAccount {
    const TYPE_URL: &'static str = "/cosmos.auth.v1beta1.ModuleAccount";
}

impl TypeUrl for ibc::applications::transfer::v1::MsgTransfer {
    const TYPE_URL: &'static str = "/ibc.applications.transfer.v1.MsgTransfer";
}

impl TypeUrl for initia::mstaking::v1::MsgCreateValidator{
    const TYPE_URL: &'static str = "/initia.mstaking.v1.MsgCreateValidator";
}

impl TypeUrl for initia::mstaking::v1::MsgEditValidator{
    const TYPE_URL: &'static str = "/initia.mstaking.v1.MsgEditValidator";
}

impl TypeUrl for initia::mstaking::v1::MsgDelegate{
    const TYPE_URL: &'static str = "/initia.mstaking.v1.MsgDelegate";
}

impl TypeUrl for initia::mstaking::v1::MsgBeginRedelegate{
    const TYPE_URL: &'static str = "/initia.mstaking.v1.MsgBeginRedelegate";
}

impl TypeUrl for initia::mstaking::v1::MsgUndelegate{
    const TYPE_URL: &'static str = "/initia.mstaking.v1.MsgUndelegate";
}

impl TypeUrl for initia::r#move::v1::MsgPublish{
    const TYPE_URL: &'static str = "/initia.move.v1.MsgPublish";
}

impl TypeUrl for initia::r#move::v1::MsgExecute{
    const TYPE_URL: &'static str = "/initia.move.v1.MsgExecute";
}

impl TypeUrl for initia::r#move::v1::MsgScript{
    const TYPE_URL: &'static str = "/initia.move.v1.MsgScript";
}

// no msgs for tendermint