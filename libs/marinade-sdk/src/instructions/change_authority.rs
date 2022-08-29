use borsh::{BorshDeserialize, BorshSerialize};
use micro_anchor::{Discriminator, InstructionData, Owner, ToAccountInfos, ToAccountMetas};
use solana_program::{account_info::AccountInfo, instruction::AccountMeta, pubkey::Pubkey};

#[derive(Clone, Copy, Debug, Default, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct ChangeAuthorityData {
    pub admin: Option<Pubkey>,
    pub validator_manager: Option<Pubkey>,
    pub operational_sol_account: Option<Pubkey>,
    pub treasury_msol_account: Option<Pubkey>,
}

impl Discriminator for ChangeAuthorityData {
    const DISCRIMINATOR: [u8; 8] = [50, 106, 66, 104, 99, 118, 145, 88];
}

impl InstructionData for ChangeAuthorityData {}

impl ChangeAuthorityData {
    pub fn with_admin(mut self, v: Pubkey) -> Self {
        let old = self.admin.replace(v);
        assert!(old.is_none(), "Parameter admin pubkey was already set");
        self
    }
    pub fn with_validator_manager(mut self, v: Pubkey) -> Self {
        let old = self.validator_manager.replace(v);
        assert!(
            old.is_none(),
            "Parameter validator manager pubkey was already set"
        );
        self
    }
    pub fn with_operational_sol_account(mut self, v: Pubkey) -> Self {
        let old = self.operational_sol_account.replace(v);
        assert!(
            old.is_none(),
            "Parameter operational sol account pubkey was already set"
        );
        self
    }
    pub fn with_treasury_msol_account(mut self, v: Pubkey) -> Self {
        let old = self.treasury_msol_account.replace(v);
        assert!(
            old.is_none(),
            "Parameter treasury msol account pubkey was already set"
        );
        self
    }
}

pub struct ChangeAuthorityAccounts {
    pub marinade: Pubkey, //state
    pub admin_authority: Pubkey,
}

// could be a different owner then Marinade program defined as crate::ID, could be this a default implementation of trait?
// would not be better to name it as "program_address" or "address"? Me personally have term "owner" associated with owning something (a key, a secret...)?
impl Owner for ChangeAuthorityAccounts {
    fn owner() -> Pubkey {
        crate::ID
    }
}

impl ToAccountMetas for ChangeAuthorityAccounts {
    fn to_account_metas(&self) -> Vec<AccountMeta> {
        vec![
            AccountMeta::new(self.marinade, false),
            AccountMeta::new_readonly(self.admin_authority, true),
        ]
    }

    type Data = ChangeAuthorityData;
}

pub struct ChangeAuthorityAccountInfos<'info> {
    pub marinade: AccountInfo<'info>,
    pub admin_authority: AccountInfo<'info>,
}

impl<'info> Owner for ChangeAuthorityAccountInfos<'info> {
    fn owner() -> Pubkey {
        crate::ID
    }
}

impl<'info> From<&ChangeAuthorityAccountInfos<'info>> for ChangeAuthorityAccounts {
    fn from(
        ChangeAuthorityAccountInfos {
            marinade,
            admin_authority,
        }: &ChangeAuthorityAccountInfos<'info>,
    ) -> Self {
        Self {
            marinade: marinade.key.clone(),
            admin_authority: admin_authority.key.clone(),
        }
    }
}

impl<'info> ToAccountMetas for ChangeAuthorityAccountInfos<'info> {
    fn to_account_metas(&self) -> Vec<AccountMeta> {
        vec![
            AccountMeta::new(self.marinade.key.clone(), false),
            AccountMeta::new_readonly(self.admin_authority.key.clone(), true),
        ]
    }
    type Data = ChangeAuthorityData;
}

impl<'info> ToAccountInfos<'info> for ChangeAuthorityAccountInfos<'info> {
    fn to_account_infos(&self) -> Vec<AccountInfo<'info>> {
        vec![self.marinade.clone(), self.admin_authority.clone()]
    }
}
