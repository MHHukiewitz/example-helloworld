//! Program instructions

use crate::id;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
};

/// Instructions supported by the program
#[derive(Clone, Debug, BorshSerialize, BorshDeserialize, PartialEq)]
pub enum GreetInstruction {
    /// Create a new greet record
    ///
    /// Accounts expected by this instruction:
    ///
    /// 0. `[writable]` Record account, must be uninitialized
    /// 1. `[]` Record authority
    Initialize,

    /// Write to the provided record account
    ///
    /// Accounts expected by this instruction:
    ///
    /// 0. `[writable]` Record account, must be previously initialized
    /// 1. `[signer]` Current record authority
    Greet {
        /// The amount of greetings to add
        count: u32,
    }
}

/// Create a `RecordInstruction::Initialize` instruction
pub fn initialize(record_account: &Pubkey, authority: &Pubkey) -> Instruction {
    Instruction::new_with_borsh(
        id(),
        &GreetInstruction::Initialize,
        vec![
            AccountMeta::new(*record_account, false),
            AccountMeta::new_readonly(*authority, false),
        ],
    )
}

/// Create a `RecordInstruction::Write` instruction
pub fn greet(record_account: &Pubkey, signer: &Pubkey, count: u32) -> Instruction {
    Instruction::new_with_borsh(
        id(),
        &GreetInstruction::Greet { count },
        vec![
            AccountMeta::new(*record_account, false),
            AccountMeta::new_readonly(*signer, true),
        ],
    )
}