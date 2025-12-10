# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Comprehensive test suite with 99 passing tests covering all major components
  - Services layer tests (Transmission protocol, Put.io API)
  - HTTP layer tests (routes, authentication)
  - Download system tests (transfer management)
  - Configuration tests (TOML parsing, validation)
  - Utilities tests (template generation)
- Test documentation (`TESTING.md`) with detailed testing guide
- CI/CD automation with GitHub Actions:
  - Automated testing workflow (`test.yml`) for all PRs and pushes
  - Tests run on Ubuntu and macOS with stable and beta Rust
  - Clippy linting and rustfmt checking
  - Build verification for multiple platforms
  - Automated testing of Dependabot PRs
  - Auto-approval and auto-merge for passing patch/minor dependency updates
- Dependabot configuration for daily dependency updates (3 AM UTC)
  - Cargo dependencies monitoring
  - Docker base image monitoring
  - GitHub Actions workflow monitoring
  - Conventional commit messages for dependency updates
- Dev dependencies for testing:
  - `mockito` for HTTP mocking capability
  - `tokio` with test-util and macros features
  - `actix-web` with macros feature for HTTP testing
  - `tempfile` for temporary file creation in tests

### Changed
- Made `SESSION_ID` constant public for testing purposes
- Made `TEMPLATE` and `Context` public in utils module for testing
- Added `Serialize` derive to `TransmissionRequest` struct
- Added `Deserialize` derive to `TransmissionResponse` struct
- Added `Deserialize` derive to `Context` struct

### Documentation
- Added comprehensive testing documentation
- Added test coverage summary
- Documented all test modules and their purposes
- Included examples of running tests
- Added troubleshooting section for common test issues

## [0.5.36] - 2025-12-10

### Features
- Put.io to Sonarr/Radarr/Whisparr proxy functionality
- Transmission protocol compatibility
- Automatic file download and management
- Support for multiple Arr services
- Docker support with linuxserver.io base images
- OAuth token generation via OOB flow
- Configurable download workers and orchestration
- Skip directories configuration
- UID/GID management for downloaded files

### Supported Platforms
- Linux (amd64, arm64)
- macOS (via Cargo)
- Docker containers

---

## Release Notes

### Version Unreleased - Test Coverage & CI/CD Automation

This release adds comprehensive test coverage and automated CI/CD workflows to the project, improving code quality, maintainability, and confidence in future changes.

**Test Statistics:**
- Total Tests: 99
- Success Rate: 100%
- Code Coverage: All major components covered
- Test Lines: ~1,868 lines of test code

**What's Tested:**
- ✅ Data structure serialization/deserialization
- ✅ Transmission protocol compatibility
- ✅ Put.io API response handling
- ✅ HTTP authentication and routing
- ✅ Configuration parsing with TOML
- ✅ Transfer and target management
- ✅ Template generation and validation

**CI/CD Improvements:**
- Dependabot configured for daily updates at 3 AM UTC
- GitHub Actions workflow for automated testing
  - Runs on every push and pull request
  - Tests on Ubuntu and macOS
  - Validates with Clippy and rustfmt
  - Builds for multiple platforms
- Automated testing of Dependabot PRs
- Auto-merge for passing patch/minor dependency updates
- Conventional commit messages for all dependency updates

**For Users:**
No user-facing changes in this release. All changes are internal improvements to code quality, testing infrastructure, and automated dependency management.

**For Developers:**
- Run tests with `cargo test`
- See `TESTING.md` for detailed testing guide
- See `CI_CD.md` for CI/CD documentation
- All major components now have unit tests
- Edge cases and error conditions covered
- CI/CD runs automatically on all PRs
- Dependabot PRs are auto-tested and can auto-merge if tests pass

---

## Previous Versions

For release history prior to version 0.5.36, please refer to the Git commit history.

---

## Contributing

When contributing to this project:

1. **Add Tests**: All new features should include tests
2. **Run Tests**: Ensure `cargo test` passes before submitting PR
3. **Update Changelog**: Add your changes to the [Unreleased] section
4. **Follow Conventions**: Use conventional commit messages
5. **Documentation**: Update relevant documentation files

## Links

- [Repository](https://github.com/wouterdebie/putioarr)
- [Issues](https://github.com/wouterdebie/putioarr/issues)
- [Pull Requests](https://github.com/wouterdebie/putioarr/pulls)