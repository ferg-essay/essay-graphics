pub struct Shell<'a, Message> {
    messages: &'a mut Vec<Message>,
}

impl<'a, Message> Shell<'a, Message> {
    pub fn new(messages: &'a mut Vec<Message>) -> Self {
        Self {
            messages,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.messages.is_empty()
    }

    pub fn publish(&mut self, message: Message) {
        self.messages.push(message);
    }
}