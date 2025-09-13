#[cfg(test)]
mod tests {
    use mollusk_svm::{result::Check, Mollusk};
    use solana_sdk::account::Account;
    use solana_sdk::pubkey::Pubkey;
    use solana_sdk::instruction::{AccountMeta, Instruction};

    // 1111zdCYdbjoJtjfyCvQfRjGSK3sgjxZE5uybQEc3G
    pub const ADMIN: Pubkey = Pubkey::new_from_array([
        0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 
        0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22,
        0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22,
        0x22, 0x22, 0x22, 0x22, 0x00, 0x00, 0x00, 0x00
    ]);
    // pub const ADMIN: Pubkey = Pubkey::new_from_array([0x00, 0x00, 0x00, 0x00, 0x60, 0x98, 0xae, 0xc7, 0x89, 0xe7, 0x15, 0x78, 0xad, 0x43, 0x46, 0x36, 0x71, 0x90, 0xcc, 0x05, 0x0c, 0x8e, 0xb2, 0xe2, 0x3c, 0x91, 0xa7, 0x37, 0x47, 0x64, 0x7a, 0x27]);

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