use std::io;
use std::path::Path;

/// Interface for filesystem mount operations.
///
/// Actual FUSE operations will be connected
/// when the filesystem module is provided by Member 2.
pub trait VaultFilesystem {
    fn mount(&self, mount_point: &Path) -> io::Result<()>;

    fn unmount(&self, mount_point: &Path) -> io::Result<()>;
}
