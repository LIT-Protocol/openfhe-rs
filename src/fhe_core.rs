mod compression_level;
mod decryption_noise_mode;
mod encryption_technique;
mod execution_mode;
mod hash_algorithm;
mod key_switch_technique;
mod large_scaling_factor_constants;
mod math;
mod multiparty_mode;
mod multiplication_technique;
pub mod noise_flooding;
mod pke_scheme_feature;
mod plaintext_encodings;
mod polynomial_format;
mod proxy_reencryption_mode;
mod scaling_technique;
mod utils;

pub use decryption_noise_mode::*;
pub use encryption_technique::*;
pub use execution_mode::*;
pub use hash_algorithm::*;
pub use key_switch_technique::*;
pub use large_scaling_factor_constants::*;
pub use math::*;
pub use multiparty_mode::*;
pub use multiplication_technique::*;
pub use pke_scheme_feature::*;
pub use plaintext_encodings::*;
pub use polynomial_format::*;
pub use proxy_reencryption_mode::*;
pub use scaling_technique::*;
pub use utils::*;

pub const MAX_MODULUS_SIZE: usize = if cfg!(target_pointer_width = "128") {
    121
} else if cfg!(target_pointer_width = "64") {
    60
} else {
    28
};
