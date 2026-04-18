# Contributing to AVM

## Development Setup

### Prerequisites
- Rust 1.75 or later
- Cargo
- Git

### Clone and Build
```bash
git clone https://github.com/astrolune/avm.git
cd avm
cargo build --workspace
cargo test --workspace
```

## Project Structure

```
avm/
├── avm-core/          # Core traits and types
├── avm-runtime/       # Runtime implementation
├── avm-sandbox/       # Sandbox and resource limits
├── avm-plugin/        # Plugin system
├── avm-ipc/          # IPC integration
└── avm-examples/     # Usage examples
```

## Development Workflow

1. Create a feature branch
2. Make changes
3. Run tests: `cargo test --workspace`
4. Run clippy: `cargo clippy --workspace -- -D warnings`
5. Format code: `cargo fmt --all`
6. Submit pull request

## Code Standards

### Rust Style
- Follow official Rust style guide
- Use `cargo fmt` for formatting
- Pass `cargo clippy` with no warnings
- Document public APIs with rustdoc

### Testing
- Unit tests in same file as implementation
- Integration tests in `tests/` directory
- Aim for >80% code coverage
- Include negative/security tests

### Documentation
- Document all public APIs
- Include examples in rustdoc
- Update README.md for user-facing changes
- Add ADRs for architectural decisions

## Pull Request Process

1. Update documentation
2. Add tests for new functionality
3. Ensure all tests pass
4. Update CHANGELOG.md
5. Request review from maintainers

## Security

Report security issues to security@astrolune.dev (do not open public issues).

## License

By contributing, you agree to license your contributions under MIT OR Apache-2.0.
