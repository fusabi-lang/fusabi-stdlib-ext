# Documentation Structure

This document describes the required structure and organization for documentation in the fusabi-stdlib-ext repository.

## Directory Layout

```
docs/
├── STRUCTURE.md         # This file - describes documentation organization
├── RELEASE.md          # Release process and guidelines
└── versions/           # Versioned documentation
    ├── v0.1/          # Stable release documentation
    └── vNEXT/         # Upcoming/development documentation
```

## Required Sections

Each versioned documentation directory should contain:

### 1. Introduction
- Overview of the library and its purpose
- Key features and capabilities
- Target audience and use cases

### 2. Installation
- Installation via Cargo
- Feature flags and optional dependencies
- Minimum Supported Rust Version (MSRV)

### 3. Usage Guide
- Quickstart examples
- Common patterns and best practices
- Safety model and security considerations
- Configuration options

### 4. API Documentation
- Module reference for each feature
- Function signatures and parameters
- Return values and error handling
- Code examples for each module

### 5. Capability Flags
- Complete list of feature flags
- Dependencies for each feature
- Capability-based security model
- Matrix of features and their use cases

### 6. Examples
- Practical examples covering real-world scenarios
- Integration examples for scarab, hibana, tolaria
- Performance and benchmarking examples

### 7. Changelog Links
- Link to CHANGELOG.md for release history
- Migration guides for breaking changes
- Deprecation notices

## Documentation Standards

### Formatting
- Use Markdown for all documentation
- Code blocks must specify language for syntax highlighting
- Include complete, runnable examples where possible
- Use tables for feature matrices and comparisons

### Versioning Policy
- Documentation is versioned alongside releases
- `vNEXT` contains documentation for unreleased features
- On release, `vNEXT` is copied to the appropriate version directory
- README.md should always point to the latest stable version

### Link Validation
- All internal links must be validated in CI
- External links should be checked periodically
- Broken links will fail the CI pipeline

## CI Integration

The documentation check runs on every pull request and ensures:
- All required sections are present
- No orphaned files exist
- Internal links are valid
- Code examples compile (where applicable)
- Versioned docs are properly structured

## Maintenance

- Update docs when adding new features
- Keep examples up to date with API changes
- Archive old versions when dropping support
- Remove only truly obsolete content - never delete useful information without migrating it
