

# QuantumVault – Member 4 CLI

## Week 1 – Day 5: Module Interfaces and Documentation

### Features completed

- Organized CLI command modules.
- Separated initialization and status command logic.
- Defined the cryptographic engine interface.
- Defined the key-management interface.
- Defined the filesystem mount/unmount interface.
- Added a basic configuration model.
- Added CLI argument parsing tests.
- Documented installation and usage.

## Requirements

- Rust toolchain
- Cargo
- Git

## Installation

Clone the repository:

```powershell
git clone https://github.com/TirthaRaj-dev/QuantumVault.git
cd QuantumVault
```

Switch to the Member 4 branch:

```powershell
git checkout member-4-cli-integration
```

Build the application:

```powershell
cargo build
```

Run tests:

```powershell
cargo test
```

## Usage

Display help:

```powershell
cargo run -- --help
```

Display version:

```powershell
cargo run -- --version
```

Initialize the default vault:

```powershell
cargo run -- init
```

Initialize a custom vault:

```powershell
cargo run -- init --path test-vault
```

Check vault status:

```powershell
cargo run -- status
```

Mount and unmount command interfaces:

```powershell
cargo run -- mount
cargo run -- unmount
```

## Integration Architecture

### Member 1 – Cryptographic Engine

File: `src/integration/crypto.rs`

Expected responsibilities:
- File encryption and decryption.
- Cryptographic operations.
- Secure integration with the CLI.

### Member 2 – Filesystem / FUSE

File: `src/integration/filesystem.rs`

Expected responsibilities:
- Mount the secure filesystem.
- Unmount the secure filesystem.
- Handle filesystem operations.

### Member 3 – Key Management

File: `src/integration/key_manager.rs`

Expected responsibilities:
- Key generation and management.
- Master password handling.
- Secure key derivation and storage.
- Lock and unlock operations.

### Member 4 – CLI Integration

Responsibilities:
- Command-line interface.
- Vault initialization.
- Command dispatch.
- Integration between the other modules.
- Error handling and testing.

## Current Limitations

The integration files currently define interfaces only.
Actual cryptographic operations, key management, and FUSE
mounting will be connected when the respective modules
are provided by the other team members.

The initial vault configuration does not contain plaintext
master passwords or cryptographic keys.
