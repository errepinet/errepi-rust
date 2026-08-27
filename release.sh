#!/bin/bash

# Release script for the Rust library
# Usage: ./release.sh <version> [--dry-run]

set -e

# Output colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Utility functions
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check parameters
if [ $# -eq 0 ]; then
    log_error "Specify the version to release"
    echo "Usage: $0 <version> [--dry-run]"
    echo "Example: $0 1.2.3"
    exit 1
fi

VERSION=$1
DRY_RUN=false

if [ "$2" = "--dry-run" ]; then
    DRY_RUN=true
    log_warning "DRY RUN mode activated - no changes will be applied"
fi

# Validate version format (semantic versioning)
if ! [[ $VERSION =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
    log_error "Invalid version format. Use semver format (e.g.: 1.2.3)"
    exit 1
fi

# Check that we are on main/master branch
CURRENT_BRANCH=$(git branch --show-current)
if [[ "$CURRENT_BRANCH" != "main" && "$CURRENT_BRANCH" != "master" ]]; then
    log_error "You must be on the main/master branch to create a release"
    exit 1
fi

# Check that the repository is clean
if [ -n "$(git status --porcelain)" ]; then
    log_error "The repository has uncommitted changes. Commit or stash changes before continuing."
    exit 1
fi

# Check that we are up to date with remote
log_info "Checking for updates from remote..."
git fetch origin

LOCAL=$(git rev-parse HEAD)
REMOTE=$(git rev-parse origin/$CURRENT_BRANCH)

if [ $LOCAL != $REMOTE ]; then
    log_error "Local branch is not up to date with remote. Run 'git pull' before continuing."
    exit 1
fi

# Check that the tag does not already exist
if git tag -l | grep -q "^v$VERSION$"; then
    log_error "Tag v$VERSION already exists"
    exit 1
fi

log_info "Preparing release v$VERSION..."

# Update the version in Cargo.toml if it differs
CARGO_VERSION=$(grep -oP '^version = "\K[^"]+' Cargo.toml | head -1)
if [ "$CARGO_VERSION" != "$VERSION" ] && [ "$DRY_RUN" = false ]; then
    log_info "Updating version in Cargo.toml from $CARGO_VERSION to $VERSION..."
    sed -i.bak "s/^version = \"$CARGO_VERSION\"/version = \"$VERSION\"/" Cargo.toml
    rm Cargo.toml.bak
    log_success "Version updated in Cargo.toml"
elif [ "$CARGO_VERSION" != "$VERSION" ]; then
    log_info "[DRY RUN] Would update version in Cargo.toml from $CARGO_VERSION to $VERSION"
else
    log_info "Cargo.toml already at version $VERSION"
fi

# Run tests
log_info "Running tests..."
if [ "$DRY_RUN" = false ]; then
    cargo test
else
    log_info "[DRY RUN] Skipping test execution"
fi

# Commit changes
if [ "$DRY_RUN" = false ]; then
    if [ -n "$(git status --porcelain)" ]; then
        log_info "Committing changes for version $VERSION..."
        git add .
        git commit -m "Bump version to v$VERSION"
    fi

    # Create the tag
    log_info "Creating tag v$VERSION..."
    git tag -a "v$VERSION" -m "Release version $VERSION"

    # Push branch and tag
    log_info "Pushing changes and tag..."
    git push origin $CURRENT_BRANCH
    git push origin "v$VERSION"

    log_success "Release v$VERSION created successfully!"
    log_info "Tag v$VERSION has been pushed to origin"
else
    log_info "[DRY RUN] The following operations would have been performed:"
    log_info "  - Commit changes with message 'Bump version to v$VERSION'"
    log_info "  - Create tag v$VERSION"
    log_info "  - Push branch $CURRENT_BRANCH"
    log_info "  - Push tag v$VERSION"
fi

echo ""
log_success "Release script completed!"
