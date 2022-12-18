use solana_program::program_error::ProgramError;

pub enum Instructions {
    /// Create a new PDA
    /// Accounts:
    /// 0. `[signer]` The account of the person initializing the PDA
    /// 1. `[writable]` The PDA account
    /// 2. `[]` The system program
    /// 3. `[]` The rent sysvar
    CreatePDA {
        bump: u8,
        uuid: String
    },
    /// Close a PDA
    /// Accounts:
    /// 0. `[signer]` The account of the person initializing the PDA
    /// 1. `[writable]` The PDA account
    /// 2. `[]` The system program
    /// 3. `[]` The rent sysvar
    ClosePDA {
        bump: u8,
        uuid: String
    },
}


impl Instructions {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (tag, rest) = input.split_first().ok_or(ProgramError::InvalidInstructionData)?;
        Ok(match tag {
            0 => {
                // let (bump, uuid)= array_refs![rest, 1, 16];
                let (bump, rest) = rest.split_first().ok_or(ProgramError::InvalidInstructionData)?;
                let (uuid, _rest) = rest.split_at(16);
                let uuid = String::from_utf8(uuid.to_vec()).unwrap();
                Self::CreatePDA {
                    bump: *bump,
                    uuid
                }
            },
            1 => {
                let (bump, rest) = rest.split_first().ok_or(ProgramError::InvalidInstructionData)?;
                let (uuid, _rest) = rest.split_at(16);
                let uuid = String::from_utf8(uuid.to_vec()).unwrap();
                Self::ClosePDA {
                    bump: *bump,
                    uuid
                }
            },
            _ => return Err(ProgramError::InvalidInstructionData),
        })
    }
}