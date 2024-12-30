#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DanglingComment(pub crate::Comment);

impl AsRef<crate::Comment> for DanglingComment {
    fn as_ref(&self) -> &crate::Comment {
        &self.0
    }
}

impl From<crate::Comment> for DanglingComment {
    fn from(comment: crate::Comment) -> Self {
        DanglingComment(comment)
    }
}

impl From<DanglingComment> for crate::Comment {
    fn from(comment: DanglingComment) -> Self {
        comment.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BeginDanglingComment(crate::Comment);

impl AsRef<crate::Comment> for BeginDanglingComment {
    fn as_ref(&self) -> &crate::Comment {
        &self.0
    }
}

impl From<crate::Comment> for BeginDanglingComment {
    fn from(comment: crate::Comment) -> Self {
        BeginDanglingComment(comment)
    }
}

impl From<BeginDanglingComment> for crate::Comment {
    fn from(comment: BeginDanglingComment) -> Self {
        comment.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EndDanglingComment(crate::Comment);

impl AsRef<crate::Comment> for EndDanglingComment {
    fn as_ref(&self) -> &crate::Comment {
        &self.0
    }
}

impl From<crate::Comment> for EndDanglingComment {
    fn from(comment: crate::Comment) -> Self {
        EndDanglingComment(comment)
    }
}

impl From<EndDanglingComment> for crate::Comment {
    fn from(comment: EndDanglingComment) -> Self {
        comment.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LeadingComment(crate::Comment);

impl AsRef<crate::Comment> for LeadingComment {
    fn as_ref(&self) -> &crate::Comment {
        &self.0
    }
}

impl From<crate::Comment> for LeadingComment {
    fn from(comment: crate::Comment) -> Self {
        LeadingComment(comment)
    }
}

impl From<LeadingComment> for crate::Comment {
    fn from(comment: LeadingComment) -> Self {
        comment.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TailingComment(crate::Comment);

impl AsRef<crate::Comment> for TailingComment {
    fn as_ref(&self) -> &crate::Comment {
        &self.0
    }
}

impl From<crate::Comment> for TailingComment {
    fn from(comment: crate::Comment) -> Self {
        TailingComment(comment)
    }
}

impl From<TailingComment> for crate::Comment {
    fn from(comment: TailingComment) -> Self {
        comment.0
    }
}
