use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

/// 8 byte unique identifier for a type.
pub trait Discriminator {
    const DISCRIMINATOR: [u8; 8];
}

/// Defines an address expected to own an account.
pub trait Owner {
    fn owner() -> Pubkey;
}

pub enum AccountDeserializeError {
    DiscriminatorNotFound,
    DiscriminatorMismatch,
    DidNotDeserialize,
}

/// A data structure that can be deserialized and stored into account storage,
/// i.e. an
/// [`AccountInfo`](../solana_program/account_info/struct.AccountInfo.html#structfield.data)'s
/// mutable data slice.
pub trait AccountDeserialize: Sized {
    /// Deserializes previously initialized account data. Should fail for all
    /// uninitialized accounts, where the bytes are zeroed. Implementations
    /// should be unique to a particular account type so that one can never
    /// successfully deserialize the data of one account type into another.
    /// For example, if the SPL token program were to implement this trait,
    /// it should be impossible to deserialize a `Mint` account into a token
    /// `Account`.
    fn try_deserialize(buf: &mut &[u8]) -> Result<Self, AccountDeserializeError> {
        Self::try_deserialize_unchecked(buf)
    }

    /// Deserializes account data without checking the account discriminator.
    /// This should only be used on account initialization, when the bytes of
    /// the account are zeroed.
    fn try_deserialize_unchecked(buf: &mut &[u8]) -> Result<Self, AccountDeserializeError>;
}

impl<T: BorshDeserialize + Discriminator> AccountDeserialize for T {
    fn try_deserialize_unchecked(buf: &mut &[u8]) -> Result<Self, AccountDeserializeError> {
        let mut data: &[u8] = &buf[8..];
        BorshDeserialize::deserialize(&mut data)
            .map_err(|_| AccountDeserializeError::DidNotDeserialize.into())
    }

    fn try_deserialize(buf: &mut &[u8]) -> Result<Self, AccountDeserializeError> {
        if buf.len() < Self::DISCRIMINATOR.len() {
            return Err(AccountDeserializeError::DiscriminatorNotFound.into());
        }
        let given_disc = &buf[..8];
        if &Self::DISCRIMINATOR != given_disc {
            return Err(AccountDeserializeError::DiscriminatorMismatch.into());
        }
        Self::try_deserialize_unchecked(buf)
    }
}

#[cfg(test)]
mod tests {}
