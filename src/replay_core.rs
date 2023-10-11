use solana_sdk::pubkey::Pubkey;

use solana_transaction_status::EncodedConfirmedTransactionWithStatusMeta;

use poc_framework::{Environment, LocalEnvironment, LocalEnvironmentBuilder};

use crate::{decoded_instructions::DecodedWhirlpoolInstruction, types::AccountMap};
use crate::util_replay;

use crate::programs;
use crate::replay_instructions;

pub struct WritableAccountSnapshot {
  pub pre_snapshot: AccountMap,
  pub post_snapshot: AccountMap,
}

pub struct ReplayInstructionResult {
  pub transaction_status: EncodedConfirmedTransactionWithStatusMeta,
  pub snapshot: WritableAccountSnapshot,
}

pub struct ReplayInstructionParams<'info, T> {
  pub env_builder: &'info mut LocalEnvironmentBuilder,
  pub decoded_instruction: &'info T,
  pub account_map: &'info AccountMap,
}

const SPL_TOKEN_PROGRAM_ID: Pubkey = solana_program::pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");
const ORCA_WHIRLPOOL_PROGRAM_ID: Pubkey = solana_program::pubkey!("whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc");
const METAPLEX_METADATA_PROGRAM_ID: Pubkey = solana_program::pubkey!("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s");

pub fn replay_whirlpool_instruction(
  instruction: DecodedWhirlpoolInstruction,
  account_map: &AccountMap, // readonly
  clock_unixtimestamp: i64,
  whirlpool_program_so: &[u8],
  token_metadata_program_so: &[u8],
) -> ReplayInstructionResult {
  let mut builder = LocalEnvironment::builder();

  // emulate SYSVAR/Clock
  builder.set_creation_time(clock_unixtimestamp);

  // deploy programs: SPL Token
  // TODO: ATA
  // TODO: receive as params
  util_replay::add_upgradable_program(&mut builder, SPL_TOKEN_PROGRAM_ID, programs::SPL_TOKEN);
  // deploy programs: Orca Whirlpool & Metaplex Token Metadata
  util_replay::add_upgradable_program(&mut builder, ORCA_WHIRLPOOL_PROGRAM_ID, whirlpool_program_so);
  util_replay::add_upgradable_program(&mut builder, METAPLEX_METADATA_PROGRAM_ID, token_metadata_program_so);

  match instruction {
    DecodedWhirlpoolInstruction::Swap(decoded) => replay_instructions::swap::replay(ReplayInstructionParams { env_builder: &mut builder, decoded_instruction: &decoded, account_map: &account_map }),
    DecodedWhirlpoolInstruction::TwoHopSwap(decoded) => replay_instructions::two_hop_swap::replay(ReplayInstructionParams { env_builder: &mut builder, decoded_instruction: &decoded, account_map: &account_map }),
    DecodedWhirlpoolInstruction::UpdateFeesAndRewards(decoded) => replay_instructions::update_fees_and_rewards::replay(ReplayInstructionParams { env_builder: &mut builder, decoded_instruction: &decoded, account_map: &account_map }),
    DecodedWhirlpoolInstruction::CollectFees(decoded) => replay_instructions::collect_fees::replay(ReplayInstructionParams { env_builder: &mut builder, decoded_instruction: &decoded, account_map: &account_map }),
    DecodedWhirlpoolInstruction::CollectReward(decoded) => replay_instructions::collect_reward::replay(ReplayInstructionParams { env_builder: &mut builder, decoded_instruction: &decoded, account_map: &account_map }),
    // IncreaseLiquidity
    // DecreaseLiquidity

    // OpenPosition
    // OpenPositionWithMetadata

    // InitializePositionBundle
    // InitializePositionBundleWithMetadata
    // OpenBundledPosition
    // CloseBundledPosition

    // InitializePool
    // InitializeTickArray

    // InitializeReward
    // SetRewardEmissions
    // ---------------------------------
    // ClosePosition
    // CollectProtocolFees
    // DeletePositionBundle
    // InitializeConfig
    // InitializeFeeTier
    // SetCollectProtocolFeesAuthority
    // SetDefaultFeeRate
    // SetDefaultProtocolFeeRate
    // SetFeeAuthority
    // SetFeeRate
    // SetProtocolFeeRate
    // SetRewardAuthority
    // SetRewardAuthorityBySuperAuthority
    // SetRewardEmissionsSuperAuthority
    // AdminIncreaseLiquidity
    _ => {
      println!("IGNORE INSTRUCTION AT THE MOMENT: {:?}", instruction);
      panic!("instruction not supported yet");
    }
  }
}
