# Claude Code Wrapper with Hook Support (PowerShell)
# 使用方法: .\claude-wrapper.ps1 [claude args]

$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$HookScript = Join-Path $ScriptDir "hook.ps1"

# Check if hook script exists
if (-not (Test-Path $HookScript)) {
    Write-Warning "Hook script not found at $HookScript"
    Write-Host "Running claude without hooks..."
    & claude @args
    exit $LASTEXITCODE
}

# Set hook environment variable
$env:CLAUDE_CODE_HOOKS = $HookScript

# Run claude with all arguments
& claude @args
