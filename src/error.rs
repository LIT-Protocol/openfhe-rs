use thiserror::Error;

/// Error type for FHE operations
#[derive(Error, Debug)]
pub enum Error {
    /// Invalid hash algorithm
    #[error("Invalid hash algorithm: {0}")]
    InvalidHashAlgorithm(usize),
    /// Invalid polynomial format
    #[error("Invalid polynomial format: {0}")]
    InvalidPolynomialFormat(usize),
    /// Invalid string format
    #[error("String parse error")]
    ParseError(String),
    /// Invalid PKE scheme feature
    #[error("Invalid PKE scheme feature: {0}")]
    InvalidPkeSchemeFeature(usize),
    /// Invalid scaling technique
    #[error("Invalid scaling technique: {0}")]
    InvalidScalingTechnique(usize),
    /// Invalid proxy re-encryption mode
    #[error("Invalid proxy re-encryption mode: {0}")]
    InvalidProxyReEncryptionMode(usize),
    /// Invalid multiparty mode
    #[error("Invalid multiparty mode: {0}")]
    InvalidMultipartyMode(usize),
    /// Invalid execution mode
    #[error("Invalid execution mode: {0}")]
    InvalidExecutionMode(usize),
    /// Invalid decryption noise mode
    #[error("Invalid decryption noise mode: {0}")]
    InvalidDecryptionNoiseMode(usize),
    /// Invalid key switch technique
    #[error("Invalid key switch technique: {0}")]
    InvalidKeySwitchTechnique(usize),
    /// Invalid encryption technique
    #[error("Invalid encryption technique: {0}")]
    InvalidEncryptionTechnique(usize),
    /// Invalid multiplication technique
    #[error("Invalid multiplication technique: {0}")]
    InvalidMultiplicationTechnique(usize),
    /// Invalid plaintext encoding
    #[error("Invalid plaintext encoding: {0}")]
    InvalidPlaintextEncoding(usize),
    /// Invalid large scaling factor constant
    #[error("Invalid large scaling factor constant: {0}")]
    InvalidLargeScalingFactorConstant(usize),
    /// Invalid compression level
    #[error("Invalid compression level: {0}")]
    InvalidCompressionLevel(usize),
}

/// Result type for FHE operations
pub type FheResult<T> = Result<T, Error>;
