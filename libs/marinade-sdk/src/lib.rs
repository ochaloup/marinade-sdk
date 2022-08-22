pub mod calc;
pub mod checks;
pub mod error;
pub mod fee;
pub mod liq_pool;
pub mod list;
pub mod located;
pub mod stake_system;
pub mod state;
pub mod ticket_account;
pub mod validator_system;

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

/// The static program ID
pub static ID: Pubkey = Pubkey::new_from_array([
    5, 69, 227, 101, 190, 242, 113, 173, 117, 53, 3, 103, 86, 93, 164, 13, 163, 54, 220, 28, 135,
    155, 177, 84, 138, 122, 252, 197, 90, 169, 57, 30,
]); // "MarBmsSgKXdrN1egZf5sqe1TMai9K1rChYNDJgjq7aD"
