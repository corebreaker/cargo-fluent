pub(super) enum EntryType {
    Junk,
    Group,
    ResourceHeader,
    Message(String),
    Comment(Vec<String>),
}
