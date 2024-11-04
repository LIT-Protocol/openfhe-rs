use crate::fhe_core::MAX_MODULUS_SIZE;

/// noise flooding distribution parameter for distributed decryption in threshold FHE
pub const MP_SD: usize = 1048576;
/// noise flooding distribution parameter for fixed 20 bits noise multihop PRE
pub const PRE_SD: usize = 1048576;
/// number of additional moduli in NOISE_FLOODING_MULTIPARTY mode
pub const NUM_MODULI_MULTIPARTY: usize = 2;
/// modulus size for additional moduli in NOISE_FLOODING_MULTIPARTY mode
pub const MULTIPART_MOD_SIZE: usize = if cfg!(target_pointer_width = "64") {
    60
} else {
    MAX_MODULUS_SIZE
};
