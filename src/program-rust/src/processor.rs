//! Program state processor

use {
    crate::{
        error::RecordError,
        instruction::GreetInstruction,
        state::{GreetingData},
    },
    borsh::{BorshDeserialize, BorshSerialize},
    solana_program::{
        account_info::{next_account_info, AccountInfo},
        entrypoint::ProgramResult,
        msg,
        program_error::ProgramError,
        program_pack::IsInitialized,
        pubkey::Pubkey,
    },
};

fn check_authority(authority_info: &AccountInfo, expected_authority: &Pubkey) -> ProgramResult {
    if expected_authority != authority_info.key {
        msg!("Incorrect record authority provided");
        return Err(RecordError::IncorrectAuthority.into());
    }
    if !authority_info.is_signer {
        msg!("Record authority signature missing");
        return Err(ProgramError::MissingRequiredSignature);
    }
    Ok(())
}

/// Instruction processor
pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    input: &[u8],
) -> ProgramResult {
    let instruction = GreetInstruction::try_from_slice(input)?;
    let account_info_iter = &mut accounts.iter();

    match instruction {
        GreetInstruction::Initialize => {
            msg!("GreetInstruction::Initialize");

            let account_info = next_account_info(account_info_iter)?;
            let authority_info = next_account_info(account_info_iter)?;

            let mut account_data = GreetingData::try_from_slice(*account_info.data.borrow())?;
            if account_data.is_initialized() {
                msg!("Record account already initialized");
                return Err(ProgramError::AccountAlreadyInitialized);
            }

            account_data.authority = *authority_info.key;
            account_data.version = GreetingData::CURRENT_VERSION;
            account_data
                .serialize(&mut *account_info.data.borrow_mut())
                .map_err(|e| e.into())
        }

        GreetInstruction::Greet { count } => {
            msg!("GreetInstruction::Greet");
            let account_info = next_account_info(account_info_iter)?;
            let authority_info = next_account_info(account_info_iter)?;
            let mut account_data = GreetingData::try_from_slice(&account_info.data.borrow())?;
            if !account_data.is_initialized() {
                msg!("Record account not initialized");
                return Err(ProgramError::UninitializedAccount);
            }
            check_authority(authority_info, &account_data.authority)?;
            // Increment and store the number of times the account has been greeted
            account_data.counter += count;
            account_data.serialize(&mut &mut account_info.data.borrow_mut()[..])?;
            msg!("Greeted {} time(s)!", account_data.counter);
            Ok(())
        }
    }
}
