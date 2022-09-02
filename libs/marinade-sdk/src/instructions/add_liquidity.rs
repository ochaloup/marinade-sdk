use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

#[derive(
    MarinadeInstructionData,
    Clone,
    Copy,
    Debug,
    Default,
    PartialEq,
    BorshSerialize,
    BorshDeserialize,
)]
#[discriminator([181, 157, 89, 67, 143, 182, 52, 72])]
pub struct AddLiquidityData {
    pub lamports: u64,
}

#[derive(MarinadeInstructionAccounts)]
#[ownerid(crate::ID)]
pub struct AddLiquidityAccounts {
    #[account(mut)]
    pub marinade: Pubkey, // state
    #[account(mut)]
    pub lp_mint: Pubkey,
    pub lp_mint_authority: Pubkey,
    #[account(mut)]
    pub liq_pool_msol_leg: Pubkey,
    pub liq_pool_sol_leg_pda: Pubkey,
    #[account(mut, signer)]
    pub transfer_from: Pubkey,
    #[account(mut)]
    pub mint_to: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
}
