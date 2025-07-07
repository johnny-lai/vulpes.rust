use crate::tool::Tool;
use anyhow::Result;
use serde_json::{Value, json};
use std::fs;
use std::process::Command;

pub struct ShellExecute {}

impl ShellExecute {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait::async_trait]
impl Tool for ShellExecute {
    fn definition(&self) -> Value {
        // TODO: Description should specify the correct shell
        json!({
            "type": "function",
            "function": {
                "name": "shell_execute",
                "title": null,
                "description": "Execute commands using the bash shell",
                "input_schema": {
                    "type": "object",
                    "properties": {
                        "command": {
                            "type": "string",
                            "description": "The shell command to execute"
                        }
                    },
                    "required": ["command"]
                }
            }
        })
    }

    async fn call(&self, arguments: Value) -> Result<String> {
        let command = arguments["command"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Missing command argument"))?
            .to_string();
        self.execute_command(command).await
    }
}

impl ShellExecute {
    /// Execute a shell command in the current directory
    async fn execute_command(&self, command: String) -> Result<String> {
        println!("🔧 Executing command: {}", command);

        let output = if cfg!(target_os = "windows") {
            Command::new("cmd").args(["/C", &command]).output()
        } else {
            Command::new("sh").args(["-c", &command]).output()
        };

        match output {
            Ok(output) => {
                let stdout = String::from_utf8_lossy(&output.stdout);
                let stderr = String::from_utf8_lossy(&output.stderr);

                if output.status.success() {
                    let result = format!("Command output:\n{}", stdout);
                    println!("✅ Command succeeded");
                    Ok(result)
                } else {
                    let error_msg = format!(
                        "Command failed with exit code {:?}\nstdout: {}\nstderr: {}",
                        output.status.code(),
                        stdout,
                        stderr
                    );
                    println!("❌ Command failed: {}", error_msg);
                    Err(anyhow::anyhow!(error_msg))
                }
            }
            Err(e) => {
                let error_msg = format!("Failed to execute command: {}", e);
                println!("❌ Execution error: {}", error_msg);
                Err(anyhow::anyhow!(error_msg))
            }
        }
    }
}

pub struct ReadFile {}

impl ReadFile {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait::async_trait]
impl Tool for ReadFile {
    fn definition(&self) -> Value {
        json!({
            "type": "function",
            "function": {
                "name": "read_file",
                "description": "Read the contents of a file",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "file_path": {
                            "type": "string",
                            "description": "Path to the file to read"
                        }
                    },
                    "required": ["file_path"]
                }
            }
        })
    }

    async fn call(&self, arguments: Value) -> Result<String> {
        let file_path = arguments["file_path"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Missing command argument"))?
            .to_string();
        println!("📁 Reading file: {}", file_path);
        match fs::read_to_string(&file_path) {
            Ok(contents) => {
                let result = format!("File contents of {}:\n{}", file_path, contents);
                println!("✅ File read successfully");
                Ok(result)
            }
            Err(e) => {
                let error_msg = format!("Error reading file '{}': {}", file_path, e);
                println!("❌ File read failed: {}", error_msg);
                Err(anyhow::anyhow!(error_msg))
            }
        }
    }
}
