#!/bin/bash
# Claude Code Monitor Hook Template
# 此文件会被复制到 ~/.claude-monitor/hook.sh

HOOK_URL="http://localhost:{{PORT}}/hook"
PID=$$
CWD="$(pwd)"

send_hook() {
    local event_type="$1"
    local data="${2:-null}"

    curl -s -X POST "$HOOK_URL" \
        -H "Content-Type: application/json" \
        -d "{
            \"event\": \"$event_type\",
            \"pid\": $PID,
            \"cwd\": \"$CWD\",
            \"timestamp\": $(date +%s),
            \"data\": $data
        }" > /dev/null 2>&1 || true
}

# Hook into various events
case "$1" in
    start)
        send_hook "task_start"
        ;;
    end)
        send_hook "task_end"
        ;;
    prompt)
        # Escape JSON special characters
        local content="$2"
        content="${content//\\/\\\\}"
        content="${content//\"/\\"}"
        send_hook "prompt" "{\"content\": \"$content\"}"
        ;;
    response)
        local content="$2"
        content="${content//\\/\\\\}"
        content="${content//\"/\\"}"
        send_hook "response" "{\"content\": \"$content\"}"
        ;;
    error)
        send_hook "error" "{\"message\": \"$2\"}"
        ;;
    *)
        send_hook "$1"
        ;;
esac
