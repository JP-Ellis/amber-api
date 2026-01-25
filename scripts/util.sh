# shellcheck shell=bash

################################################################################
## Logging Functions
################################################################################

# Print a message to stderr
#
# This function prints a formatted message with a colored prefix if the
# terminal supports it.
#
# This function is not intended to be called directly, and is instead
# used by the other logging functions.
#
# Usage:
#   _log <prefix> <colour> <message>
#
# Arguments:
#   $1 - The prefix text to display in brackets
#   $2 - The tput color index for the prefix
#   $3 - The message to display
#
# Example:
#   _log "INFO" 2 "This is an information message"
#
_log() {
  local _prefix="$1"
  local _message="$3"
  local _color
  local _reset
  if check_cmd tput && _color=$(tput setaf "$2" 2>/dev/null) && _reset=$(tput sgr0 2>/dev/null); then
    :
  else
    _color=""
    _reset=""
  fi

  printf '%s %s\n' "[${_color}$(date +%T)|${_prefix}${_reset}]" "$_message" >&2
}

# Print debug message
#
# Only prints if VERBOSE is set.
#
# Usage:
#   debug <message>
#
# Arguments:
#   $1 - The debug message to display
#
# Example:
#   debug "This is a debug message"
#
debug() {
  if [ -n "${VERBOSE-}" ]; then
    _log Debug 4 "$1"
  fi
}

# Print info message
#
# Can be silenced with QUIET.
#
# Usage:
#   info <message>
#
# Arguments:
#   $1 - The info message to display
#
# Example:
#   info "This is an info message"
#
info() {
  if [ -z "${QUIET-}" ]; then
    _log Info 2 "$1"
  fi
}

# Alias for info
log() {
  info "$@"
}

# Print warning message
#
# Cannot be silenced.
#
# Usage:
#   warn <message>
#
# Arguments:
#   $1 - The warning message to display
#
# Example:
#   warn "This is a warning"
#
warn() {
  _log Warning 3 "$1"
}

# Print error message and exit
#
# Cannot be silenced. Exits with status code 255.
#
# Usage:
#   err <message>
#
# Arguments:
#   $1 - The error message to display
#
# Example:
#   err "Required command not found"
#
err() {
  _log Error 1 "$1"
  exit 255
}

################################################################################
## Command Utilities
################################################################################

# Check if a command exists
#
# Usage:
#   check_cmd <command>
#
# Arguments:
#   $1 - The command to check for
#
# Returns:
#   0 - If the command exists
#   non-zero - If the command does not exist.
#
# Example:
#   if ! check_cmd yq; then
#     curl -sSL ...
#   fi
#
check_cmd() {
  command -v "$1" >/dev/null 2>&1
}

# Assert that a command exists
#
# Exits with an error if the command is not found.
#
# Usage:
#   assert_cmd <command>
#
# Arguments:
#   $1 - The command to check for
#
# Example:
#   assert_cmd yq
#
assert_cmd() {
  if ! check_cmd "$1"; then
    err "need '$1' (command not found)"
  fi
}

################################################################################
## Environment Utilities
################################################################################

# Check if an environment variable is set
#
# Usage:
#   check_env <var_name>
#
# Arguments:
#   $1 - The environment variable name to check for
#
# Returns:
#   0 - If the environment variable is set and non-empty
#   non-zero - If the environment variable is not set (or is empty)
#
# Example:
#   if check_env AWS_PROFILE; then
#     ...
#   fi
#
check_env() {
  local _name="$1"
  [ -n "${!_name-}" ]
}

# Assert that an environment variable is set
#
# Exits with an error if the environment variable is not set.
#
# Usage:
#   assert_env <var_name>
#
# Arguments:
#   $1 - The environment variable name to check
#
# Example:
#   assert_env GITHUB_TOKEN
#
assert_env() {
  local _name="$1"
  if ! check_env "${_name}"; then
    err "need env var '${_name}'"
  fi
}

# Run a command that should never fail
#
# If the command fails, execution will immediately terminate with an error
# showing the failing command. It is recommended that the `errexit` option is
# enabled, which technically makes this redundant; but this can still be good to
# ensure consistency irrespective of the `errexit` setting. It additionally will
# make sure to log the failing command (which `errexit` does not do).
#
# Usage:
#   ensure <command> [args...]
#
# Arguments:
#   $@ - The command and arguments to execute
#
# Example:
#   ensure aws s3 cp local s3://<bucket>/<key>
#
ensure() {
  if ! "$@"; then
    err "command failed: $*"
  fi
}

# Run a command that is allowed to fail
#
# If the command fails, execution will continue without terminating. A debug
# message will be displayed, but the script will continue running.
#
# Usage:
#   ignore <command> [args...]
#
# Arguments:
#   $@ - The command and arguments to execute
#
# Example:
#   ignore aws s3 cp local s3://<bucket>/<key>
#
ignore() {
  if ! "$@"; then
    debug "command failed (allowed to fail): $*"
  fi
}
