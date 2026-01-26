# shellcheck shell=bash

################################################################################
## Version Utilities
##
## Shared utilities for version manipulation, validation, and querying.
## Used by both CI scripts and local operations.
################################################################################

# The crate name used in tags
readonly TAG_PREFIX="amber-api/v"

# Get the current version from Cargo.toml
#
# Parses the version field from Cargo.toml and returns it.
#
# Usage:
#   get_current_version
#
# Returns:
#   The current version string (e.g., "1.0.0")
#
# Example:
#   VERSION=$(get_current_version)
#
get_current_version() {
  # Use [[:space:]]* for BSD grep compatibility (macOS)
  grep '^[[:space:]]*version' Cargo.toml | head -n1 | sed 's/.*=.*"\(.*\)".*/\1/'
}

# Get the latest tag matching the amber-api pattern
#
# Returns the most recent tag in the format amber-api/vX.Y.Z
#
# Usage:
#   get_latest_tag
#
# Returns:
#   The latest tag (e.g., "amber-api/v1.0.0"), or empty if no tags exist
#
# Example:
#   LATEST_TAG=$(get_latest_tag)
#
get_latest_tag() {
  git tag -l "${TAG_PREFIX}*" | sort -V | tail -n1
}

# Check if a version is already tagged
#
# Usage:
#   is_version_tagged <version>
#
# Arguments:
#   $1 - The version to check (e.g., "2.0.0")
#
# Returns:
#   0 - If the version is tagged
#   1 - If the version is not tagged
#
# Example:
#   if is_version_tagged "2.0.0"; then
#     echo "Version already tagged"
#   fi
#
is_version_tagged() {
  local version="$1"
  local tag_name
  tag_name=$(format_tag_name "$version")
  git rev-parse "$tag_name" >/dev/null 2>&1
}

# Determine the next version using git-cliff
#
# Uses git-cliff's --bumped-version to automatically determine the next
# semantic version based on conventional commits.
#
# Usage:
#   determine_next_version
#
# Returns:
#   The next version string (e.g., "2.0.0")
#
# Example:
#   NEXT_VERSION=$(determine_next_version)
#
determine_next_version() {
  git-cliff --bumped-version | sed "s|^${TAG_PREFIX}||"
}

# Validate a semantic version format
#
# Checks if the provided version follows semantic versioning (X.Y.Z)
#
# Usage:
#   validate_semver <version>
#
# Arguments:
#   $1 - The version to validate (e.g., "2.0.0")
#
# Returns:
#   0 - If the version is valid semver
#   1 - If the version is invalid
#
# Example:
#   if ! validate_semver "$VERSION"; then
#     echo "Invalid version format"
#   fi
#
validate_semver() {
  local version="$1"
  [[ $version =~ ^[0-9]+\.[0-9]+\.[0-9]+(-[a-zA-Z0-9.+-]+)?$ ]]
}

# Format a version as a tag name
#
# Converts a version string to the tag format used by this project.
#
# Usage:
#   format_tag_name <version>
#
# Arguments:
#   $1 - The version (e.g., "2.0.0")
#
# Returns:
#   The formatted tag name (e.g., "amber-api/v2.0.0")
#
# Example:
#   TAG=$(format_tag_name "2.0.0")
#
format_tag_name() {
  local version="$1"
  echo "${TAG_PREFIX}${version}"
}
