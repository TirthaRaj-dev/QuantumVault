use std::io;
use std::path::Path;

/// Interface for the cryptographic engine.
///
/// Actual encryption and decryption will be connected
/// when the crypto module is provided by Member 1.
pub trait CryptoEngine {
    fn encrypt_file(&self, source: &Path, destination: &Path) -> io::Result<()>;

    fn decrypt_file(&self, source: &Path, destination: &Path) -> io::Result<()>;
}
