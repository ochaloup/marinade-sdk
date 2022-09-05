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
#[discriminator([97, 167, 144, 107, 117, 190, 128, 36])]
pub struct OrderUnstakeData {
    pub msol_amount: u64,
}

#[derive(MarinadeInstructionAccounts)]
#[ownerid(crate::ID)]
pub struct OrderUnstakeAccounts {
    #[account(mut)]
    pub state: Pubkey,
    #[account(mut)]
    pub msol_mint: Pubkey,
    #[account(mut)]
    pub burn_msol_from: Pubkey,
    #[account(signer)]
    pub burn_msol_authority: Pubkey,
    pub new_ticket_account: Pubkey,
    pub clock: Pubkey,
    pub rent: Pubkey,
    pub token_program: Pubkey,
}
