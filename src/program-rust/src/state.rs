//! Program state
use {
    borsh::{BorshDeserialize, BorshSchema, BorshSerialize},
    solana_program::{program_pack::IsInitialized, pubkey::Pubkey},
};

/// Define the type of state stored in accounts
#[derive(Clone, Debug, BorshSerialize, BorshDeserialize, BorshSchema, PartialEq)]
pub struct GreetingData {
    /// Struct version, allows for upgrades to the program
    pub version: u8,
    /// number of greetings
    pub counter: u32,
    /// The account allowed to update the data
    pub authority: Pubkey,
}

impl GreetingData {
    /// Version to fill in on new created accounts
    pub const CURRENT_VERSION: u8 = 1;
}

impl IsInitialized for GreetingData {
    /// Is initialized
    fn is_initialized(&self) -> bool {
        self.version == Self::CURRENT_VERSION
    }
}