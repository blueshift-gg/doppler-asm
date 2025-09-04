#[cfg(test)]
mod tests {
    use mollusk_svm::{result::Check, Mollusk};
    use solana_sdk::account::Account;
    use solana_sdk::pubkey::Pubkey;
    use solana_sdk::instruction::{AccountMeta, Instruction};

    pub const ADMIN: Pubkey = Pubkey::new_from_array([0x22;32]);

    #[test]
    fn oracle_update() {
        let program_id_keypair_bytes = std::fs::read("deploy/doppler-asm-keypair.json").unwrap()
            [..32]
            .try_into()
            .expect("slice with incorrect length");
        let program_id = Pubkey::new_from_array(program_id_keypair_bytes);

        let oracle = Pubkey::new_unique();

        let instruction = Instruction::new_with_bytes(
            program_id,
            &[0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x13, 0x37, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
            vec![
                AccountMeta::new_readonly(ADMIN, true),
                AccountMeta::new(oracle, false),
            ]
        );

        let mollusk = Mollusk::new(&program_id, "deploy/doppler-asm");

        let result = mollusk.process_and_validate_instruction(
            &instruction,
            &[
                (
                    ADMIN,
                    Account::new(1_000_000_000, 0, &Pubkey::default())
                ),
                (
                    oracle,
                    Account::new(1_000_000_000, 16, &program_id_keypair_bytes.into())
                )
            ],
            &[Check::success()]
        );
        assert!(!result.program_result.is_err());
    }
}