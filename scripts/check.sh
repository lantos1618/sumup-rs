#!/bin/bash

# Development quality check script
# This script runs all the same checks as the CI/CD pipeline locally

set -e  # Exit on any error

echo "🔍 Running SumUp Rust Client Quality Checks..."
echo "=============================================="

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    if [ $1 -eq 0 ]; then
        echo -e "${GREEN}✅ $2${NC}"
    else
        echo -e "${RED}❌ $2${NC}"
        exit 1
    fi
}

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    echo -e "${RED}❌ Error: Cargo.toml not found. Please run this script from the project root.${NC}"
    exit 1
fi

echo -e "${YELLOW}📦 Checking Rust toolchain...${NC}"
rustc --version
cargo --version

echo -e "\n${YELLOW}🔧 Checking code formatting...${NC}"
cargo fmt --all -- --check
print_status $? "Code formatting check passed"

echo -e "\n${YELLOW}🔍 Running clippy linting...${NC}"
cargo clippy --all-targets --all-features -- -D warnings
print_status $? "Clippy linting passed"

echo -e "\n${YELLOW}🧪 Running tests...${NC}"
cargo test --all-features
print_status $? "All tests passed"

echo -e "\n${YELLOW}🔒 Running security audit...${NC}"
if command -v cargo-audit &> /dev/null; then
    cargo audit
    print_status $? "Security audit passed"
else
    echo -e "${YELLOW}⚠️  cargo-audit not installed. Installing...${NC}"
    cargo install cargo-audit
    cargo audit
    print_status $? "Security audit passed"
fi

echo -e "\n${YELLOW}📚 Building documentation...${NC}"
cargo doc --no-deps --all-features
print_status $? "Documentation built successfully"

echo -e "\n${YELLOW}🏗️  Building release version...${NC}"
cargo build --release
print_status $? "Release build successful"

echo -e "\n${GREEN}🎉 All quality checks passed!${NC}"
echo -e "${GREEN}Your code is ready for commit and CI/CD pipeline.${NC}"

# Optional: Show some useful information
echo -e "\n${YELLOW}📊 Project Statistics:${NC}"
echo "Lines of code: $(find src -name "*.rs" -exec wc -l {} + | tail -1 | awk '{print $1}')"
echo "Test files: $(find tests -name "*.rs" | wc -l)"
echo "Example files: $(find examples -name "*.rs" | wc -l)" 