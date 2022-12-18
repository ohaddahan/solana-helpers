use arrayref::{array_mut_ref, array_ref, array_refs, mut_array_refs};
use solana_program::program_error::ProgramError;
use solana_program::program_pack::{Pack, Sealed};

pub struct PDA {
    pub bump: u8,
    pub uuid: String
}

impl PDA {
    const SIZE: usize =
    1 + // bump
    16  // String
    ;
}

impl Sealed for PDA {}

impl Pack for PDA {
    const LEN: usize = PDA::SIZE;

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, PDA::LEN];
        let (
            bump_dst,
            uuid_dst,
        ) = mut_array_refs![dst, 1, 16];

        bump_dst[0] = self.bump;
        uuid_dst.copy_from_slice(self.uuid.as_bytes());
    }

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, PDA::LEN];
        let (
            bump,
            uuid,
        ) = array_refs![src, 1, 16];

        Ok(PDA {
            bump: bump[0],
            uuid: String::from_utf8(uuid.to_vec()).unwrap(),
        })
    }
}