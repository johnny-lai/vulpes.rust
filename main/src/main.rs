use clap::Parser;
use rustyline::DefaultEditor;
use std::sync::Mutex;
use vulpes::{Handler, Session, ToolCall, agent::Ollama};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "http://localhost:11434")]
    server: String,

    #[arg(short, long, default_value = "devstral")]
    model: String,
}

struct ReadlineHandler {
    editor: Mutex<DefaultEditor>,
}

impl ReadlineHandler {
    fn new() -> rustyline::Result<Self> {
        let editor = DefaultEditor::new()?;
        Ok(Self {
            editor: Mutex::new(editor),
        })
    }
}

#[async_trait::async_trait]
impl Handler for ReadlineHandler {
    async fn prompt(&self) -> Option<String> {
        loop {
            let result = { self.editor.lock().unwrap().readline("> ") };

            match result {
                Ok(line) => {
                    let trimmed = line.trim();
                    if trimmed == "quit" || trimmed == "exit" {
                        return None;
                    } else if trimmed.is_empty() {
                        continue; // Try again for empty input
                    } else {
                        return Some(trimmed.to_string());
                    }
                }
                Err(_) => return None,
            }
        }
    }

    async fn response(&self, content: &str) {
        if !content.is_empty() {
            println!("Assistant: {}", content);
        }
    }

    async fn allow_tool(&self, tool_call: &ToolCall) -> bool {
        println!("{:?}", tool_call);
        true
    }
}

#[tokio::main]
async fn main() -> rustyline::Result<()> {
    let args = Args::parse();

    let agent = Ollama::new(&args.server, &args.model);

    let handler = ReadlineHandler::new()?;

    let mut session = Session::new();
    session.handler(Box::new(handler));
    session.agent(Box::new(agent));
    session.start().await;

    Ok(())
}
