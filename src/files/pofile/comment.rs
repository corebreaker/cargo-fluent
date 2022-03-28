use poreader::comment::Comment;

#[derive(Clone, Eq, PartialEq, Debug)]
pub(crate) struct PoComment {
    comment: Comment
}

impl PoComment {
    pub(super) fn new(comment: Comment) -> Self {
        PoComment { comment }
    }

    pub(crate) fn comment(&self) -> &str {
        self.comment.comment()
    }
}
