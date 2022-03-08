use super::{Message, Origin};
use proc_macro2::Span;

pub(super) struct Messages(Vec<Message>);

impl Messages {
    pub(super) fn new() -> Self {
        Messages(vec![])
    }

    pub(super) fn new_translation(&mut self, origin: Origin, msg_id: String, span: Span) -> &mut Message {
        let idx = self.0.len();
        let loc = span.start();

        self.0.push(Message::new(origin, msg_id, loc.line, loc.column));
        &mut self.0[idx]
    }

    pub(super) fn take(self) -> Vec<Message> {
        self.0
    }
}