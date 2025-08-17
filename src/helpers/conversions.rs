use num_bigint::BigInt;

/// Provides utilities for converting between different data formats.
pub struct Conversions;

impl Conversions {
    /// Converts a byte array to a `BigInt`.
    ///
    /// # Arguments
    ///
    /// * `arr` - The byte array to convert
    /// * `reverse` - Whether to reverse the byte order before conversion
    ///
    /// # Returns
    ///
    /// A `BigInt` representation of the byte array
    #[must_use]
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

    /// Converts a vector to a fixed-size 32-byte array.
    ///
    /// If the vector is shorter than 32 bytes, the remaining bytes are zero-filled.
    /// If longer, only the first 32 bytes are used.
    ///
    /// # Arguments
    ///
    /// * `vec` - The byte vector to convert
    ///
    /// # Returns
    ///
    /// A 32-byte array representation of the vector
    #[must_use]
    pub fn vec_to_array(vec: &[u8]) -> [u8; 32] {
        let mut array = [0u8; 32]; // Initialize with zeros
        let len = vec.len().min(32); // Use the minimum of 32 or vector length
        array[..len].copy_from_slice(&vec[..len]);
        array
    }

    /// Changes the endianness of bytes by reversing every 32-byte chunk.
    ///
    /// # Arguments
    ///
    /// * `bytes` - The input bytes to reverse
    ///
    /// # Returns
    ///
    /// A new vector with reversed byte order within each 32-byte chunk
    #[must_use]
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
