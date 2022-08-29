use borsh::{BorshDeserialize, BorshSerialize};
use micro_anchor::{Discriminator, InstructionData, Owner, ToAccountInfos, ToAccountMetas};
use solana_program::{account_info::AccountInfo, instruction::AccountMeta, pubkey::Pubkey};



#[derive(Clone, Copy, Debug, Default, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct AddLiquidityData {
    pub lamports: Option<u64>,
}

impl Discriminator for AddLiquidityData {
    const DISCRIMINATOR: [u8; 8] = [181, 157, 89, 67, 143, 182, 52, 72];
}

impl InstructionData for AddLiquidityData {}

impl AddLiquidityData {
    pub fn with_lamports(mut self, v: u64) -> Self {
        let old = self.lamports.replace(v);
        assert!(old.is_none(), "Parameter lamports was already set");
        assert!(v > 0, "Requesting add liquidity expects a possitive number");  // is such assertion correct here?
        self
    }
}

pub struct AddLiquidityAccounts {
    pub marinade: Pubkey,  // state
    pub lp_mint: Pubkey,
    pub lp_mint_authority: Pubkey,
    pub liq_pool_msol_leg: Pubkey,
    pub liq_pool_sol_leg_pda: Pubkey,
    pub transfer_from: Pubkey,
    pub mint_to: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
}

impl Owner for AddLiquidityAccounts {
    fn owner() -> Pubkey {
        crate::ID
    }
}

impl ToAccountMetas for AddLiquidityAccounts {
    fn to_account_metas(&self) -> Vec<AccountMeta> {
        vec![
            AccountMeta::new(self.marinade, false),
            AccountMeta::new(self.lp_mint, false),
            AccountMeta::new_readonly(self.lp_mint_authority, false),
            AccountMeta::new_readonly(self.liq_pool_msol_leg, false),
            AccountMeta::new(self.liq_pool_sol_leg_pda, false),
            AccountMeta::new(self.transfer_from, true),
            AccountMeta::new(self.mint_to, false),
            AccountMeta::new_readonly(self.system_program, false),
            AccountMeta::new_readonly(self.token_program, false),
        ]
    }

    type Data = AddLiquidityData;
}

pub struct AddLiquidityAccountInfos<'info> {
    pub marinade: AccountInfo<'info>,
    pub lp_mint: AccountInfo<'info>,
    pub lp_mint_authority: AccountInfo<'info>,
    pub liq_pool_msol_leg: AccountInfo<'info>,
    pub liq_pool_sol_leg_pda: AccountInfo<'info>,
    pub transfer_from: AccountInfo<'info>,
    pub mint_to: AccountInfo<'info>,
    pub system_program: AccountInfo<'info>,
    pub token_program: AccountInfo<'info>,
}

impl<'info> Owner for AddLiquidityAccountInfos<'info> {
    fn owner() -> Pubkey {
        crate::ID
    }
}

impl<'info> From<&AddLiquidityAccountInfos<'info>> for AddLiquidityAccounts {
    fn from(
        AddLiquidityAccountInfos {
            marinade,
            lp_mint,
            lp_mint_authority,
            liq_pool_msol_leg,
            liq_pool_sol_leg_pda,
            transfer_from,
            mint_to,
            system_program,
            token_program,
        }: &AddLiquidityAccountInfos<'info>,
    ) -> Self {
        Self {
            marinade: marinade.key.clone(),
            lp_mint: lp_mint.key.clone(),
            lp_mint_authority: lp_mint_authority.key.clone(),
            liq_pool_msol_leg: liq_pool_msol_leg.key.clone(),
            liq_pool_sol_leg_pda: liq_pool_sol_leg_pda.key.clone(),
            transfer_from: transfer_from.key.clone(),
            mint_to: mint_to.key.clone(),
            system_program: system_program.key.clone(),
            token_program: token_program.key.clone(),
        }
    }
}

impl<'info> ToAccountMetas for AddLiquidityAccountInfos<'info> {
    fn to_account_metas(&self) -> Vec<AccountMeta> {
        vec![
            AccountMeta::new(marinade.key.clone(), false),
            AccountMeta::new(lp_mint.key.clone(), false),
            AccountMeta::new_readonly(lp_mint_authority.key.clone(), false),
            AccountMeta::new_readonly(liq_pool_msol_leg.key.clone(), false),
            AccountMeta::new(liq_pool_sol_leg_pda.key.clone(), false),
            AccountMeta::new(transfer_from.key.clone(), true),
            AccountMeta::new(mint_to.key.clone(), false),
            AccountMeta::new_readonly(system_program.key.clone(), false),
            AccountMeta::new_readonly(token_program.key.clone(), false),
        ]
    }
    type Data = AddLiquidityData;
}

impl<'info> ToAccountInfos<'info> for AddLiquidityAccountInfos<'info> {
    fn to_account_infos(&self) -> Vec<AccountInfo<'info>> {
        vec![
            marinade.key.clone(),
            lp_mint.key.clone(),
            lp_mint_authority.key.clone(),
            liq_pool_msol_leg.key.clone(),
            liq_pool_sol_leg_pda.key.clone(),
            transfer_from.key.clone(),
            mint_to.key.clone(),
            system_program.key.clone(),
            token_program.key.clone(),
        ]
    }
}
