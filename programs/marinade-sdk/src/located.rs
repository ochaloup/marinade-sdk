use solana_program::pubkey::Pubkey;

/* Parsed account together with location key concept.
 * For example ProgramAccount or CpiAccount from anchor.
 */
pub trait Located<T> {
    fn as_ref(&self) -> &T;
    fn as_mut(&mut self) -> &mut T;
    fn key(&self) -> Pubkey;
}
