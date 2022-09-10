use crate::files::{pofile::{PoNote, PoComment}, fluent::FluentInformations};

#[inline]
pub(super) fn add_comments_and_notes(comments: &Vec<PoComment>, notes: &Vec<PoNote>, into: &mut FluentInformations) {
    {
        let mut infos = FluentInformations::new();

        for comment in comments {
            infos.add_comment(comment.comment().to_string());
        }

        if !infos.comments().is_empty() && !into.contains_comments(&infos) {
            into.add_comments(infos);
        }
    }

    {
        let mut infos = FluentInformations::new();

        for note in notes {
            if note.origin_developper() {
                infos.add_comment(format!("Developper> {}", note.value()));
            } else if note.origin_translator() {
                infos.add_comment(format!("Translator> {}", note.value()));
            }
        }

        if !infos.comments().is_empty() && !into.contains_comments(&infos) {
            into.add_comments(infos);
        }
    }
}