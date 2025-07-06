use rustyline::DefaultEditor;
use std::sync::Mutex;
use vulpes::{Handler, Session};

struct ReadlineHandler {
    editor: Mutex<DefaultEditor>,
}

impl ReadlineHandler {
    fn new() -> rustyline::Result<Self> {
        let editor = DefaultEditor::new()?;
        Ok(Self { editor: Mutex::new(editor) })
    }
}

#[async_trait::async_trait]
impl Handler for ReadlineHandler {
    async fn prompt(&self) -> Option<String> {
        loop {
            let result = {
                self.editor.lock().unwrap().readline("> ")
            };
            
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
}

#[tokio::main]
async fn main() -> rustyline::Result<()> {
    let handler = ReadlineHandler::new()?;
    
    let mut session = Session::new();
    session.handler(Box::new(handler));
    session.start().await;
    
    Ok(())
}
