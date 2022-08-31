#[cfg(test)]
mod tests {
    use borsh::{BorshDeserialize, BorshSerialize};
    use is_trait::is_trait;
    use marinade_sdk_macro::{MarinadeInstructionAccounts, MarinadeInstructionData};
    use solana_program::pubkey::Pubkey;

    #[test]
    fn test_instruction_data() {
        #[derive(MarinadeInstructionData, BorshSerialize, BorshDeserialize)]
        #[discriminator([1,2,3,4,5,6,7,8])]
        struct SimpleTestData {
            lamports: u64,
        }

        assert!(is_trait!(SimpleTestData, micro_anchor::Discriminator));
        assert!(is_trait!(SimpleTestData, micro_anchor::InstructionData));
    }

    #[test]
    fn test_account_infos<'info>() {
        // SimpleTestData struct is required ->
        //   as the macro MarinadeInstructionAccounts depends on existence of it
        #[derive(BorshSerialize, BorshDeserialize)]
        pub struct SimpleTestData {
            lamports: u64,
        }
        impl micro_anchor::InstructionData for SimpleTestData {}
        impl micro_anchor::Discriminator for SimpleTestData {
            const DISCRIMINATOR: [u8; 8] = ([1, 2, 3, 4, 5, 6, 7, 8]);
        }

        // for being able to work with micro anchor to_account_metas()
        use micro_anchor::ToAccountMetas;

        #[derive(MarinadeInstructionAccounts)]
        #[ownerid(solana_program::bpf_loader::ID)]
        pub struct SimpleTestAccounts {
            #[account(mut)]
            pub marinade: Pubkey,
        }

        let simple_test_accounts = SimpleTestAccounts {
            marinade: Pubkey::try_from("MarBmsSgKXdrN1egZf5sqe1TMai9K1rChYNDJgjq7aD").unwrap(),
        };

        assert!(is_trait!(SimpleTestAccounts, micro_anchor::Owner));
        assert!(is_trait!(SimpleTestAccountInfos, micro_anchor::Owner));

        let account_metas: Vec<solana_program::instruction::AccountMeta> =
            simple_test_accounts.to_account_metas();
        assert_eq!(account_metas.len(), 1);
    }
}
