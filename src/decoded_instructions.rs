use serde_derive::{Deserialize, Serialize};
use serde::de;

use crate::errors::ErrorCode;

#[derive(Debug, PartialEq, Eq)]
pub enum DecodedWhirlpoolInstruction {
  AdminIncreaseLiquidity(DecodedAdminIncreaseLiquidity),
  CloseBundledPosition(DecodedCloseBundledPosition),
  ClosePosition(DecodedClosePosition),
  CollectFees(DecodedCollectFees),
  CollectProtocolFees(DecodedCollectProtocolFees),
  CollectReward(DecodedCollectReward),
  DecreaseLiquidity(DecodedDecreaseLiquidity),
  DeletePositionBundle(DecodedDeletePositionBundle),
  IncreaseLiquidity(DecodedIncreaseLiquidity),
  InitializeConfig(DecodedInitializeConfig),
  InitializeFeeTier(DecodedInitializeFeeTier),
  InitializePool(DecodedInitializePool),
  InitializePositionBundle(DecodedInitializePositionBundle),
  InitializePositionBundleWithMetadata(DecodedInitializePositionBundleWithMetadata),
  InitializeReward(DecodedInitializeReward),
  InitializeTickArray(DecodedInitializeTickArray),
  OpenBundledPosition(DecodedOpenBundledPosition),
  OpenPosition(DecodedOpenPosition),
  OpenPositionWithMetadata(DecodedOpenPositionWithMetadata),
  SetCollectProtocolFeesAuthority(DecodedSetCollectProtocolFeesAuthority),
  SetDefaultFeeRate(DecodedSetDefaultFeeRate),
  SetDefaultProtocolFeeRate(DecodedSetDefaultProtocolFeeRate),
  SetFeeAuthority(DecodedSetFeeAuthority),
  SetFeeRate(DecodedSetFeeRate),
  SetProtocolFeeRate(DecodedSetProtocolFeeRate),
  SetRewardAuthority(DecodedSetRewardAuthority),
  SetRewardAuthorityBySuperAuthority(DecodedSetRewardAuthorityBySuperAuthority),
  SetRewardEmissions(DecodedSetRewardEmissions),
  SetRewardEmissionsSuperAuthority(DecodedSetRewardEmissionsSuperAuthority),
  Swap(DecodedSwap),
  TwoHopSwap(DecodedTwoHopSwap),
  UpdateFeesAndRewards(DecodedUpdateFeesAndRewards),
}

