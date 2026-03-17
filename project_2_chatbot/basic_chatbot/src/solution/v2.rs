use kalosm::language::*;

#[allow(dead_code)]
pub struct ChatbotV2 {
    chat_session: Chat<Llama>,
    // What should you store inside your Chatbot type?
    // The model? The chat_session?
}

impl ChatbotV2 {
    #[allow(dead_code)]
    pub fn new(model: Llama) -> ChatbotV2 {
        let mut chat_session = model
            .chat()
            .with_system_prompt("The assistant will act like a pirate");
        return ChatbotV2 {
            chat_session: chat_session,
            // Whatever you decide to store in the struct
            // you need to make sure you pass here!
        };
    }

    #[allow(dead_code)]
    pub async fn chat_with_user(&mut self, message: String) -> String {
        let asynchronous_output = self.chat_session.add_message(message);
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
    }
}