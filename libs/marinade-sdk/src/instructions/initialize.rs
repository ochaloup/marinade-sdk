use solana_program::{pubkey::Pubkey, account_info::AccountInfo};
use borsh::{BorshDeserialize, BorshSerialize};
use crate::state::fee::Fee;

pub struct InitializeAccountInfos<'info> {
    pub creator_authority: AccountInfo<'info>,
    pub state: AccountInfo<'info>,

    pub reserve_pda: AccountInfo<'info>,
    pub stake_list: AccountInfo<'info>,
    pub validator_list: AccountInfo<'info>,

    pub msol_mint: AccountInfo<'info>,

    pub operational_sol_account: AccountInfo<'info>,

    pub liq_pool: LiqPoolInitializeAccountInfos<'info>,

    pub treasury_msol_account: AccountInfo<'info>,

    pub clock: AccountInfo<'info>,
    pub rent: AccountInfo<'info>,
}

pub struct LiqPoolInitializeAccountInfos<'info> {
    pub lp_mint: AccountInfo<'info>,
    pub sol_leg_pda: AccountInfo<'info>,
    pub msol_leg: AccountInfo<'info>
}

#[derive(Clone, Copy, Debug, Default, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct InitializeData {
    pub admin_authority: Pubkey,
    pub validator_manager_authority: Pubkey,
    pub min_stake: u64,
    pub reward_fee: Fee,

    pub liq_pool: LiqPoolInitializeData,
    pub additional_stake_record_space: u32,
    pub additional_validator_record_space: u32,
    pub slots_for_stake_delta: u64,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct LiqPoolInitializeData {
    pub lp_liquidity_target: u64,
    pub lp_max_fee: Fee,
    pub lp_min_fee: Fee,
    pub lp_treasury_cut: Fee,
}