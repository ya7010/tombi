pub type SyntaxNode = rowan::SyntaxNode<crate::TomlLang>;
#[allow(unused)]
pub type SyntaxToken = rowan::SyntaxToken<crate::TomlLang>;
#[allow(unused)]
pub type SyntaxElement = rowan::NodeOrToken<SyntaxNode, SyntaxToken>;
