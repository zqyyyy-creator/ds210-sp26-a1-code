use kalosm::language::*;
use file_chatbot::solution::file_library;

use crate::solution::Cache;

pub struct ChatbotV5 {
    model: Llama,
    cache: Cache<Chat<Llama>>,
}

impl ChatbotV5 {
    pub fn new(model: Llama) -> ChatbotV5 {
        return ChatbotV5 {
            model: model,
            cache: Cache::new(3),
        };
    }

    pub async fn chat_with_user(&mut self, username: String, message: String) -> String {
        let filename = &format!("{}.txt", username);
        let cached_chat = self.cache.get_chat(&username).cloned();

        match cached_chat {
            None => {
                println!("chat_with_user: {username} is not in the cache!");
                // The cache does not have the chat. What should you do?
                let mut chat_session = self.model.chat();
                if let Some(past_session) = file_library::load_chat_session_from_file(&filename){
                    chat_session = chat_session.with_session(past_session);
                }
                
                let output = chat_session.add_message(message).await;

                match output {
                    Ok(response) => {
                        self.cache.insert_chat(username.clone(), chat_session.clone());
                        if let Ok(updated_session) = chat_session.session() {
                            file_library::save_chat_session_to_file(&filename, &updated_session);
                        }
                        response
                    }
                    Err(e) => {
                        println!("Something went wrong");
                        String::from("Error")
                    }
                }  
            }
            Some(mut chat_session) => {
                println!("chat_with_user: {username} is in the cache! Nice!");
                let output = chat_session.add_message(message).await;

                match output {
                    Ok(response) => {
                        self.cache.insert_chat(username.clone(), chat_session.clone());
                        if let Ok(updated_session) = chat_session.session() {
                            file_library::save_chat_session_to_file(&filename, &updated_session);
                        }
                        response
                    }
                    Err(e) => {
                        println!("Something went wrong");
                        return String::from("Error");
                    }
                }
            }
        }
    }

    pub fn get_history(&mut self, username: String) -> Vec<String> {
        let filename = &format!("{}.txt", username);
        let cached_chat = self.cache.get_chat(&username);

        match cached_chat {
            None => {
                println!("get_history: {username} is not in the cache!");
                let mut chat_session = self.model.chat();
                let history_chat_session = file_library::load_chat_session_from_file(&filename);
                match history_chat_session {
                    None => {
                    },
                Some(session) => {
                    chat_session = chat_session.with_session(session)
                    }
                }
                self.cache.insert_chat(username.clone(), chat_session);
                let chat_session = self.cache.get_chat(&username).unwrap();
                let history = chat_session.session().unwrap().history();
                let mut history_in_strings = Vec::new();
                for message in history {
                    history_in_strings.push(message.content().to_string());
                }
                return history_in_strings;
            }
            Some(chat_session) => {
                println!("get_history: {username} is in the cache! Nice!");
                let mut history_as_strings = Vec::new();
                let history = chat_session.session().unwrap().history();
                for message in history{
                    let mut text = message.content().to_string();
                    history_as_strings.push(text);
                }

                return history_as_strings;

            }
        }
    }
}