pub fn from_json(ix: &String, json: &String) -> Result<DecodedWhirlpoolInstruction, ErrorCode> {
  fn from_str<'de, T>(json: &'de String) -> Result<T, ErrorCode>
  where T: de::Deserialize<'de>,
  {
    serde_json::from_str(json).map_err(|_| ErrorCode::InvalidWhirlpoolInstructionJsonString)
  }

  match ix.as_str() {
    "adminIncreaseLiquidity" => Ok(DecodedWhirlpoolInstruction::AdminIncreaseLiquidity(from_str(&json)?)),
    "closeBundledPosition" => Ok(DecodedWhirlpoolInstruction::CloseBundledPosition(from_str(&json)?)),
    "closePosition" => Ok(DecodedWhirlpoolInstruction::ClosePosition(from_str(&json)?)),
    "collectFees" => Ok(DecodedWhirlpoolInstruction::CollectFees(from_str(&json)?)),
    "collectProtocolFees" => Ok(DecodedWhirlpoolInstruction::CollectProtocolFees(from_str(&json)?)),
    "collectReward" => Ok(DecodedWhirlpoolInstruction::CollectReward(from_str(&json)?)),
    "decreaseLiquidity" => Ok(DecodedWhirlpoolInstruction::DecreaseLiquidity(from_str(&json)?)),
    "deletePositionBundle" => Ok(DecodedWhirlpoolInstruction::DeletePositionBundle(from_str(&json)?)),
    "increaseLiquidity" => Ok(DecodedWhirlpoolInstruction::IncreaseLiquidity(from_str(&json)?)),
    "initializeConfig" => Ok(DecodedWhirlpoolInstruction::InitializeConfig(from_str(&json)?)),
    "initializeFeeTier" => Ok(DecodedWhirlpoolInstruction::InitializeFeeTier(from_str(&json)?)),
    "initializePool" => Ok(DecodedWhirlpoolInstruction::InitializePool(from_str(&json)?)),
    "initializePositionBundle" => Ok(DecodedWhirlpoolInstruction::InitializePositionBundle(from_str(&json)?)),
    "initializePositionBundleWithMetadata" => Ok(DecodedWhirlpoolInstruction::InitializePositionBundleWithMetadata(from_str(&json)?)),
    "initializeReward" => Ok(DecodedWhirlpoolInstruction::InitializeReward(from_str(&json)?)),
    "initializeTickArray" => Ok(DecodedWhirlpoolInstruction::InitializeTickArray(from_str(&json)?)),
    "openBundledPosition" => Ok(DecodedWhirlpoolInstruction::OpenBundledPosition(from_str(&json)?)),
    "openPosition" => Ok(DecodedWhirlpoolInstruction::OpenPosition(from_str(&json)?)),
    "openPositionWithMetadata" => Ok(DecodedWhirlpoolInstruction::OpenPositionWithMetadata(from_str(&json)?)),
    "setCollectProtocolFeesAuthority" => Ok(DecodedWhirlpoolInstruction::SetCollectProtocolFeesAuthority(from_str(&json)?)),
    "setDefaultFeeRate" => Ok(DecodedWhirlpoolInstruction::SetDefaultFeeRate(from_str(&json)?)),
    "setDefaultProtocolFeeRate" => Ok(DecodedWhirlpoolInstruction::SetDefaultProtocolFeeRate(from_str(&json)?)),
    "setFeeAuthority" => Ok(DecodedWhirlpoolInstruction::SetFeeAuthority(from_str(&json)?)),
    "setFeeRate" => Ok(DecodedWhirlpoolInstruction::SetFeeRate(from_str(&json)?)),
    "setProtocolFeeRate" => Ok(DecodedWhirlpoolInstruction::SetProtocolFeeRate(from_str(&json)?)),
    "setRewardAuthority" => Ok(DecodedWhirlpoolInstruction::SetRewardAuthority(from_str(&json)?)),
    "setRewardAuthorityBySuperAuthority" => Ok(DecodedWhirlpoolInstruction::SetRewardAuthorityBySuperAuthority(from_str(&json)?)),
    "setRewardEmissions" => Ok(DecodedWhirlpoolInstruction::SetRewardEmissions(from_str(&json)?)),
    "setRewardEmissionsSuperAuthority" => Ok(DecodedWhirlpoolInstruction::SetRewardEmissionsSuperAuthority(from_str(&json)?)),
    "swap" => Ok(DecodedWhirlpoolInstruction::Swap(from_str(&json)?)),
    "twoHopSwap" => Ok(DecodedWhirlpoolInstruction::TwoHopSwap(from_str(&json)?)),
    "updateFeesAndRewards" => Ok(DecodedWhirlpoolInstruction::UpdateFeesAndRewards(from_str(&json)?)),
    _ => Err(ErrorCode::UnknownWhirlpoolInstruction(ix.to_string())),
  }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DecodedAdminIncreaseLiquidity {
  pub data_liquidity: u128,
  pub key_whirlpools_config: String,
  pub key_whirlpool: String,
  pub key_authority: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DecodedCloseBundledPosition {
  pub data_bundle_index: u16,
  pub key_bundled_position: String,
  pub key_position_bundle: String,
  pub key_position_bundle_token_account: String,
  pub key_position_bundle_authority: String,
  pub key_receiver: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DecodedClosePosition {
  pub key_position_authority: String,
  pub key_receiver: String,
  pub key_position: String,
  pub key_position_mint: String,
  pub key_position_token_account: String,
  pub key_token_program: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DecodedCollectFees {
  pub key_whirlpool: String,
  pub key_position_authority: String,
  pub key_position: String,
  pub key_position_token_account: String,
  pub key_token_owner_account_a: String,
  pub key_token_vault_a: String,
  pub key_token_owner_account_b: String,
  pub key_token_vault_b: String,
  pub key_token_program: String,
  pub transfer_amount0: u64,
  pub transfer_amount1: u64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DecodedCollectProtocolFees {
  pub key_whirlpools_config: String,
  pub key_whirlpool: String,
  pub key_collect_protocol_fees_authority: String,
  pub key_token_vault_a: String,
  pub key_token_vault_b: String,
  pub key_token_destination_a: String,
  pub key_token_destination_b: String,
  pub key_token_program: String,
  pub transfer_amount0: u64,
  pub transfer_amount1: u64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DecodedCollectReward {
  pub data_reward_index: u8,
  pub key_whirlpool: String,
  pub key_position_authority: String,
  pub key_position: String,
  pub key_position_token_account: String,
  pub key_reward_owner_account: String,
  pub key_reward_vault: String,
  pub key_token_program: String,
  pub transfer_amount0: u64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DecodedDecreaseLiquidity {
  pub data_liquidity_amount: u128,
  pub data_token_amount_min_a: u64,
  pub data_token_amount_min_b: u64,
  pub key_whirlpool: String,
  pub key_token_program: String,
  pub key_position_authority: String,
  pub key_position: String,
  pub key_position_token_account: String,
  pub key_token_owner_account_a: String,
  pub key_token_owner_account_b: String,
  pub key_token_vault_a: String,
  pub key_token_vault_b: String,
  pub key_tick_array_lower: String,
  pub key_tick_array_upper: String,
  pub transfer_amount0: u64,
  pub transfer_amount1: u64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DecodedDeletePositionBundle {
  pub key_position_bundle: String,
  pub key_position_bundle_mint: String,
  pub key_position_bundle_token_account: String,
  pub key_position_bundle_owner: String,
  pub key_receiver: String,
  pub key_token_program: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DecodedIncreaseLiquidity {
  pub data_liquidity_amount: u128,
  pub data_token_amount_max_a: u64,
  pub data_token_amount_max_b: u64,
  pub key_whirlpool: String,
  pub key_token_program: String,
  pub key_position_authority: String,
  pub key_position: String,
  pub key_position_token_account: String,
  pub key_token_owner_account_a: String,
  pub key_token_owner_account_b: String,
  pub key_token_vault_a: String,
  pub key_token_vault_b: String,
  pub key_tick_array_lower: String,
  pub key_tick_array_upper: String,
  pub transfer_amount0: u64,
  pub transfer_amount1: u64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DecodedInitializeConfig {
  pub data_default_protocol_fee_rate: u16,
  pub data_fee_authority: String,
  pub data_collect_protocol_fees_authority: String,
  pub data_reward_emissions_super_authority: String,
  pub key_whirlpools_config: String,
  pub key_funder: String,
  pub key_system_program: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DecodedInitializeFeeTier {
  pub data_tick_spacing: u16,
  pub data_default_fee_rate: u16,
  pub key_whirlpools_config: String,
  pub key_fee_tier: String,
  pub key_funder: String,
  pub key_fee_authority: String,
  pub key_system_program: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DecodedInitializePool {
  pub data_tick_spacing: u16,
  pub data_initial_sqrt_price: u128,
  pub key_whirlpools_config: String,
  pub key_token_mint_a: String,
  pub key_token_mint_b: String,
  pub key_funder: String,
  pub key_whirlpool: String,
  pub key_token_vault_a: String,
  pub key_token_vault_b: String,
  pub key_fee_tier: String,
  pub key_token_program: String,
  pub key_system_program: String,
  pub key_rent: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DecodedInitializePositionBundle {
  pub key_position_bundle: String,
  pub key_position_bundle_mint: String,
  pub key_position_bundle_token_account: String,
  pub key_position_bundle_owner: String,
  pub key_funder: String,
  pub key_token_program: String,
  pub key_system_program: String,
  pub key_rent: String,
  pub key_associated_token_program: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DecodedInitializePositionBundleWithMetadata {
  pub key_position_bundle: String,
  pub key_position_bundle_mint: String,
  pub key_position_bundle_metadata: String,
  pub key_position_bundle_token_account: String,
  pub key_position_bundle_owner: String,
  pub key_funder: String,
  pub key_metadata_update_auth: String,
  pub key_token_program: String,
  pub key_system_program: String,
  pub key_rent: String,
  pub key_associated_token_program: String,
  pub key_metadata_program: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DecodedInitializeReward {
  pub data_reward_index: u8,
  pub key_reward_authority: String,
  pub key_funder: String,
  pub key_whirlpool: String,
  pub key_reward_mint: String,
  pub key_reward_vault: String,
  pub key_token_program: String,
  pub key_system_program: String,
  pub key_rent: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DecodedInitializeTickArray {
  pub data_start_tick_index: i32,
  pub key_whirlpool: String,
  pub key_funder: String,
  pub key_tick_array: String,
  pub key_system_program: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DecodedOpenBundledPosition {
  pub data_bundle_index: u16,
  pub data_tick_lower_index: i32,
  pub data_tick_upper_index: i32,
  pub key_bundled_position: String,
  pub key_position_bundle: String,
  pub key_position_bundle_token_account: String,
  pub key_position_bundle_authority: String,
  pub key_whirlpool: String,
  pub key_funder: String,
  pub key_system_program: String,
  pub key_rent: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DecodedOpenPosition {
  pub data_tick_lower_index: i32,
  pub data_tick_upper_index: i32,
  pub key_funder: String,
  pub key_owner: String,
  pub key_position: String,
  pub key_position_mint: String,
  pub key_position_token_account: String,
  pub key_whirlpool: String,
  pub key_token_program: String,
  pub key_system_program: String,
  pub key_rent: String,
  pub key_associated_token_program: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DecodedOpenPositionWithMetadata {
  pub data_tick_lower_index: i32,
  pub data_tick_upper_index: i32,
  pub key_funder: String,
  pub key_owner: String,
  pub key_position: String,
  pub key_position_mint: String,
  pub key_position_metadata_account: String,
  pub key_position_token_account: String,
  pub key_whirlpool: String,
  pub key_token_program: String,
  pub key_system_program: String,
  pub key_rent: String,
  pub key_associated_token_program: String,
  pub key_metadata_program: String,
  pub key_metadata_update_auth: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DecodedSetCollectProtocolFeesAuthority {
  pub key_whirlpools_config: String,
  pub key_collect_protocol_fees_authority: String,
  pub key_new_collect_protocol_fees_authority: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DecodedSetDefaultFeeRate {
  pub data_default_fee_rate: u16,
  pub key_whirlpools_config: String,
  pub key_fee_tier: String,
  pub key_fee_authority: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DecodedSetDefaultProtocolFeeRate {
  pub data_default_protocol_fee_rate: u16,
  pub key_whirlpools_config: String,
  pub key_fee_authority: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DecodedSetFeeAuthority {
  pub key_whirlpools_config: String,
  pub key_fee_authority: String,
  pub key_new_fee_authority: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DecodedSetFeeRate {
  pub data_fee_rate: u16,
  pub key_whirlpools_config: String,
  pub key_whirlpool: String,
  pub key_fee_authority: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DecodedSetProtocolFeeRate {
  pub data_protocol_fee_rate: u16,
  pub key_whirlpools_config: String,
  pub key_whirlpool: String,
  pub key_fee_authority: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DecodedSetRewardAuthority {
  pub data_reward_index: u8,
  pub key_whirlpool: String,
  pub key_reward_authority: String,
  pub key_new_reward_authority: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DecodedSetRewardAuthorityBySuperAuthority {
  pub data_reward_index: u8,
  pub key_whirlpools_config: String,
  pub key_whirlpool: String,
  pub key_reward_emissions_super_authority: String,
  pub key_new_reward_authority: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DecodedSetRewardEmissions {
  pub data_reward_index: u8,
  pub data_emissions_per_second_x64: u128,
  pub key_whirlpool: String,
  pub key_reward_authority: String,
  pub key_reward_vault: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DecodedSetRewardEmissionsSuperAuthority {
  pub key_whirlpools_config: String,
  pub key_reward_emissions_super_authority: String,
  pub key_new_reward_emissions_super_authority: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DecodedSwap {
  pub data_amount: u64,
  pub data_other_amount_threshold: u64,
  pub data_sqrt_price_limit: u128,
  #[serde(deserialize_with = "deserialize_bool")]
  pub data_amount_specified_is_input: bool,
  #[serde(deserialize_with = "deserialize_bool")]
  pub data_a_to_b: bool,
  pub key_token_program: String,
  pub key_token_authority: String,
  pub key_whirlpool: String,
  pub key_token_owner_account_a: String,
  pub key_vault_a: String,
  pub key_token_owner_account_b: String,
  pub key_vault_b: String,
  pub key_tick_array0: String,
  pub key_tick_array1: String,
  pub key_tick_array2: String,
  pub key_oracle: String,
  pub transfer_amount0: u64,
  pub transfer_amount1: u64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DecodedTwoHopSwap {
  pub data_amount: u64,
  pub data_other_amount_threshold: u64,
  #[serde(deserialize_with = "deserialize_bool")]
  pub data_amount_specified_is_input: bool,
  #[serde(deserialize_with = "deserialize_bool")]
  pub data_a_to_b_one: bool,
  #[serde(deserialize_with = "deserialize_bool")]
  pub data_a_to_b_two: bool,
  pub data_sqrt_price_limit_one: u128,
  pub data_sqrt_price_limit_two: u128,
  pub key_token_program: String,
  pub key_token_authority: String,
  pub key_whirlpool_one: String,
  pub key_whirlpool_two: String,
  pub key_token_owner_account_one_a: String,
  pub key_vault_one_a: String,
  pub key_token_owner_account_one_b: String,
  pub key_vault_one_b: String,
  pub key_token_owner_account_two_a: String,
  pub key_vault_two_a: String,
  pub key_token_owner_account_two_b: String,
  pub key_vault_two_b: String,
  pub key_tick_array_one_0: String,
  pub key_tick_array_one_1: String,
  pub key_tick_array_one_2: String,
  pub key_tick_array_two_0: String,
  pub key_tick_array_two_1: String,
  pub key_tick_array_two_2: String,
  pub key_oracle_one: String,
  pub key_oracle_two: String,
  pub transfer_amount_0: u64,
  pub transfer_amount_1: u64,
  pub transfer_amount_2: u64,
  pub transfer_amount_3: u64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DecodedUpdateFeesAndRewards {
  pub key_whirlpool: String,
  pub key_position: String,
  pub key_tick_array_lower: String,
  pub key_tick_array_upper: String,
}


// 0 to false, 1 to true
fn deserialize_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: de::Deserializer<'de>,
{
    let n: u8 = de::Deserialize::deserialize(deserializer)?;
    match n {
        0 => Ok(false),
        1 => Ok(true),
        _ => Err(de::Error::custom("expected 0 or 1")),
    }
}
