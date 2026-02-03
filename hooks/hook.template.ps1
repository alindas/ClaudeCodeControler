# Claude Code Monitor Hook Template (PowerShell)
# 此文件会被复制到 ~/.claude-monitor/hook.ps1

param(
    [string]$Event,
    [string]$Data
)

$HookUrl = "http://localhost:{{PORT}}/hook"
$Pid = $PID
$Cwd = Get-Location | Select-Object -ExpandProperty Path

function Send-Hook {
    param(
        [string]$EventType,
        [object]$Payload
    )

    $body = @{
        event = $EventType
        pid = $Pid
        cwd = $Cwd
        timestamp = [int](Get-Date -UFormat %s)
        data = $Payload
    } | ConvertTo-Json -Compress

    try {
        Invoke-RestMethod -Uri $HookUrl -Method Post -Body $body -ContentType "application/json" -TimeoutSec 2 | Out-Null
    } catch {
        # Silently fail
    }
}

switch ($Event) {
    "start" { Send-Hook -EventType "task_start" }
    "end" { Send-Hook -EventType "task_end" }
    "prompt" { Send-Hook -EventType "prompt" -Payload @{ content = $Data } }
    "response" { Send-Hook -EventType "response" -Payload @{ content = $Data } }
    "error" { Send-Hook -EventType "error" -Payload @{ message = $Data } }
    default { Send-Hook -EventType $Event }
}
