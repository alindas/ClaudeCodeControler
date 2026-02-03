use crate::models::InstallationStatus;
use std::process::Command;

pub struct ClaudeInstaller;

impl ClaudeInstaller {
    pub fn check_installation() -> InstallationStatus {
        // 尝试多种方式检测 claude
        let checks = [
            ("claude", vec!["--version"]),
            ("npx", vec!["@anthropic-ai/claude-code", "--version"]),
        ];

        for (cmd, args) in checks {
            if let Ok(output) = Command::new(cmd).args(&args).output() {
                if output.status.success() {
                    let version = String::from_utf8_lossy(&output.stdout)
                        .trim()
                        .to_string();
                    let path = which::which(cmd)
                        .ok()
                        .map(|p| p.to_string_lossy().to_string());

                    return InstallationStatus {
                        installed: true,
                        version: Some(version),
                        path,
                    };
                }
            }
        }

        // 尝试全局 npm 包
        if let Ok(output) = Command::new("npm").args(["list", "-g", "@anthropic-ai/claude-code"]).output() {
            let output_str = String::from_utf8_lossy(&output.stdout);
            if output_str.contains("claude-code") {
                return InstallationStatus {
                    installed: true,
                    version: Some("global".to_string()),
                    path: None,
                };
            }
        }

        // Windows 专用：检查 npm 全局目录
        #[cfg(target_os = "windows")]
        {
            // 尝试使用 cmd /c 执行
            if let Ok(output) = Command::new("cmd")
                .args(["/c", "claude --version"])
                .output()
            {
                if output.status.success() {
                    let version = String::from_utf8_lossy(&output.stdout)
                        .trim()
                        .to_string();
                    return InstallationStatus {
                        installed: true,
                        version: Some(version),
                        path: Some("cmd /c claude".to_string()),
                    };
                }
            }

            // 检查常见 npm 全局安装路径
            let npm_paths = [
                "C:\\Program Files\\nodejs\\claude.cmd",
                "C:\\Users\\%USERNAME%\\AppData\\Roaming\\npm\\claude.cmd",
                "C:\\Programs\\nvm\\npm\\claude",
                "C:\\ProgramData\\nvm\\npm\\claude",
            ];

            for path in &npm_paths {
                let expanded = if path.contains("%USERNAME%") {
                    dirs::home_dir()
                        .map(|h| path.replace("%USERNAME%", h.file_name().unwrap().to_str().unwrap()))
                        .unwrap_or_else(|| path.to_string())
                } else {
                    path.to_string()
                };

                if std::path::Path::new(&expanded).exists() {
                    return InstallationStatus {
                        installed: true,
                        version: Some("detected".to_string()),
                        path: Some(expanded),
                    };
                }
            }
        }

        InstallationStatus {
            installed: false,
            version: None,
            path: None,
        }
    }

    pub fn install() -> Result<String, String> {
        // 使用 npm 全局安装
        let output = Command::new("npm")
            .args([
                "install",
                "-g",
                "@anthropic-ai/claude-code",
            ])
            .output()
            .map_err(|e| format!("Failed to run npm install: {}", e))?;

        if output.status.success() {
            Ok("Claude Code installed successfully".to_string())
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(format!("Installation failed: {}", stderr))
        }
    }

    pub fn get_install_command() -> String {
        "npm install -g @anthropic-ai/claude-code".to_string()
    }

    pub fn is_npm_available() -> bool {
        which::which("npm").is_ok()
    }

    pub fn install_npm_windows() -> Result<String, String> {
        // Windows 上尝试使用 PowerShell 安装 Node.js/npm
        let ps_command = r#"
            if (!(Get-Command node -ErrorAction SilentlyContinue)) {
                Write-Host "Installing Node.js..."
                Invoke-WebRequest -Uri "https://nodejs.org/dist/v20.10.0/node-v20.10.0-x64.msi" -OutFile "$env:TEMP\nodejs.msi"
                Start-Process -Wait -FilePath "msiexec.exe" -ArgumentList "/i", "$env:TEMP\nodejs.msi", "/quiet", "/norestart"
                Remove-Item "$env:TEMP\nodejs.msi"
                $env:Path = [System.Environment]::GetEnvironmentVariable("Path", "Machine") + ";" + [System.Environment]::GetEnvironmentVariable("Path", "User")
                Write-Host "Node.js installed successfully"
            } else {
                Write-Host "Node.js already installed"
            }
        "#;

        let output = Command::new("powershell")
            .args(["-Command", ps_command])
            .output()
            .map_err(|e| format!("Failed to run PowerShell: {}", e))?;

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            Err(String::from_utf8_lossy(&output.stderr).to_string())
        }
    }

    pub fn install_npm_unix() -> Result<String, String> {
        // macOS/Linux 使用 nvm 或直接安装
        let script = r#"
            if ! command -v node &> /dev/null; then
                if [[ "$OSTYPE" == "darwin"* ]]; then
                    if command -v brew &> /dev/null; then
                        brew install node
                    else
                        echo "Please install Homebrew first: https://brew.sh"
                        exit 1
                    fi
                else
                    # Linux
                    curl -fsSL https://deb.nodesource.com/setup_20.x | sudo -E bash -
                    sudo apt-get install -y nodejs
                fi
            fi
        "#;

        let output = Command::new("bash")
            .args(["-c", script])
            .output()
            .map_err(|e| format!("Failed to run bash: {}", e))?;

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            Err(String::from_utf8_lossy(&output.stderr).to_string())
        }
    }
}
