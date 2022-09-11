use crate::files::{pofile::{PoNote, PoComment}, fluent::FluentInformations};

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

pub(super) fn replace_placeholders(s: &String) -> String {
    let mut i = 1usize;
    let mut res = s.to_string();

    while let Some(pos) = res.find("{}") {
        res.replace_range(pos..(pos+2), &format!("{{ $arg{} }}", i));
        i += 1;
    }

    if i == 2 {
        res = res.replace("{ $arg1 }", "{ $arg }");
    }

    res
}
