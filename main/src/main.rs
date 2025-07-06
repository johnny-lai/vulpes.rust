use rustyline::DefaultEditor;
use std::cell::RefCell;
use vulpes::{Handler, Session};

struct ReadlineHandler {
    editor: RefCell<DefaultEditor>,
}

impl ReadlineHandler {
    fn new() -> rustyline::Result<Self> {
        let editor = DefaultEditor::new()?;
        Ok(Self { editor: RefCell::new(editor) })
    }
}

impl Handler for ReadlineHandler {
    fn prompt(&self) -> Option<String> {
        match self.editor.borrow_mut().readline("> ") {
            Ok(line) => {
                let trimmed = line.trim();
                if trimmed == "quit" || trimmed == "exit" {
                    None
                } else if trimmed.is_empty() {
                    self.prompt() // Try again for empty input
                } else {
                    Some(trimmed.to_string())
                }
            }
            Err(_) => None,
        }
    }
}

#[tokio::main]
async fn main() -> rustyline::Result<()> {
    let handler = ReadlineHandler::new()?;
    
    let mut session = Session::new();
    session.handler(Box::new(handler));
    session.start();
    
    Ok(())
}
