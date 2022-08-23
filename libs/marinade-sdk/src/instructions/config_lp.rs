use crate::state::fee::Fee;
use borsh::{BorshDeserialize, BorshSerialize};
use micro_anchor::{Discriminator, InstructionData, Owner, ToAccountInfos, ToAccountMetas};
use solana_program::{account_info::AccountInfo, instruction::AccountMeta, pubkey::Pubkey};

#[derive(Clone, Copy, Debug, Default, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct ConfigLpData {
    pub min_fee: Option<Fee>,
    pub max_fee: Option<Fee>,
    pub liquidity_target: Option<u64>,
    pub treasury_cut: Option<Fee>,
}

impl Discriminator for ConfigLpData {
    const DISCRIMINATOR: [u8; 8] = [10, 24, 168, 119, 86, 48, 225, 17];
}

impl InstructionData for ConfigLpData {}

impl ConfigLpData {
    pub fn with_min_fee(mut self, v: Fee) -> Self {
        let old = self.min_fee.replace(v);
        assert!(old.is_none(), "Min fee was already set");
        self
    }

    pub fn with_max_fee(mut self, v: Fee) -> Self {
        let old = self.max_fee.replace(v);
        assert!(old.is_none(), "Max fee was already set");
        self
    }

    pub fn with_liquidity_target(mut self, v: u64) -> Self {
        let old = self.liquidity_target.replace(v);
        assert!(old.is_none(), "Liquidity target was already set");
        self
    }

    pub fn with_treasury_cut(mut self, v: Fee) -> Self {
        let old = self.treasury_cut.replace(v);
        assert!(old.is_none(), "Treasury cut was already set");
        self
    }
}

pub struct ConfigLpAccounts {
    pub marinade: Pubkey,
    pub admin_authority: Pubkey,
}

impl Owner for ConfigLpAccounts {
    fn owner() -> Pubkey {
        crate::ID
    }
}

impl ToAccountMetas for ConfigLpAccounts {
    fn to_account_metas(&self) -> Vec<AccountMeta> {
        vec![
            AccountMeta::new(self.marinade, false),
            AccountMeta::new_readonly(self.admin_authority, true),
        ]
    }

    type Data = ConfigLpData;
}

pub struct ConfigLpAccountInfos<'info> {
    pub marinade: AccountInfo<'info>,
    pub admin_authority: AccountInfo<'info>,
}

impl<'info> Owner for ConfigLpAccountInfos<'info> {
    fn owner() -> Pubkey {
        crate::ID
    }
}

impl<'info> From<&ConfigLpAccountInfos<'info>> for ConfigLpAccounts {
    fn from(
        ConfigLpAccountInfos {
            marinade,
            admin_authority,
        }: &ConfigLpAccountInfos<'info>,
    ) -> Self {
        Self {
            marinade: marinade.key.clone(),
            admin_authority: admin_authority.key.clone(),
        }
    }
}

impl<'info> ToAccountMetas for ConfigLpAccountInfos<'info> {
    fn to_account_metas(&self) -> Vec<AccountMeta> {
        vec![
            AccountMeta::new(self.marinade.key.clone(), false),
            AccountMeta::new_readonly(self.admin_authority.key.clone(), true),
        ]
    }
    type Data = ConfigLpData;
}

impl<'info> ToAccountInfos<'info> for ConfigLpAccountInfos<'info> {
    fn to_account_infos(&self) -> Vec<AccountInfo<'info>> {
        vec![self.marinade.clone(), self.admin_authority.clone()]
    }
}
