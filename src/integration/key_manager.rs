use std::io;

/// Interface for key management.
///
/// Passwords must be handled securely in memory.
/// Never print or store plaintext passwords.
pub trait KeyManager {
    fn initialize_identity(&self) -> io::Result<()>;

    fn unlock(&self, password: &[u8]) -> io::Result<()>;

    fn lock(&self) -> io::Result<()>;
}
