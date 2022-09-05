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
#[discriminator()]
pub struct SetValidatorScoreData {
    pub index: u32,
    pub validator_vote: Pubkey,
    pub score: u32,
}

#[derive(MarinadeInstructionAccounts)]
#[ownerid(crate::ID)]
pub struct SetValidatorScoreAccounts {
    #[account(mut)]
    pub state: Pubkey,
    #[account(signer)]
    pub manager_authority: Pubkey,
    #[account(mut)]
    pub validator_list: Pubkey,
}
