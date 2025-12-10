# Testing Documentation

This document provides an overview of the test coverage for the putioarr project.

## Overview

The project now includes comprehensive unit and integration tests covering the main components:

- **Services Layer**: Tests for Put.io API interactions, Transmission protocol compatibility, and Arr service integrations
- **HTTP Layer**: Tests for authentication, routing, and request/response handling
- **Download System**: Tests for transfer management and target handling
- **Configuration**: Tests for config parsing, serialization, and validation
- **Utilities**: Tests for template generation and config file creation

## Test Statistics

- **Total Tests**: 99 passing tests
- **Test Coverage Areas**: 6 major modules
- **Test Files**: 6 test modules

## Running Tests

### Run All Tests
```bash
cargo test
```

### Run Tests with Output
```bash
cargo test -- --nocapture
```

### Run Specific Test Module
```bash
# Services tests
cargo test services::

# HTTP route tests
cargo test routes_tests::

# Config tests
cargo test config_tests::

# Utils tests
cargo test utils_tests::

# Transfer tests
cargo test transfer_tests::

# Transmission tests
cargo test transmission_tests::
```

### Run Specific Test
```bash
cargo test test_name
```

## Test Modules

### 1. Services - Transmission Tests (`src/services/transmission_tests.rs`)

Tests the Transmission protocol compatibility layer that translates between Put.io transfers and Transmission RPC protocol.

**Key Tests:**
- `test_transmission_config_default()` - Validates default configuration values
- `test_transmission_torrent_from_putio_transfer_completed()` - Tests conversion of completed transfers
- `test_transmission_torrent_from_putio_transfer_downloading()` - Tests conversion of active downloads
- `test_transmission_torrent_status_*()` - Tests all status mapping variants
- `test_transmission_torrent_left_until_done_negative_protection()` - Ensures download progress never goes negative

**Coverage:**
- TransmissionConfig serialization/deserialization
- TransmissionTorrent conversion from PutIOTransfer
- All TransmissionTorrentStatus variants
- Edge cases (missing data, errors, unknown statuses)

### 2. Services - Put.io Tests (`src/services/putio_tests.rs`)

Tests the Put.io API client and data structures.

**Key Tests:**
- `test_putio_transfer_is_downloadable_*()` - Tests downloadability logic
- `test_putio_account_info_*()` - Tests account info handling
- `test_putio_transfer_deserialization_*()` - Tests transfer data parsing
- `test_list_transfer_response_deserialization()` - Tests list responses
- `test_file_response_deserialization()` - Tests file metadata parsing

**Coverage:**
- PutIOTransfer validation and downloadability checks
- All Put.io API response structures
- Various transfer statuses (DOWNLOADING, COMPLETED, ERROR, etc.)
- Edge cases (null values, missing fields)

### 3. HTTP - Routes Tests (`src/http/routes_tests.rs`)

Tests the HTTP endpoints and authentication.

**Key Tests:**
- `test_rpc_get_with_valid_auth()` - Tests GET endpoint with valid credentials
- `test_rpc_get_with_invalid_auth()` - Tests authentication rejection
- `test_rpc_post_session_get()` - Tests session-get RPC method
- `test_transmission_request_deserialization()` - Tests request parsing

**Coverage:**
- Basic authentication (valid/invalid credentials)
- Transmission RPC endpoints (GET/POST)
- Session management
- Request/response serialization

### 4. Download System - Transfer Tests (`src/download_system/transfer_tests.rs`)

Tests the transfer management and download target generation.

**Key Tests:**
- `test_download_target_display()` - Tests display formatting
- `test_download_target_file()` - Tests file target creation
- `test_download_target_directory()` - Tests directory target creation
- `test_transfer_from_putio_transfer()` - Tests transfer object creation
- `test_transfer_get_top_level()` - Tests top-level target retrieval

**Coverage:**
- DownloadTarget creation and validation
- TargetType (File/Directory) handling
- Transfer display formatting
- Serialization/deserialization
- Transfer hash handling

### 5. Configuration Tests (`src/config_tests.rs`)

Tests configuration parsing, validation, and serialization.

**Key Tests:**
- `test_config_serialization()` - Tests config to JSON
- `test_config_deserialization()` - Tests JSON to config
- `test_config_toml_parsing()` - Tests TOML file parsing
- `test_config_with_defaults()` - Tests default value handling
- `test_config_with_all_arr_services()` - Tests multiple Arr services

**Coverage:**
- Full Config structure serialization/deserialization
- PutioConfig and ArrConfig sub-structures
- TOML file parsing with Figment
- Default value application
- Various configuration scenarios (all services, no services, different ports, etc.)
- Config cloning

### 6. Utilities Tests (`src/utils_tests.rs`)

Tests the configuration file template and utility functions.

**Key Tests:**
- `test_template_contains_required_fields()` - Validates all required config fields
- `test_template_contains_putio_section()` - Tests Put.io section
- `test_template_contains_arr_sections()` - Tests Sonarr/Radarr/Whisparr sections
- `test_template_has_default_values()` - Tests default value presence
- `test_context_serialization()` - Tests template context handling

**Coverage:**
- TEMPLATE constant validation
- All configuration sections
- Default values
- Comments and documentation
- TOML structure validation
- Context structure for template rendering

## Test Patterns and Best Practices

### 1. Unit Tests
Each module has comprehensive unit tests for its data structures and functions:
- Tests are isolated and don't depend on external services
- Mock data is used for API responses
- Edge cases are thoroughly tested

### 2. Integration Tests
Config and HTTP tests verify component integration:
- TOML parsing with Figment
- HTTP request/response handling with actix-web
- Authentication flow

