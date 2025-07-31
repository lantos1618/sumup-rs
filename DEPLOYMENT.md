# Deployment Guide

This guide covers how to deploy the SumUp Rust client to Crates.io and set up continuous integration.

## 🚀 Publishing to Crates.io

### Prerequisites

1. **Crates.io Account**: Create an account at [crates.io](https://crates.io)
2. **API Token**: Generate an API token in your account settings
3. **GitHub Secrets**: Add the token to your GitHub repository secrets

### Setup Steps

#### 1. Update Package Metadata

Before publishing, ensure your `Cargo.toml` has the correct metadata:

```toml
[package]
name = "sumup-rs"
version = "0.2.0"
edition = "2021"
description = "A comprehensive, type-safe Rust client for the SumUp API with full async/await support"
license = "MIT"
repository = "https://github.com/yourusername/sumup-rs"
documentation = "https://docs.rs/sumup-rs"
readme = "README.md"
keywords = ["sumup", "api", "payment", "fintech", "3ds", "checkout", "merchant"]
categories = ["api-bindings", "web-programming", "asynchronous"]
authors = ["Your Name <your.email@example.com>"]
homepage = "https://github.com/yourusername/sumup-rs"
```

#### 2. Set Up GitHub Secrets

1. Go to your GitHub repository settings
2. Navigate to "Secrets and variables" → "Actions"
3. Add the following secret:
   - **Name**: `CARGO_REGISTRY_TOKEN`
   - **Value**: Your Crates.io API token

#### 3. Test Locally

Before publishing, test the package locally:

```bash
# Check if the package builds correctly
cargo check

# Run all tests
cargo test

# Check formatting
cargo fmt --all -- --check

# Run clippy
cargo clippy --all-targets --all-features -- -D warnings

# Build documentation
cargo doc --no-deps --all-features
```

#### 4. Manual Publishing (Optional)

If you prefer to publish manually:

```bash
# Login to Crates.io
cargo login

# Publish the package
cargo publish
```

### Automated Publishing

The CI/CD pipeline will automatically publish when you:

1. Create a new release on GitHub
2. Tag it with a version number (e.g., `v0.2.0`)
3. The workflow will run tests and security audits
4. If all checks pass, it will publish to Crates.io

## 🔧 CI/CD Pipeline

### What the Pipeline Does

The GitHub Actions workflow (`.github/workflows/ci.yml`) includes:

#### **Test Suite Job**
- ✅ **Format Check**: Ensures code follows Rust formatting standards
- ✅ **Clippy**: Runs static analysis for potential issues
- ✅ **Unit Tests**: Runs all tests with verbose output
- ✅ **Release Build**: Ensures the package builds in release mode

#### **Security Audit Job**
- ✅ **Cargo Audit**: Checks for known security vulnerabilities
- ✅ **Dependency Scanning**: Identifies outdated or vulnerable dependencies

#### **Documentation Job**
- ✅ **Doc Generation**: Builds comprehensive documentation
- ✅ **Artifact Upload**: Stores documentation for later use

#### **Publish Job** (Triggered on Release)
- ✅ **Automated Publishing**: Publishes to Crates.io when tests pass
- ✅ **Token Security**: Uses encrypted secrets for authentication

### Pipeline Triggers

The pipeline runs on:
- **Push to main/develop**: Runs tests and security checks
- **Pull Requests**: Validates code quality before merging
- **Release Creation**: Automatically publishes to Crates.io

## 📋 Pre-Publication Checklist

Before publishing a new version:

### Code Quality
- [ ] All tests pass (`cargo test`)
- [ ] Code is formatted (`cargo fmt --all`)
- [ ] No clippy warnings (`cargo clippy --all-targets --all-features -- -D warnings`)
- [ ] Security audit passes (`cargo audit`)

### Documentation
- [ ] README.md is up to date
- [ ] IMPLEMENTATION_STATUS.md reflects current state
- [ ] Examples are working
- [ ] API documentation is complete

### Version Management
- [ ] Update version in `Cargo.toml`
- [ ] Update version in `README.md` installation example
- [ ] Create a git tag for the version
- [ ] Create a GitHub release

### Metadata
- [ ] Package description is accurate
- [ ] Keywords are relevant
- [ ] Categories are appropriate
- [ ] Repository URL is correct
- [ ] Author information is current

## 🎯 Release Process

### 1. Prepare Release
```bash
# Update version in Cargo.toml
# Update documentation
# Run final tests
cargo test
cargo check
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
```

### 2. Create Release
```bash
# Commit changes
git add .
git commit -m "Prepare for v0.2.0 release"
git push

# Create and push tag
git tag v0.2.0
git push origin v0.2.0
```

### 3. GitHub Release
1. Go to GitHub repository
2. Click "Releases" → "Create a new release"
3. Select the tag you just created
4. Add release notes
5. Publish the release

The CI/CD pipeline will automatically:
- Run all tests
- Perform security audit
- Publish to Crates.io

## 🔍 Post-Publication Verification

After publishing:

1. **Check Crates.io**: Verify the package appears correctly
2. **Test Installation**: Try installing in a new project
3. **Documentation**: Check that docs.rs builds correctly
4. **Examples**: Verify examples work with the published version

## 🛠️ Troubleshooting

### Common Issues

#### **Publishing Fails**
- Check that your API token is valid
- Ensure the package name is available
- Verify all required metadata is present

#### **CI/CD Pipeline Fails**
- Check GitHub Actions logs for specific errors
- Ensure all secrets are properly configured
- Verify the workflow file syntax

#### **Documentation Build Fails**
- Check for missing documentation comments
- Ensure all public APIs are documented
- Verify markdown syntax in doc comments

### Getting Help

- **Crates.io Issues**: Check [crates.io documentation](https://doc.rust-lang.org/cargo/reference/publishing.html)
- **GitHub Actions**: Review [GitHub Actions documentation](https://docs.github.com/en/actions)
- **Rust Documentation**: Consult [Rust book](https://doc.rust-lang.org/book/) for packaging

## 📈 Monitoring

After deployment, monitor:

- **Download Statistics**: Track package usage on Crates.io
- **Issue Reports**: Monitor GitHub issues for user feedback
- **Security Alerts**: Watch for security advisories
- **Dependency Updates**: Keep dependencies current

This ensures your package remains reliable and secure for users. 