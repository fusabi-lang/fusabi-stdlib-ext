# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.4] - 2025-12-09

### Fixed
- Relaxed clippy warnings for pre-existing code issues in release workflow

## [0.1.3] - 2025-12-08

### Added
- **MCP Server Configuration Builder**:
  - `McpServerConfig` struct for Phage context injection configurations
  - `mcp_server_new()` - Create new MCP server configurations
  - `mcp_server_with_inject()` - Add inject items to configurations
  - `mcp_server_to_json()` - Serialize configurations to JSON
  - `mcp_server_get_name()`, `mcp_server_get_endpoint()`, `mcp_server_get_inject()` - Accessors
  - Full roundtrip conversion between Rust and `fusabi_host::Value`

- **Error Types**:
  - Added `InvalidValue` error variant for type conversion failures
  - Added `Serialization` error variant for JSON serialization errors

- **Documentation Infrastructure**:
  - Added `docs/STRUCTURE.md` describing documentation organization
  - Added `docs/RELEASE.md` with complete release process
  - Created `docs/versions/vNEXT/` for versioned documentation
  - Added documentation structure validation to CI pipeline

- **Extended Modules** (Feature-flagged, stub implementations):
  - `terminal` module: key events, clipboard access, ANSI colorization
  - `gpu` module: NVML/DGX metrics (utilization, memory, temperature, power)
  - `fs_stream` module: file tailing with backpressure, streaming reads
  - `net_http` module: enhanced HTTP client with retries, streaming, custom options

- **Release Infrastructure**:
  - Enhanced `.github/workflows/release.yml` with comprehensive automation
  - Added semantic version validation
  - Added changelog extraction and GitHub release creation
  - Added benchmark job for performance tracking
  - Created `.github/CODEOWNERS` for code ownership and review requirements

- **Sigilforge Integration** (Feature-flagged):
  - `sigilforge` module: Integration with Sigilforge authentication daemon
  - Now uses `sigilforge-client` from crates.io (v0.1.2)
  - Provides `get_token`, `ensure_token`, `resolve`, and `is_available` functions

### Changed
- Updated README with versioned docs links and new module descriptions
- Enhanced CI workflow with documentation validation checks
- Updated `Cargo.toml` with feature flags for new modules
- Sigilforge feature now uses crates.io dependency instead of path dependency

## [0.1.1] - 2025-12-05

### Fixed
- Fixed compilation issues for crates.io release
- Switched to using fusabi-host from crates.io instead of git dependency

### Added
- Added initial release workflow for crates.io publishing

## [0.1.0] - 2025-12-04

### Added
- Initial release of `fusabi-stdlib-ext`
- Core standard library modules: `Process`, `Fs`, `Path`, `Env`, `Format`, `Net`, `Time`, `Metrics`
- Domain-specific packs:
  - `terminal-ui` (Ratatui/Crossterm integration)
  - `observability` (Tracing/OpenTelemetry integration)
  - `k8s` (Kubernetes client helpers)
  - `mcp` (Model Context Protocol helpers)
- Default-deny safety policies for filesystem and network access

[Unreleased]: https://github.com/fusabi-lang/fusabi-stdlib-ext/compare/v0.1.4...HEAD
[0.1.4]: https://github.com/fusabi-lang/fusabi-stdlib-ext/compare/v0.1.3...v0.1.4
[0.1.3]: https://github.com/fusabi-lang/fusabi-stdlib-ext/compare/v0.1.1...v0.1.3
[0.1.1]: https://github.com/fusabi-lang/fusabi-stdlib-ext/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/fusabi-lang/fusabi-stdlib-ext/releases/tag/v0.1.0
