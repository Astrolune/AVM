# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Initial project structure with workspace layout
- Core traits: Runtime, Executor, Plugin
- Configuration model with sandbox, runtime, and telemetry settings
- Sandbox implementation with memory, filesystem, and network controls
- Runtime lifecycle management with graceful shutdown
- Plugin registry system
- IPC integration via Unix sockets (Unix platforms)
- Telemetry hooks for monitoring
- Comprehensive test suite (unit, integration, security tests)
- Documentation: README, ARCHITECTURE, SECURITY, CONTRIBUTING
- Example: basic_usage demonstrating runtime and executor usage

### Security
- Semaphore-based memory tracking
- Path-based filesystem access control
- Network access control
- Timeout enforcement for task execution
- No custom cryptography

## [0.1.0] - 2026-04-18

### Added
- Initial release of AVM (Astrolune Virtual Machine)
- Rust-first architecture for secure isolated execution
- Modular crate structure: avm-core, avm-runtime, avm-sandbox, avm-plugin, avm-ipc
- MVP feature set ready for integration testing
