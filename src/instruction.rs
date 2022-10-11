use crate::error::MailError::InvalidInstruction;
use solana_program::program_error::ProgramError;

#[derive(Debug)]
pub enum MailInstruction {
    ///Initialize a new account
    /// 
    /// Accounts expected
    /// 
    /// 1. `[Writable]` The AccountInfo of the account to be initialized
    InitAccount,
}

impl MailInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (tag, rest) = input.split_first().ok_or(InvalidInstruction)?;

        OK(if let 0 = tag {
            Self::InitAccount
        } else {
            return Err(InvalidInstruction.into())
        })
    }
}