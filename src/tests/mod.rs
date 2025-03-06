#[cfg(test)]
mod tests {
    use mollusk_svm::{program, result::Check, Mollusk};

    use solana_sdk::{
        account::{AccountSharedData, WritableAccount},
        instruction::{AccountMeta, Instruction},
        program_option::COption,
        program_pack::Pack,
        pubkey::Pubkey,
    };

    use crate::BondingCurveState;

    #[test]
    fn test_initialize() {
        let program_id = Pubkey::new_from_array(five8_const::decode_32_const(
            "111111111111111111111111111111111111111111",
        ));
        let mut mollusk = Mollusk::new(&program_id, "target/deploy/bonding_curve_pinocchio");
    
        mollusk.add_program(
            &spl_token::id(),
            "src/tests/spl_token-3.5.0",
            &mollusk_svm::program::loader_keys::LOADER_V3,
        );
    
        let (token_program, token_program_account) = (
            spl_token::ID,
            program::create_program_account_loader_v3(&spl_token::ID),
        );
    
        let state_key = Pubkey::new_unique();
        let vtoken_mint = Pubkey::new_unique();
        let vsol_mint = Pubkey::new_unique();    
        let (state_ata, bump) = Pubkey::find_program_address(&[&state_key.to_bytes()], &program_id);
    
        // Initialize the state account with minimum balance for BondingCurveState
        let state_account = AccountSharedData::new(
            mollusk.sysvars.rent.minimum_balance(BondingCurveState::LEN),
            BondingCurveState::LEN,
            &program_id,
        );
    
        // Initialize vtoken mint and vsol mint accounts
        let mut vtoken_mint_account = AccountSharedData::new(
            mollusk.sysvars.rent.minimum_balance(spl_token::state::Mint::LEN),
            spl_token::state::Mint::LEN,
            &spl_token::ID,
        );
        spl_token::state::Mint {
            mint_authority: COption::None,
            supply: 0,
            decimals: 6,
            is_initialized: true,
            freeze_authority: COption::None,
        }
        .pack_into_slice(vtoken_mint_account.data_as_mut_slice());
    
        let mut vsol_mint_account = AccountSharedData::new(
            mollusk.sysvars.rent.minimum_balance(spl_token::state::Mint::LEN),
            spl_token::state::Mint::LEN,
            &spl_token::ID,
        );
        spl_token::state::Mint {
            mint_authority: COption::None,
            supply: 0,
            decimals: 6,
            is_initialized: true,
            freeze_authority: COption::None,
        }
        .pack_into_slice(vsol_mint_account.data_as_mut_slice());
    
        let state_token_account = AccountSharedData::new(
            mollusk.sysvars.rent.minimum_balance(spl_token::state::Account::LEN),
            spl_token::state::Account::LEN,
            &spl_token::ID,
        );
    
        let data = vec![bump];
    
        let instruction = Instruction {
            program_id,
            accounts: vec![
                AccountMeta::new(state_key, false),            // State account for bonding curve
                AccountMeta::new(vtoken_mint, false),          // vToken mint account
                AccountMeta::new(state_ata, false), // State's token account
                AccountMeta::new(vsol_mint, false),            // vSOL mint account
            ],
            data,
        };
    
        // Process instruction and validate
        mollusk.process_and_validate_instruction(
            &instruction,
            &vec![
                (state_key, state_account),                    // State account
                (vtoken_mint, vtoken_mint_account),            // vToken mint
                (state_ata, state_token_account), // State token account
                (vsol_mint, vsol_mint_account),                // vSOL mint
                (token_program, token_program_account),        // Token program account
            ],
            &[Check::success()],
        );
    }

    #[test]
    fn test_buy() {
        let program_id = Pubkey::new_from_array(five8_const::decode_32_const(""));
        let mut mollusk = Mollusk::new(&program_id, "target/deploy/bonding_curve_pinocchio");

        mollusk.add_program(
            &spl_token::id(),
            "src/tests/spl_token-3.5.0",
            &mollusk_svm::program::loader_keys::LOADER_V3,
        );

        let (token_program, token_program_account) = (
            spl_token::ID,
            program::create_program_account_loader_v3(&spl_token::ID),
        );

        let state_key = Pubkey::new_unique();
        let vtoken_mint = Pubkey::new_unique();
        let vsol_mint = Pubkey::new_unique();
        let buyer = Pubkey::new_unique();
        let (state_ata, bump) = Pubkey::find_program_address(&[&state_key.to_bytes()], &program_id);
        let (buyer_ata, _) = Pubkey::find_program_address(&[&buyer.to_bytes(), &vtoken_mint.to_bytes()], &program_id);
        // Initialize the state account with minimum balance for BondingCurveState
        let state_account = AccountSharedData::new(
            mollusk.sysvars.rent.minimum_balance(BondingCurveState::LEN),
            BondingCurveState::LEN,
            &program_id,
        );

        // Initialize vtoken mint and vsol mint accounts
        let mut vtoken_mint_account = AccountSharedData::new(
            mollusk.sysvars.rent.minimum_balance(spl_token::state::Mint::LEN),
            spl_token::state::Mint::LEN,
            &spl_token::ID,
        );

        spl_token::state::Mint {
            mint_authority: COption::None,
            supply: 0,
            decimals: 6,
            is_initialized: true,
            freeze_authority: COption::None,
        }
        .pack_into_slice(vtoken_mint_account.data_as_mut_slice());

        let mut vsol_mint_account = AccountSharedData::new(
            mollusk.sysvars.rent.minimum_balance(spl_token::state::Mint::LEN),
            spl_token::state::Mint::LEN,
            &spl_token::ID,
        );
        spl_token::state::Mint {
            mint_authority: COption::None,
            supply: 0,
            decimals: 6,
            is_initialized: true,
            freeze_authority: COption::None,
        }
        .pack_into_slice(vsol_mint_account.data_as_mut_slice());

        let state_token_account = AccountSharedData::new(
            mollusk.sysvars.rent.minimum_balance(spl_token::state::Account::LEN),
            spl_token::state::Account::LEN,
            &spl_token::ID,
        );

        let buyer_token_account = AccountSharedData::new(
            mollusk.sysvars.rent.minimum_balance(spl_token::state::Account::LEN),
            spl_token::state::Account::LEN,
            &spl_token::ID,
        );


        let data = [
            vec![bump],
            1_000u64.to_le_bytes().to_vec(), //amount
        ].concat();

        let instruction = Instruction {
            program_id,
            accounts: vec![
                AccountMeta::new(state_key, false),            // State account for bonding curve
                AccountMeta::new(buyer, true),                 // Buyer
                AccountMeta::new(buyer_ata, false),            // Buyer's token account
                AccountMeta::new(vtoken_mint, false),          // vToken mint account
                AccountMeta::new(state_ata, false),            // State's token account
                AccountMeta::new(vsol_mint, false),            // vSOL mint account
                AccountMeta::new_readonly(token_program, false),
            ],
            data,
        };

        // Process instruction and validate
        mollusk.process_and_validate_instruction(
            &instruction,
            &vec![
                (state_key, state_account),                    // State account
                (buyer, AccountSharedData::new(1_000_000_000, 0, &buyer)), // Buyer
                (buyer_ata, buyer_token_account),               // Buyer's token account
                (vtoken_mint, vtoken_mint_account),            // vToken mint
                (state_ata, state_token_account),               // State token account
                (vsol_mint, vsol_mint_account),                // vSOL mint
                (token_program, token_program_account),        // Token program account
            ],
            &[Check::success()],
        );


    }

}