use kalosm::language::*;

// Look at the docs for std::fs
// https://doc.rust-lang.org/std/fs/index.html
// std::fs provides functions that write to a file, read from a file,
// check if a file exists, etc.
use std::fs;

// LlamaChatSession provides helpful functions for loading and storing sessions.
// Look at https://docs.rs/kalosm/latest/kalosm/language/trait.ChatSession.html#saving-and-loading-sessions
// for some examples!

// Implement this
pub fn save_chat_session_to_file(filename: &str, session: &LlamaChatSession) {
    let bytes_session = session.to_bytes();
    match bytes_session {
        Ok(bytes) => {
            fs::write(filename, bytes).unwrap();
        }
        Err(e) => {
            println!("Error occurred while converting session to bytes: {}", e);
        }
    };
}

// Implement this
pub fn load_chat_session_from_file(filename: &str) -> Option<LlamaChatSession> {
    let load_file = fs::read(filename).unwrap();
    let mut session = LlamaChatSession::from_bytes(&load_file);
    match session {
        Ok(session) => {
            return Some(session);
        }
        Err(e) => {
            println!("Error occurred while loading session from file: {}", e);
            return None;
        }
    }
    // look at fs::read(...)
    // also look at LlamaChatSession::from_bytes(...)
}
