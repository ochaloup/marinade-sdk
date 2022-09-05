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
#[discriminator([25, 96, 211, 155, 161, 14, 168, 188])]
pub struct RemoveValidatorData {
    pub index: u32,
    pub validator_vote: Pubkey,
}

#[derive(MarinadeInstructionAccounts)]
#[ownerid(crate::ID)]
pub struct RemoveValidatorAccounts {
    #[account(mut)]
    pub state: Pubkey,
    #[account(signer)]
    pub manager_authority: Pubkey,
    #[account(mut)]
    pub validator_list: Pubkey,
    #[account(mut)]
    pub duplication_flag: Pubkey,
    #[account(mut)]
    pub operational_sol_account: Pubkey,
}
