# CI/CD Documentation

This document describes the continuous integration and continuous deployment (CI/CD) setup for putioarr.

## Overview

The project uses GitHub Actions for automated testing, linting, and building. All workflows are configured to ensure code quality and prevent breaking changes from being merged.

## Workflows

### 1. Test Workflow (`.github/workflows/test.yml`)

The primary CI workflow that runs on every push and pull request.

#### Triggers

- Push to `main` or `develop` branches
- Pull requests to `main` or `develop` branches
- Dependabot PRs (via `pull_request_target`)

#### Jobs

##### Test Suite
- **Platforms**: Ubuntu (latest), macOS (latest)
- **Rust Versions**: stable, beta
- **Actions**:
  - Runs full test suite with `cargo test --verbose --all-features`
  - Runs tests with no default features
  - Uses cargo caching for faster builds

##### Formatting
- **Platform**: Ubuntu (latest)
- **Actions**:
  - Checks code formatting with `cargo fmt --all -- --check`
  - Ensures consistent code style across the project

##### Clippy
- **Platform**: Ubuntu (latest)
- **Actions**:
  - Runs Clippy linter with `cargo clippy --all-targets --all-features -- -D warnings`
  - Treats all warnings as errors to maintain code quality

##### Build
- **Platforms**: Ubuntu (x86_64-unknown-linux-gnu), macOS (x86_64-apple-darwin)
- **Actions**:
  - Builds release binaries for verification
  - Uploads build artifacts (retained for 7 days)
  - Uses cargo caching for faster builds

##### Dependabot Auto-Merge
- **Trigger**: Only runs when PR is from `dependabot[bot]`
- **Dependencies**: Requires all other jobs (test, fmt, clippy, build) to pass
- **Actions**:
  - Fetches Dependabot metadata
  - Auto-approves patch and minor version updates
  - Enables auto-merge for approved PRs
  - Major version updates require manual review

### 2. Release Workflow (`.github/workflows/release.yml`)

Handles Docker image building and publishing.

#### Triggers

- Push to `main` branch
- Push to `releases/**` branches
- Version tags (`v*`)

#### Actions

- Builds multi-platform Docker images (amd64, arm64)
- Pushes to GitHub Container Registry (ghcr.io)
- Tags images appropriately based on branch/tag

## Dependabot Configuration

Located at `.github/dependabot.yml`

### Monitored Ecosystems

1. **Cargo Dependencies**
   - Daily checks at 3:00 AM UTC
   - Up to 10 open PRs
   - Labels: `dependencies`, `rust`
   - Commit prefix: `cargo`

2. **Docker**
   - Daily checks at 3:00 AM UTC
   - Up to 5 open PRs
   - Labels: `dependencies`, `docker`
   - Commit prefix: `docker`

3. **GitHub Actions**
   - Daily checks at 3:00 AM UTC
   - Up to 5 open PRs
   - Labels: `dependencies`, `github-actions`
   - Commit prefix: `ci`

### Auto-Merge Strategy

Dependabot PRs are automatically merged if:
- ✅ All tests pass
- ✅ Clippy check passes
- ✅ Formatting check passes
- ✅ Build succeeds on all platforms
- ✅ Update is a patch or minor version (not major)

Major version updates require manual review to assess breaking changes.

## Running CI Checks Locally

Before pushing code, you can run the same checks locally:

```bash
# Run all tests
cargo test --all-features
cargo test --no-default-features

# Check code formatting
cargo fmt --all -- --check

# Auto-format code
cargo fmt --all

# Run Clippy
cargo clippy --all-targets --all-features -- -D warnings

# Build release binary
cargo build --release
```

## Caching Strategy

The workflows use GitHub Actions caching to speed up builds:

- **Cargo Registry**: `~/.cargo/registry`
- **Cargo Index**: `~/.cargo/git`
- **Build Cache**: `target/`

Cache keys are based on `Cargo.lock` hash, ensuring fresh builds when dependencies change.

## Workflow Status

View the current status of workflows:

- **Badge**: ![Tests](https://github.com/wouterdebie/putioarr/actions/workflows/test.yml/badge.svg)
- **Actions Page**: https://github.com/wouterdebie/putioarr/actions

## Permissions

### Test Workflow
- `contents: read` (default)
- `pull-requests: write` (for Dependabot auto-approve)

### Release Workflow
- Requires `CR_PAT` secret for GitHub Container Registry authentication

## Troubleshooting

### Tests Fail in CI but Pass Locally

1. Ensure you're using the same Rust version as CI (stable)
2. Check for platform-specific issues (macOS vs Linux)
3. Verify all dependencies are up to date: `cargo update`

### Dependabot PR Not Auto-Merging

Check that:
1. All workflow jobs passed (test, fmt, clippy, build)
2. The update is patch or minor (not major)
3. Repository settings allow auto-merge
4. Branch protection rules are satisfied

### Cache Issues

If builds are slow or behaving oddly:
1. Clear GitHub Actions cache from the Actions UI
2. Re-run the workflow

## Best Practices

### For Contributors

1. **Before Creating PR**:
   - Run tests locally: `cargo test`
   - Format code: `cargo fmt --all`
   - Check with Clippy: `cargo clippy --all-targets --all-features`

2. **During PR Review**:
   - Ensure all CI checks pass (green checkmarks)
   - Address any warnings or errors from Clippy
   - Fix formatting issues if any

3. **After PR Approval**:
   - PRs are typically squash-merged to maintain clean history
   - CI will run again on the merged commit

### For Maintainers

1. **Reviewing Dependabot PRs**:
   - Patch/minor updates: Let auto-merge handle it if tests pass
   - Major updates: Review changelog, test manually, assess breaking changes

2. **Release Process**:
   - Tag releases with semantic versioning: `v1.2.3`
   - Release workflow automatically builds and publishes Docker images
   - Update CHANGELOG.md before tagging

## Security Considerations

- Secrets are stored in GitHub repository settings
- Dependabot has limited permissions (cannot access secrets)
- `pull_request_target` is used carefully to avoid security risks
- Only patch/minor updates auto-merge; major versions require review

## Future Improvements

Potential enhancements to the CI/CD pipeline:

- [ ] Code coverage reporting with cargo-tarpaulin
- [ ] Security audits with cargo-audit
- [ ] Performance benchmarks with criterion
- [ ] Integration tests with real Put.io sandbox account
- [ ] Automated release notes generation
- [ ] Nightly builds for early issue detection
- [ ] Docker image vulnerability scanning

## Support

For issues with CI/CD:
- Check workflow logs in GitHub Actions
- Review this documentation
- Open an issue with `ci` label
- See [TESTING.md](TESTING.md) for testing documentation
- See [CHANGELOG.md](CHANGELOG.md) for recent changes

---

**Last Updated**: December 10, 2025
**Workflow Version**: test.yml v1.0