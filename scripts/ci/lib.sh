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

# Authenticate git with GitHub using a token
#
# GitHub Actions provide a `GITHUB_TOKEN` secret by default, but if a custom
# token is provided via the `GH_TOKEN` environment variable (e.g., to
# authenticate an app or user), this function configures git to use it instead.
#
# Usage:
#   configure_git_auth
#
# Environment Variables:
#   GH_TOKEN - (optional) GitHub token to use for git authentication. If not
#     set, this function does nothing.
#
# Returns:
#   0 - Always
#
configure_git_auth() {
  if [ -n "${GH_TOKEN-}" ]; then
    git remote set-url origin "https://x-access-token:${GH_TOKEN}@github.com/$(git remote get-url origin | sed -E 's#https://github.com/##')"
    info "Configured git authentication using GH_TOKEN"
  else
    info "GH_TOKEN not set; skipping git authentication configuration"
  fi
}
