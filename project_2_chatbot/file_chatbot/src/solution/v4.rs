use kalosm::language::*;
use crate::solution::file_library;

pub struct ChatbotV4 {
    model: Llama,
}

impl ChatbotV4 {
    pub fn new(model: Llama) -> ChatbotV4 {
        return ChatbotV4 {
            model: model,
        };
    }

    pub async fn chat_with_user(&mut self, username: String, message: String) -> String {
        let filename = &format!("{}.txt", username);

        let mut chat_session: Chat<Llama> = self.model.chat();


        // TODO: You have to implement the rest:
        // You need to load the chat session from the file using file_library::load_chat_session_from_file(...).
        // Think about what needs to happen if the function returns None vs Some(session).
        // Hint: look at https://docs.rs/kalosm/latest/kalosm/language/struct.Chat.html#method.with_session
        let history_chat_session = file_library::load_chat_session_from_file(&filename);

        if let Some(session) = history_chat_session {
            chat_session = chat_session.with_session(session);
        }

        //match history_chat_session {
         //   None => {
          //  },
          //  Some(session) => {
          ///      chat_session = chat_session.with_session(session);
          //  }
         //  }
        //let asynchronous_output = chat_session.add_message(message);
        //let output= asynchronous_output.await;
        //let session = chat_session.session().unwrap();

        let output = chat_session.add_message(message).await;

        match output{
            Ok(response) => {
                if let Ok(updated_session) = chat_session.session() {
                    file_library::save_chat_session_to_file(&filename, &updated_session);
                }
                //file_library::save_chat_session_to_file(&filename, &session);
                return response;
            }
            Err(e) => {
                println!("Something went wrong");
                return "Error".to_string();
            }
        }
}

    pub fn get_history(&self, username: String) -> Vec<String> {
        let filename = &format!("{}.txt", username);

        match file_library::load_chat_session_from_file(&filename) {
            None => {
                return Vec::new();
            },
            Some(session) => {
                // TODO: what should happen here?
                let history = session.history();

                println!("History requested for {}: found {} messages", username, history.len());

                let mut history_in_strings = Vec::new();
                for message in history {
                    history_in_strings.push(message.content().to_string());
                }
                return history_in_strings;
            }
        }
    }
}