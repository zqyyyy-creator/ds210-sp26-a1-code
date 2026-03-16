use kalosm::language::*;

#[allow(dead_code)]
pub struct ChatbotV2 {
    model: Llama,
    chat_session: Chat<Llama>,
    // What should you store inside your Chatbot type?
    // The model? The chat_session?
}

impl ChatbotV2 {
    #[allow(dead_code)]
    pub fn new(model: Llama) -> ChatbotV2 {
        let chat_session = model
            .chat()
            .with_system_prompt("You are a helpful and concise assistant. Answer the user's questions clearly and briefly.");
        return ChatbotV2 {
            model: model,
            chat_session: chat_session,
        };
    }

    #[allow(dead_code)]
    pub async fn chat_with_user(&mut self, message: String) -> String {
        let response = self.chat_session.add_message(message).await;
        match response {
            Ok(response) => {
                self.chat_session.add_message(response.clone()).await;
                return response;
            }
            Err(e) => return String::from("Error generating response"),
        }
    }
}
