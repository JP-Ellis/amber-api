# CI helper library
#
# This file is sourced by scripts under `scripts/ci/` and provides a small set
# of helpers to keep GitHub Actions workflows readable while still being easy to
# run locally.
#
# It is intentionally dependency-light: it relies on `scripts/util.sh` for shared
# helpers like `assert_cmd`, `assert_env`, `ensure`, and logging.
# shellcheck shell=bash

CI_SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
CI_REPO_ROOT="$(cd -- "${CI_SCRIPT_DIR}/../.." && pwd)"

# shellcheck source=scripts/util.sh
source "${CI_REPO_ROOT}/scripts/util.sh"

################################################################################
## GitHub Actions Helpers
################################################################################

# Append content to the GitHub Actions step summary
#
# In GitHub Actions, `GITHUB_STEP_SUMMARY` points to a file where Markdown can
# be written to appear in the job summary. Locally (when `GITHUB_STEP_SUMMARY`
# is not set), this function writes to stdout instead.
#
# Usage:
#   ci_append_summary <<EOF
#   ## Summary
#   Hello!
#   EOF
#
# Arguments:
#   None (reads from stdin)
#
# Returns:
#   0 - Always
#
# Example:
#   ci_append_summary <<EOF
#   - It worked
#   EOF
#
ci_append_summary() {
  if [ -n "${GITHUB_STEP_SUMMARY-}" ]; then
    cat >>"${GITHUB_STEP_SUMMARY}"
  else
    cat
  fi
}

# Append key/value lines to GitHub Actions step outputs
#
# In GitHub Actions, `GITHUB_OUTPUT` points to a file where `key=value` lines
# can be written to set step outputs. Locally (when `GITHUB_OUTPUT` is not set),
# this function does nothing.
#
# Usage:
#   ci_append_output "foo=bar"
#
# Arguments:
#   $@ - One or more `key=value` lines to append
#
# Returns:
#   0 - Always
#
# Example:
#   ci_append_output "version=1.2.3"
#
ci_append_output() {
  if [ -n "${GITHUB_OUTPUT-}" ]; then
    printf '%s\n' "$@" >>"${GITHUB_OUTPUT}"
  fi
}

################################################################################
## Git Identity Configuration
################################################################################

# Configure git user identity for commits and tags
#
# Automatically detects and configures git user.name and user.email using the
# authenticated GitHub account (via gh CLI). Handles both GitHub App tokens
# (bot accounts) and regular user tokens.
#
# If git identity is already configured, this function logs the existing
# identity and returns without making changes.
#
# For GitHub App tokens (bot accounts with [bot] suffix):
#   - Retrieves user ID from GitHub API
#   - Formats email as: {USER_ID}+{username}@users.noreply.github.com
#   - Example: 123456+my-app[bot]@users.noreply.github.com
#
# For regular user tokens:
#   - Attempts to retrieve user's public email from GitHub API
#   - Falls back to noreply format: {username}@users.noreply.github.com
#
# Requirements:
#   - gh CLI must be authenticated (gh auth status)
#   - gh CLI must have network access to GitHub API
#
# Usage:
#   configure_git_identity
#
# Returns:
#   0 - Identity configured successfully or already configured
#   Calls err() on failure (exits with 255)
#
# Example:
#   configure_git_identity
#   git commit -m "chore: automated commit"
#   git tag -a v1.0.0 -m "Release v1.0.0"
#
configure_git_identity() {
  info "Configuring git identity"

  # Check if git identity is already configured
  if git config user.name >/dev/null 2>&1 && git config user.email >/dev/null 2>&1; then
    local git_user_name git_user_email
    git_user_name=$(git config user.name)
    git_user_email=$(git config user.email)
    info "Using existing git identity: $git_user_name <$git_user_email>"
    return 0
  fi

  # Verify gh CLI is authenticated
  if ! gh auth status >/dev/null 2>&1; then
    err "Git identity not configured and gh CLI not authenticated"
  fi

  local git_user_name git_user_email

  # Try GitHub App endpoint first (for GitHub App tokens)
  local app_slug
  app_slug=$(gh api /app --jq '.slug' 2>/dev/null || echo "")

  if [ -n "$app_slug" ]; then
    # GitHub App token detected
    info "Detected GitHub App authentication"

    # Construct bot username
    local bot_username="${app_slug}[bot]"

    # Get the bot's user ID
    local user_id
    user_id=$(gh api "/users/$bot_username" --jq '.id' 2>/dev/null || echo "")

    if [ -z "$user_id" ]; then
      err "Failed to retrieve user ID for GitHub App bot: $bot_username"
    fi

    # Format: {USER_ID}+{username}@users.noreply.github.com
    git_user_name="$bot_username"
    git_user_email="${user_id}+${bot_username}@users.noreply.github.com"

    info "Authenticated as: $git_user_name"
  else
    # Not a GitHub App token - try regular user endpoint (for PATs)
    local auth_user
    auth_user=$(gh api /user --jq '.login' 2>/dev/null || echo "")

    if [ -z "$auth_user" ]; then
      err "Failed to retrieve authenticated user from GitHub"
    fi

    info "Authenticated as: $auth_user"

    local user_email
    user_email=$(gh api /user --jq '.email' 2>/dev/null || echo "")

    if [ -n "$user_email" ] && [ "$user_email" != "null" ]; then
      git_user_name="$auth_user"
      git_user_email="$user_email"
    else
      # Fallback to noreply email
      git_user_name="$auth_user"
      git_user_email="${auth_user}@users.noreply.github.com"
    fi
  fi

  # Configure git
  git config user.name "$git_user_name"
  git config user.email "$git_user_email"

  info "Configured git identity: $git_user_name <$git_user_email>"
}

# Configure git to use a GitHub token for authentication
#
# Updates git remote URL to use token-based authentication for push operations.
# This is necessary when running in CI where the repository was checked out with
# default credentials that may not have sufficient permissions.
#
# Requirements:
#   - GH_TOKEN or GITHUB_TOKEN environment variable must be set
#   - git remote 'origin' must exist
#
# Usage:
#   configure_git_auth
#
# Returns:
#   0 - Authentication configured successfully
#   Calls err() on failure (exits with 255)
#
configure_git_auth() {
  # Check if we have a GitHub token
  local token="${GH_TOKEN:-${GITHUB_TOKEN:-}}"

  if [ -z "$token" ]; then
    err "Cannot configure git authentication: GH_TOKEN or GITHUB_TOKEN not set"
  fi

  # Get the remote URL
  local remote_url
  remote_url=$(git remote get-url origin)

  # Extract the repository path (owner/repo) from the URL
  local repo_path
  if [[ $remote_url =~ github\.com[:/](.+)(\.git)?$ ]]; then
    repo_path="${BASH_REMATCH[1]}"
    repo_path="${repo_path%.git}" # Remove .git suffix if present
  else
    err "Could not parse repository path from remote URL: $remote_url"
  fi

  # Configure git to use token authentication
  # Format: https://x-access-token:TOKEN@github.com/owner/repo.git
  local auth_url="https://x-access-token:${token}@github.com/${repo_path}.git"

  git remote set-url origin "$auth_url"

  info "Configured git authentication for push operations"
}
