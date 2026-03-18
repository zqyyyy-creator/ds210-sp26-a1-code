use kalosm::language::*;
use std::collections::HashMap;

#[allow(dead_code)]
pub struct ChatbotV3 {
    model: Llama,
    chat_sessions: HashMap<String, Chat<Llama>>,
}

impl ChatbotV3 {
    #[allow(dead_code)]
    pub fn new(model: Llama) -> ChatbotV3 {
        return ChatbotV3 {
            model: model,
            chat_sessions: HashMap::new(),
            // Make sure you initialize your struct members here
        };
    }

    #[allow(dead_code)]
    pub async fn chat_with_user(&mut self, username: String, message: String) -> String {

        if !self.chat_sessions.contains_key(&username) {
            let chat_session = self.model
            .chat()
            .with_system_prompt("You are a helpful and concise assistant. Answer the user's questions clearly and briefly.");
            
            self.chat_sessions.insert(username.clone(), chat_session);
        }

        let session = self.chat_sessions.get_mut(&username).unwrap();
        let asynchronous_output = session.add_message(message);
        let output= asynchronous_output.await;

        match output{
            Ok(response) => {
                return response;
            },
            Err(e) => {
                println!("Something went wrong");
                return "Error".to_string();
            }
        }
    }

    #[allow(dead_code)]
    pub fn get_history(&self, username: String) -> Vec<String> {
        let mut history_as_strings = Vec::new();

        if self.chat_sessions.contains_key(&username) {
            let session = self.chat_sessions.get(&username).unwrap();
            let history = session.session().unwrap().history();

            for message in history{
            let mut text = message.content().to_string();
            history_as_strings.push(text);
            }

            return history_as_strings;
        }
        return Vec::new();
        }
    }
