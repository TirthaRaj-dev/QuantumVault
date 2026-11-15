/// Basic QuantumVault configuration model.
///
/// This is an in-memory representation of the initial
/// configuration. Serialization will be added when
/// the configuration format is finalized.
#[derive(Debug, Clone)]
pub struct VaultConfig {
    pub application: String,
    pub format_version: u32,
    pub vault_name: String,
    pub encryption_status: String,
    pub key_management_status: String,
}

impl Default for VaultConfig {
    fn default() -> Self {
        Self {
            application: "QuantumVault".to_string(),
            format_version: 1,
            vault_name: "QuantumVault".to_string(),
            encryption_status: "not_configured".to_string(),
            key_management_status: "pending".to_string(),
        }
    }
}