### 3. Test Helpers
Common helper functions for test setup:
```rust
fn create_test_config() -> Config
fn create_test_app_data() -> web::Data<AppData>
fn create_basic_auth_header(username: &str, password: &str) -> HeaderValue
```

### 4. Serialization Tests
All data structures have serialization/deserialization tests:
- JSON format validation
- TOML format validation
- Round-trip testing (serialize → deserialize → compare)

## Dependencies for Testing

The following dev-dependencies are used:

```toml
[dev-dependencies]
mockito = "1.2.0"              # HTTP mocking (for future API tests)
tokio = { version = "1.32.0", features = ["test-util", "macros"] }
actix-web = { version = "4.5.1", features = ["macros"] }
actix-rt = "2.9.0"
tempfile = "3.8.0"             # Temporary file creation for config tests
```

## Coverage Areas

### ✅ Fully Covered
- Configuration parsing and validation
- Data structure serialization/deserialization
- Transmission protocol compatibility
- Put.io data structures
- Transfer management
- HTTP authentication
- Template generation

### 🚧 Partial Coverage
- HTTP handlers (only basic tests, no mock API calls)
- Download workers (unit tests only, no integration tests)
- Orchestration workers (not tested due to async complexity)

### ❌ Not Covered
- Main function and CLI argument parsing
- Actual file downloads (requires real HTTP connections)
- Put.io API calls (would require mocking or live API)
- Arr service API calls (would require mocking or live API)
- File system operations in download workers

## Future Improvements

1. **Mock API Tests**: Add mockito-based tests for Put.io and Arr API calls
2. **Integration Tests**: Add end-to-end tests with test servers
3. **Property-Based Testing**: Use proptest for fuzzing inputs
4. **Async Worker Tests**: Test download and orchestration workers
5. **Code Coverage**: Integrate with cargo-tarpaulin for coverage reports
6. **Performance Tests**: Add benchmarks for critical paths

## Continuous Integration

This project uses GitHub Actions for automated testing and CI/CD.

### GitHub Actions Workflows

#### Test Workflow (`.github/workflows/test.yml`)

The test workflow runs automatically on:
- Every push to `main` or `develop` branches
- Every pull request to `main` or `develop` branches
- All Dependabot PRs

**What it does:**
- Runs the full test suite on Ubuntu and macOS
- Tests with both stable and beta Rust versions
- Runs `cargo clippy` for linting
- Checks code formatting with `rustfmt`
- Builds the project for multiple platforms
- Auto-approves and auto-merges passing Dependabot PRs (patch/minor updates only)

**Jobs:**
1. **Test Suite**: Runs all 99 tests on multiple platforms
2. **Formatting**: Validates code formatting with `cargo fmt`
3. **Clippy**: Runs Clippy linter with warnings as errors
4. **Build**: Compiles release builds for verification
5. **Dependabot Auto-Merge**: Automatically merges safe dependency updates

### Dependabot Integration

Dependabot is configured to:
- Check for updates daily at 3 AM UTC
- Monitor Cargo, Docker, and GitHub Actions dependencies
- Create PRs with conventional commit messages
- Automatically trigger the test workflow

When Dependabot creates a PR:
1. Tests run automatically
2. If all tests pass and it's a patch/minor update, PR is auto-approved
3. PR is set to auto-merge when all checks pass
4. Major version updates require manual review

### Running CI Locally

You can run the same checks locally before pushing:

```bash
# Run tests (same as CI)
cargo test --all-features
cargo test --no-default-features

# Check formatting
cargo fmt --all -- --check

# Run clippy
cargo clippy --all-targets --all-features -- -D warnings

# Build release
cargo build --release
```

### CI Status

Check the status of CI runs:
- Go to the "Actions" tab in the GitHub repository
- View workflow runs and their logs
- See which tests passed/failed
- Download build artifacts

### Manual Test Coverage with Tarpaulin

For detailed coverage reports (optional):

```bash
# Install tarpaulin
cargo install cargo-tarpaulin

# Generate coverage report
cargo tarpaulin --out Html --out Xml

# View HTML report
open tarpaulin-report.html
```

## Troubleshooting

### Tests Fail to Compile
Ensure all dependencies are installed:
```bash
cargo clean
cargo build --tests
```

### Specific Test Fails
Run with verbose output:
```bash
cargo test failing_test_name -- --nocapture
```

### All Tests Fail
Check that you're in the project root:
```bash
cd putioarr
cargo test
```

## Contributing Tests

When adding new features, please:

1. Add unit tests for new functions/methods
2. Add integration tests for new endpoints or workflows
3. Test edge cases and error conditions
4. Update this documentation with new test coverage
5. Ensure all tests pass before submitting PR

### Test Naming Convention
- Unit tests: `test_<function_or_feature_name>()`
- Integration tests: `test_<workflow_or_scenario>()`
- Edge cases: `test_<feature>_<edge_case>()`

### Example Test Template
```rust
#[test]
fn test_feature_name() {
    // Arrange
    let input = create_test_data();
    
    // Act
    let result = function_under_test(input);
    
    // Assert
    assert_eq!(result, expected_value);
}
```

## Automated Quality Checks

The CI pipeline ensures:
- ✅ All tests pass before merging
- ✅ Code follows formatting standards
- ✅ No clippy warnings
- ✅ Builds successfully on all platforms
- ✅ Dependencies are kept up to date safely

This automation provides confidence that:
- PRs don't break existing functionality
- Dependency updates are safe
- Code quality standards are maintained
- The project builds correctly on all supported platforms

## License

Tests are part of the putioarr project and follow the same MIT license.