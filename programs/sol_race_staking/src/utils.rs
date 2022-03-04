use crate::account::{PoolAccount, StakingAccount};
use crate::ErrorCode;
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, TokenAccount};
use mpl_token_metadata::state::{Metadata, EDITION, PREFIX};
use std::ops::Deref;

pub fn verify_nft<'info>(
  pool_account: &PoolAccount,
  nft_token_account: &Account<'info, TokenAccount>,
  nft_metadata_account: &AccountInfo<'info>,
  nft_mint: &Account<'info, Mint>,
  creature_edition: &AccountInfo<'info>,
  token_metadata_program: &AccountInfo<'info>,
) -> ProgramResult {
  let master_edition_seed = &[
    PREFIX.as_bytes(),
    token_metadata_program.key.as_ref(),
    nft_token_account.mint.as_ref(),
    EDITION.as_bytes(),
  ];

  let (master_edition_key, _bump_seed) =
    Pubkey::find_program_address(master_edition_seed, token_metadata_program.key);

  assert_eq!(master_edition_key, creature_edition.key());

  if creature_edition.data_is_empty() {
    return Err(ErrorCode::NotInitialize.into());
  }

  let nft_mint_account_pubkey = nft_mint.key();

  // Seeds for PDS
  let metadata_seed = &[
    "metadata".as_bytes(),
    token_metadata_program.key.as_ref(),
    nft_mint_account_pubkey.as_ref(),
  ];

  let (metadata_derived_key, _bump_seed) =
    Pubkey::find_program_address(metadata_seed, token_metadata_program.key);

  // Check that the derived key is the current metadata account
  assert_eq!(metadata_derived_key, nft_metadata_account.key());

  if nft_metadata_account.data_is_empty() {
    return Err(ErrorCode::NotInitialize.into());
  }

  let metadata_full_account = &mut Metadata::from_account_info(nft_metadata_account)?;

  let full_metadata_clone = metadata_full_account.clone();

  assert_eq!(
    full_metadata_clone.data.creators.as_ref().unwrap()[0].address,
    pool_account.garage_creator
  );

  if !full_metadata_clone.data.creators.unwrap()[0].verified {
    return Err(ErrorCode::NotVerified.into());
  }

  Ok(())
}

pub fn compute_reward(pool_account: &mut PoolAccount, current_time: i64) {
  if pool_account.total_staked == 0 {
    pool_account.last_distributed = current_time;
    return;
  }

  let mut distributed_amount: u128 = 0;
  if pool_account.start_time <= current_time
    && pool_account.end_time >= pool_account.last_distributed
  {
    let time = pool_account.end_time - pool_account.start_time;

    let distributed_amount_per_sec = (pool_account.total_distribution as u128)
      .checked_div(time as u128)
      .unwrap();

    let passed_time = std::cmp::min(pool_account.end_time, current_time)
      - std::cmp::max(pool_account.start_time, pool_account.last_distributed);

    distributed_amount = distributed_amount_per_sec
      .checked_mul(passed_time as u128)
      .unwrap();
  }

  pool_account.last_distributed = current_time;
  pool_account.global_reward_index += (distributed_amount as u128)
    .checked_div(pool_account.total_staked as u128)
    .unwrap() as u64;
}

pub fn compute_staker_reward(staking_account: &mut StakingAccount, pool_account: &PoolAccount) {
  let bond_amount: u128 = match staking_account.is_bond {
    true => 1,
    false => 0,
  };

  let a = (pool_account.global_reward_index as u128)
    .checked_mul(bond_amount)
    .unwrap();
  let b = (staking_account.reward_index as u128)
    .checked_mul(bond_amount)
    .unwrap();

  let pending_reward = a.checked_sub(b).unwrap();

  staking_account.reward_index = pool_account.global_reward_index;

  staking_account.pending_reward += pending_reward as u64;
}

pub fn increase_bond_amount(staking_account: &mut StakingAccount, pool_account: &mut PoolAccount) {
  pool_account.total_staked += 1;
  staking_account.is_bond = true;
}

pub fn decrease_bond_amount(staking_account: &mut StakingAccount, pool_account: &mut PoolAccount) {
  pool_account.total_staked += 1;
  staking_account.is_bond = false;
}

pub trait TrimAsciiWhitespace {
  /// Trim ascii whitespace (based on `is_ascii_whitespace()`) from the
  /// start and end of a slice.
  fn trim_ascii_whitespace(&self) -> &[u8];
}

impl<T: Deref<Target = [u8]>> TrimAsciiWhitespace for T {
  fn trim_ascii_whitespace(&self) -> &[u8] {
    let from = match self.iter().position(|x| !x.is_ascii_whitespace()) {
      Some(i) => i,
      None => return &self[0..0],
    };
    let to = self.iter().rposition(|x| !x.is_ascii_whitespace()).unwrap();
    &self[from..=to]
  }
}
