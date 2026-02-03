#!/bin/bash
# Claude Code Wrapper with Hook Support
# 使用方法: ./claude-wrapper.sh [claude args]

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
HOOK_SCRIPT="$SCRIPT_DIR/hook.sh"

# Check if hook script exists
if [ ! -f "$HOOK_SCRIPT" ]; then
    echo "Warning: Hook script not found at $HOOK_SCRIPT" >&2
    echo "Running claude without hooks..." >&2
    claude "$@"
    exit $?
fi

# Export hook environment variable
export CLAUDE_CODE_HOOKS="$HOOK_SCRIPT"

# Run claude with all arguments
claude "$@"
