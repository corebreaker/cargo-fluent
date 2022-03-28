use crate::files::{pofile::{PoNote, PoComment}, fluent::FluentInformations};

#[inline]
pub(super) fn add_comments_and_notes(comments: &Vec<PoComment>, notes: &Vec<PoNote>, into: &mut FluentInformations) {
    for comment in comments {
        into.add_comment(comment.comment().to_string());
    }

    for note in notes {
        if note.origin_developper() {
            into.add_comment(format!("Developper> {}", note.value()));
        } else if note.origin_translator() {
            into.add_comment(format!("Translator> {}", note.value()));
        }
    }
}