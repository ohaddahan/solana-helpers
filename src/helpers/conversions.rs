use num_bigint::BigInt;

pub struct Conversions;

impl Conversions {
    pub fn number_array_to_bigint(arr: &[u8], reverse: bool) -> BigInt {
        let array = if reverse {
            let mut reversed = arr.to_vec();
            reversed.reverse();
            reversed
        } else {
            arr.to_vec()
        };

        array
            .iter()
            .fold(BigInt::from(0), |acc, &num| (acc << 8) | BigInt::from(num))
    }

    pub fn vec_to_array(vec: &[u8]) -> [u8; 32] {
        let mut array = [0u8; 32]; // Initialize with zeros
        let len = vec.len().min(32); // Use the minimum of 32 or vector length
        array[..len].copy_from_slice(&vec[..len]);
        array
    }

    pub fn change_endianness(bytes: &[u8]) -> Vec<u8> {
        let mut vec = Vec::new();
        for b in bytes.chunks(32) {
            for byte in b.iter().rev() {
                vec.push(*byte);
            }
        }
        vec
    }
}
