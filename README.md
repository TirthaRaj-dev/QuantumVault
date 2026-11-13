
## Week 1 – Day 2: CLI Architecture

Implemented the initial command-line architecture using Rust's `clap` crate.

### Features
- Added command-line argument parsing.
- Created a separate CLI module.
- Created a commands module.
- Defined `status` and `init` subcommands.
- Enabled automatic help and version information.
- Added unit tests for valid and invalid commands.

### Supported command interfaces

```text
quantumvault --help
quantumvault --version
quantumvault status
quantumvault init
```

**Current status:** CLI structure and argument parsing implemented. Actual vault operations are planned for subsequent development days.
