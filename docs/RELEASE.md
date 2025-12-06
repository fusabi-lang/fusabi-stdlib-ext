# Release Process

This document describes the release process for fusabi-stdlib-ext.

## Overview

Releases are automated through GitHub Actions and follow semantic versioning (SemVer). The release workflow handles building, testing, packaging, and publishing to crates.io.

## Semantic Versioning

We follow [Semantic Versioning 2.0.0](https://semver.org/):

- **MAJOR** version for incompatible API changes
- **MINOR** version for new functionality in a backward-compatible manner
- **PATCH** version for backward-compatible bug fixes

### Pre-release versions
- **alpha**: Early development, API unstable
- **beta**: Feature complete, API stabilizing
- **rc**: Release candidate, ready for final testing

## Release Checklist

### 1. Pre-release Preparation

- [ ] Ensure all tests pass on `main` branch
- [ ] Review and update CHANGELOG.md with all changes since last release
- [ ] Update version in Cargo.toml
- [ ] Update documentation in docs/versions/ (create new version directory)
- [ ] Verify all examples compile and run
- [ ] Run benchmarks and compare with previous version
- [ ] Update README.md if needed (especially version references)

### 2. Version Bump

```bash
# For a new minor version (e.g., 0.1.0 -> 0.2.0)
# Update Cargo.toml manually or use cargo-release
cargo release version minor --execute

# For a patch version (e.g., 0.1.0 -> 0.1.1)
cargo release version patch --execute

# For a major version (e.g., 0.1.0 -> 1.0.0)
cargo release version major --execute
```

### 3. Create Release PR

```bash
git checkout -b release/v0.x.0
git add .
git commit -m "chore: prepare release v0.x.0"
git push origin release/v0.x.0
gh pr create --title "Release v0.x.0" --body "Preparing release v0.x.0"
```

### 4. Review and Merge

- [ ] Code review by at least one CODEOWNER
- [ ] All CI checks must pass
- [ ] Documentation review
- [ ] Merge PR to `main`

### 5. Tag and Release

Once merged to `main`, create a Git tag:

```bash
git checkout main
git pull origin main
git tag -a v0.x.0 -m "Release v0.x.0"
git push origin v0.x.0
```

The GitHub Actions release workflow will automatically:
- Build the crate
- Run all tests
- Generate API documentation
- Create GitHub release with changelog
- Publish to crates.io (if configured)

### 6. Post-release

- [ ] Verify crates.io publication
- [ ] Verify GitHub release is created
- [ ] Verify documentation is published to docs.rs
- [ ] Announce release (if significant)
- [ ] Create vNEXT docs for next development cycle

## Hotfix Process

For critical bugs requiring immediate release:

1. Create hotfix branch from latest release tag
2. Apply minimal fix
3. Update CHANGELOG.md
4. Bump patch version
5. Follow release process with expedited review

```bash
git checkout v0.1.0
git checkout -b hotfix/v0.1.1
# Make changes
git commit -m "fix: critical bug description"
# Follow release process
```

## Branch Protection Rules

The `main` branch has the following protections:

- Require pull request before merging
- Require at least 1 approval from CODEOWNERS
- Require status checks to pass:
  - CI: check, test, fmt, clippy, docs
  - Security audit
- Require branches to be up to date before merging
- No force pushes
- No deletions

## Publishing to crates.io

Publishing is handled automatically by the release workflow when a version tag is pushed.

### Manual Publishing (if needed)

```bash
# Ensure you're on the release commit
git checkout v0.x.0

# Publish to crates.io
cargo publish --dry-run  # Test first
cargo publish            # Actual publish
```

### Requirements
- Must be a member of the fusabi-lang organization on crates.io
- Must have `CARGO_REGISTRY_TOKEN` configured in GitHub Secrets

## Rollback Procedure

If a release has critical issues:

1. **Do not delete the release** - crates.io versions cannot be deleted
2. Immediately release a new patch version with the fix
3. Mark the problematic release as yanked on crates.io:
   ```bash
   cargo yank --version 0.x.0
   ```
4. Update release notes to indicate the issue

## Release Artifacts

Each release produces:
- Source tarball on GitHub
- Published crate on crates.io
- API documentation on docs.rs
- GitHub release with generated changelog

## Changelog Generation

The CHANGELOG.md is maintained manually following [Keep a Changelog](https://keepachangelog.com/) format.

Categories:
- **Added**: New features
- **Changed**: Changes to existing functionality
- **Deprecated**: Soon-to-be removed features
- **Removed**: Removed features
- **Fixed**: Bug fixes
- **Security**: Security fixes

## Support Policy

- Latest minor version receives all updates
- Previous minor version receives security fixes for 3 months
- Major version support determined on a case-by-case basis

## Questions?

For questions about the release process, contact the repository maintainers or open an issue.
