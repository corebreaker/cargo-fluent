use poreader::comment::Comment;

#[derive(Clone, Eq, PartialEq, Debug)]
pub(crate) struct PoComment {
    comment: Comment
}

impl PoComment {
    pub(super) fn new(comment: Comment) -> Self {
        PoComment { comment }
    }

    pub(crate) fn kind(&self) -> char {
        self.comment.kind()
    }

    pub(crate) fn comment(&self) -> &String {
        self.comment.comment()
    }
}
