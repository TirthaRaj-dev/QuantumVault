use std::io;

/// Interface between the CLI and the key-management module.
// Some methods are reserved for future key-manager integration.
#[allow(dead_code)]
pub trait KeyManager {
    fn initialize_identity(&self) -> io::Result<()>;

    fn unlock(&self, password: &[u8]) -> io::Result<()>;

    fn lock(&self) -> io::Result<()>;

    /// Generate a key for the requested identity.
    ///
    /// The default implementation allows older backends to compile
    /// while they are being updated by Member 3.
    fn generate_key(&self, _identity_name: &str) -> io::Result<()> {
        Err(io::Error::new(
            io::ErrorKind::Unsupported,
            "Key generation is not implemented by this backend.",
        ))
    }

    /// Return the identities known to the key manager.
    fn list_identities(&self) -> io::Result<Vec<String>> {
        Err(io::Error::new(
            io::ErrorKind::Unsupported,
            "Identity listing is not implemented by this backend.",
        ))
    }
}
