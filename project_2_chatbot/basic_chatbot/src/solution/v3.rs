use kalosm::language::*;

#[allow(dead_code)]
pub struct ChatbotV3 {
    model: Llama,
    chat_sessions: std::collections::HashMap<String, Chat<Llama>>,
    // What should you store inside your Chatbot type?
    // The model? The chat_session?
    // Storing a single chat session is not enough: it mixes messages from different users
    // together!
    // Need to store one chat session per user.
    // Think of some kind of data structure that can help you with this.
}

impl ChatbotV3 {
    #[allow(dead_code)]
    pub fn new(model: Llama) -> ChatbotV3 {
        return ChatbotV3 {
            model: model,
            chat_sessions: std::collections::HashMap::new(),
            // Make sure you initialize your struct members here
        };
    }

    #[allow(dead_code)]
    pub async fn chat_with_user(&mut self, username: String, message: String) -> String {

        if !self.chat_sessions.contains_key(&username) {
            let chat_session = self.model
            .chat()
            .with_system_prompt("The assistant will act like a pirate");
            
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
        // Add your code for chatting with the agent while keeping conversation history here.
        // Notice, you are given both the `message` and also the `username`.
        // Use this information to select the correct chat session for that user and keep it
        // separated from the sessions of other users.
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
        // Extract the chat message history for the given username
        // Hint: think of how you can retrieve the Chat object for that user, when you retrieve it
        // you may want to use https://docs.rs/kalosm/0.4.0/kalosm/language/struct.Chat.html#method.session
        // to then retrieve the history!
        return Vec::new();
    }
}