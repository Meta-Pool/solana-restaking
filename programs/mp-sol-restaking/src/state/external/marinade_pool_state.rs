use anchor_lang::solana_program::{pubkey, pubkey::Pubkey};
use borsh::{BorshDeserialize, BorshSerialize};

pub const MARINADE_MSOL_MINT: Pubkey = pubkey!("mSoLzYCxHdYgdzU16g5QSh3i5K3z3KZK7ytfqcJm7So");
pub const MARINADE_POOL_PROGRAM: Pubkey = pubkey!("MarBmsSgKXdrN1egZf5sqe1TMai9K1rChYNDJgjq7aD");
pub const MARINADE_STATE_ADDRESS: Pubkey = pubkey!("8szGkuLTAux9XMgZ2vtY39jVSowEcpBfFfD8hXSEqdGC");

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
pub struct MarinadeState {
    pub discriminator: [u8; 8],
    pub msol_mint: Pubkey,
    pub admin_authority: Pubkey,
    pub operational_sol_account: Pubkey,
    pub treasury_msol_account: Pubkey,
    pub reserve_bump_seed: u8,
    pub msol_mint_authority_bump_seed: u8,
    pub rent_exempt_for_token_acc: u64,
    pub reward_fee: Fee,
    pub stake_system: StakeSystem,
    pub validator_system: ValidatorSystem,
    pub liq_pool: LiqPool,
    pub available_reserve_balance: u64,
    pub msol_supply: u64,
    pub msol_price: u64,
    pub circulating_ticket_count: u64,
    pub circulating_ticket_balance: u64,
    pub lent_from_reserve: u64,
    pub min_deposit: u64,
    pub min_withdraw: u64,
    pub staking_sol_cap: u64,
    pub emergency_cooling_down: u64,
    pub pause_authority: Pubkey,
    pub paused: bool,
    pub delayed_unstake_fee: FeeCents,
    pub withdraw_stake_account_fee: FeeCents,
    pub withdraw_stake_account_enabled: bool,
    pub last_stake_move_epoch: u64,
    pub stake_moved: u64,
    pub max_stake_moved_per_epoch: Fee,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
pub struct Fee {
    pub basis_points: u32,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
pub struct FeeCents {
    pub bp_cents: u32,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
pub struct LiqPool {
    pub lp_mint: Pubkey,
    pub lp_mint_authority_bump_seed: u8,
    pub sol_leg_bump_seed: u8,
    pub msol_leg_authority_bump_seed: u8,
    pub msol_leg: Pubkey,
    pub lp_liquidity_target: u64,
    pub lp_max_fee: Fee,
    pub lp_min_fee: Fee,
    pub treasury_cut: Fee,
    pub lp_supply: u64,
    pub lent_from_sol_leg: u64,
    pub liquidity_sol_cap: u64,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
pub struct List {
    pub account: Pubkey,
    pub item_size: u32,
    pub count: u32,
    pub reserved1: Pubkey,
    pub reserved2: u32,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
pub struct StakeList {}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
pub struct StakeSystem {
    pub stake_list: List,
    pub delayed_unstake_cooling_down: u64,
    pub stake_deposit_bump_seed: u8,
    pub stake_withdraw_bump_seed: u8,
    pub slots_for_stake_delta: u64,
    pub last_stake_delta_epoch: u64,
    pub min_stake: u64,
    pub extra_stake_delta_runs: u32,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
pub struct ValidatorSystem {
    pub validator_list: List,
    pub manager_authority: Pubkey,
    pub total_validator_score: u32,
    pub total_active_balance: u64,
    pub auto_add_validator_enabled: u8,
}
